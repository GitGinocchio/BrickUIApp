pub mod overlay;
pub mod wallpaper;

use std::{ptr::null_mut, sync::{Arc, Mutex}};
use serde::{Deserialize, Serialize};
use windows::{core::BOOL, Win32::{
    Foundation::{HWND, LPARAM, POINT, RECT, WPARAM},
    Graphics::Gdi::{GetMonitorInfoA, GetMonitorInfoW, MonitorFromPoint, MonitorFromWindow, MONITORINFO, MONITORINFOEXW, MONITOR_DEFAULTTONEAREST},
    UI::WindowsAndMessaging::{
        EnumWindows, FindWindowA, FindWindowExA, GetWindowLongPtrW, GetWindowRect, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible, IsZoomed, SendMessageTimeoutA, SetParent, SetWindowLongPtrW, SetWindowPos, GWL_STYLE, HWND_TOPMOST, SMTO_NORMAL, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOZORDER, WS_CAPTION, WS_THICKFRAME
    },
}};
use windows::core::PCSTR;

use crate::winapi::{monitor::Monitor, Rect};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    pub hwnd: isize,
    pub title: String,
    pub is_visible: bool,
    pub is_maximized: bool,
    pub rect: Option<Rect>,
    pub monitor: Option<Monitor>
}

impl Window {
    pub fn set_rect(&self, rect: &Rect) -> Result<(), String> {
        if self.hwnd == 0 {
            return Err(windows::core::Error::from_win32().message());
        }

        unsafe {
            let hwnd = HWND(self.hwnd as *mut _);

            // Usa direttamente i campi left/top/right/bottom del tuo Rect
            SetWindowPos(
                hwnd,
                None,
                rect.left,
                rect.top,
                rect.right - rect.left,
                rect.bottom - rect.top,
                SWP_NOZORDER | SWP_NOACTIVATE,
            ).map_err(|e| format!("Error setting window position: {e}"))?;
        }

        Ok(())
    }
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let windows_vec: &Mutex<Vec<Window>> = unsafe { &*(lparam.0 as *const Mutex<Vec<Window>>) };

    let is_visible = unsafe { IsWindowVisible(hwnd).as_bool() };
    let is_maximized = unsafe { IsZoomed(hwnd).as_bool() };

    // Titolo finestra
    let length = unsafe { GetWindowTextLengthW(hwnd) };
    let mut title = String::new();
    if length > 0 {
        let mut buffer = vec![0u16; (length + 1) as usize];
        let read_len = unsafe { GetWindowTextW(hwnd, &mut buffer) };
        title = String::from_utf16_lossy(&buffer[..read_len as usize]);
    }

    // Rettangolo finestra
    let mut rect: RECT = RECT::default();
    let rect_opt = if unsafe { GetWindowRect(hwnd, &mut rect).is_ok() } {
        Some(rect.into())
    } else {
        None
    };

    // Monitor associato
    let hmon = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST) };
    let mut monitor_info = MONITORINFOEXW::default();
    monitor_info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;

    let monitor = if unsafe { GetMonitorInfoW(hmon, &mut monitor_info as *mut _ as *mut _).as_bool() } {
        let device_name = String::from_utf16_lossy(
            &monitor_info.szDevice
                .iter()
                .take_while(|&&c| c != 0) // filtro fino allo zero terminatore
                .cloned()                  // <-- copia i valori, non i riferimenti
                .collect::<Vec<u16>>(),
        );

        Some(Monitor {
            hmonitor: hmon.0 as isize,
            is_primary: (monitor_info.monitorInfo.dwFlags & 1) != 0,
            rect: monitor_info.monitorInfo.rcMonitor.into(),
            workarea: monitor_info.monitorInfo.rcWork.into(),
            device_name,
        })
    } else {
        None
    };

    let window = Window {
        hwnd: hwnd.0 as isize,
        title,
        is_visible,
        is_maximized,
        rect: rect_opt,
        monitor: monitor
    };

    if let Ok(mut vec) = windows_vec.lock() {
        vec.push(window);
    }

    true.into()
}

pub fn get_maximized_windows() -> Result<Vec<Window>, String> {
    let all = get_all_windows()?;
    Ok(all
        .into_iter()
        .filter(|w| w.is_visible && w.is_maximized)
        .collect()
    )
}

pub fn get_maximized_window_for_monitor(monitor: &Monitor) -> Result<Option<Window>, String> {
    let windows = get_maximized_windows()?;

    for w in windows {
        if let Some(m) = &w.monitor {
            if m.hmonitor == monitor.hmonitor {
                return Ok(Some(w));
            }
        }
    }

    Ok(None)
}

pub fn get_all_windows() -> Result<Vec<Window>, String> {
    let windows_vec: Arc<Mutex<Vec<Window>>> = Arc::new(Mutex::new(Vec::new()));

    // Passiamo il puntatore diretto alla Mutex, senza clone Arc extra
    let ptr = Arc::as_ptr(&windows_vec);

    unsafe {
        EnumWindows(
            Some(enum_windows_proc),
            LPARAM(ptr as isize),
        ).map_err(|e| format!("Error during the windows enumeration:{e}"))?;
    }

    let result = windows_vec.lock().unwrap().clone();
    Ok(result)
}



fn force_window_style_refresh(hwnd: HWND) {
    unsafe {
        let mut rect = RECT::default();
        let _ = GetWindowRect(hwnd, &mut rect);
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        let _ = SetWindowPos(
            hwnd,
            Some(HWND_TOPMOST),
            rect.left,
            rect.top,
            width,
            height,
            SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED,
        );
    }
}

// Rendere il metodo generico che prende un HWND
pub fn remove_titlebar(window: &tauri::Window) {
    unsafe {
        let hwnd = window.hwnd().unwrap() as HWND;
        let style = GetWindowLongPtrW(hwnd, GWL_STYLE);
        let new_style = style & !(WS_CAPTION.0 as isize) & !(WS_THICKFRAME.0 as isize);

        SetWindowLongPtrW(hwnd, GWL_STYLE, new_style);
        force_window_style_refresh(hwnd); // forza il redraw senza titlebar
    }
}

pub fn set_as_wallpaper_background(hwnd_tauri: HWND) -> Result<(), String> {
    let mut workerw = HWND(null_mut());
    let progman = unsafe {
        FindWindowA(PCSTR(b"Progman\0".as_ptr()), PCSTR(null_mut()))
            .map_err(|e| format!("Errore durante l'ottenimento della finestra Progman: {e}"))?
    };

    // manda messaggio 0x052C
    unsafe {
        SendMessageTimeoutA(
            progman,
            0x052C,
            WPARAM(0),
            LPARAM(0),
            SMTO_NORMAL,
            1000,
            None,
        );
    }

    unsafe {
        let shell = FindWindowExA(
            Some(progman),
            None,
            PCSTR(b"SHELLDLL_DefView\0".as_ptr()),
            PCSTR(null_mut()),
        )
        .map_err(|e| format!("errore durante la ricerca della finestra SHELLDLL_DefView: {e}"))?;

        if shell.0 != null_mut() {
            workerw = FindWindowExA(
                Some(progman),
                None,
                PCSTR(b"WorkerW\0".as_ptr()),
                PCSTR(null_mut()),
            )
            .map_err(|e| format!("Errore durante l'ottenimento della finestra WorkerW: {e}"))?;
        }
    }

    if workerw.0 == null_mut() {
        return Err("WorkerW non trovato!".into());
    }

    let hmonitor = unsafe { MonitorFromPoint(POINT { x: 0, y: 0 }, MONITOR_DEFAULTTONEAREST) };
    let mut mi = MONITORINFO {
        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
        rcMonitor: RECT::default(),
        rcWork: RECT::default(),
        dwFlags: 0,
    };
    unsafe {
        GetMonitorInfoA(hmonitor, &mut mi)
            .ok()
            .map_err(|e| format!("Errore durante l'ottenimento delle info del monitor: {e}"))?;
    }

    /*
    unsafe {
        SetWindowPos(
            workerw,
            None,
            mi.rcMonitor.left,
            mi.rcMonitor.top,
            mi.rcMonitor.right - mi.rcMonitor.left,
            mi.rcMonitor.bottom - mi.rcMonitor.top,
            SWP_NOZORDER | SWP_NOACTIVATE,
        ).map_err(|e| format!("Errore durante la modifica della posizione della finestra workerw: {e}"))?;
    }
    */

    println!("{hmonitor:?}");

    unsafe {
        SetWindowPos(
            hwnd_tauri,
            Some(HWND(null_mut())), // non davanti ad altre finestre
            mi.rcMonitor.left,
            mi.rcMonitor.top,
            mi.rcMonitor.right - mi.rcMonitor.left,
            mi.rcMonitor.bottom - mi.rcMonitor.top,
            SWP_NOZORDER | SWP_NOACTIVATE,
        )
        .map_err(|e| format!("Errore durante la modifica della posizione della finestra: {e}"))?
    };

    // 4. Imposta la tua finestra come child di WorkerW
    unsafe {
        SetParent(hwnd_tauri, Some(workerw))
            .map_err(|e| format!("Errore durante il SetParent della finestra child: {e}"))?;
    }

    Ok(())
}
