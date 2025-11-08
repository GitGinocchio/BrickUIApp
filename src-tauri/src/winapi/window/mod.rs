use std::{ptr::null_mut, sync::{Arc, Mutex}};
use serde::{Deserialize, Serialize};
use windows::{Win32::{
    Foundation::{HWND, LPARAM, POINT, RECT, WPARAM},
    Graphics::Gdi::{GetMonitorInfoA, GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MONITORINFOEXW, MonitorFromPoint, MonitorFromWindow},
    UI::WindowsAndMessaging::{
        EnumWindows, FindWindowA, FindWindowExA, GWL_EXSTYLE, GWL_STYLE, GetParent, GetWindowLongPtrW, GetWindowRect, GetWindowTextLengthW, GetWindowTextW, HWND_TOPMOST, IsWindowVisible, IsZoomed, SIZE_RESTORED, SMTO_NORMAL, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SendMessageTimeoutA, SendMessageW, SetForegroundWindow, SetParent, SetWindowLongPtrW, SetWindowPos, ShowWindow, WM_ACTIVATE, WM_SETTINGCHANGE, WM_SIZE, WM_WINDOWPOSCHANGED, WS_CAPTION, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_THICKFRAME
    },
}, core::BOOL};
use windows::core::PCSTR;

pub mod overlay;
pub mod wallpaper;
pub mod utils;

use crate::winapi::{monitor::{Monitor, get_monitor_friendly_name, get_monitor_from_hwnd}, rect::Rect, window::utils::{get_window_class, is_tauri_window}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    pub hwnd: isize,
    pub class: Option<String>,
    pub title: String,
    pub is_visible: bool,
    pub is_maximized: bool,
    pub is_taskbar: bool,
    pub is_self: bool,
    pub rect: Option<Rect>,
    pub monitor: Option<Monitor>
}

impl Window {
    pub fn from_hwnd(hwnd: isize) -> Result<Self, String> {
        unsafe {
            let win_hwnd = HWND(hwnd as *mut _);

            let length = GetWindowTextLengthW(win_hwnd);
            let mut buffer = vec![0u16; (length + 1) as usize];
            let title_len = GetWindowTextW(win_hwnd, &mut buffer);
            let title = String::from_utf16_lossy(&buffer[..title_len as usize]);

            let is_visible = IsWindowVisible(win_hwnd).as_bool();
            let is_maximized = IsZoomed(win_hwnd).as_bool();

            let mut rect: RECT = RECT::default();
            let rect_opt = if GetWindowRect(win_hwnd, &mut rect).is_ok() {
                Some(Rect {
                    left: rect.left,
                    top: rect.top,
                    right: rect.right,
                    bottom: rect.bottom,
                })
            } else {
                None
            };

            let monitor = get_monitor_from_hwnd(hwnd)?;

            Ok(Window {
                hwnd: hwnd,
                class: get_window_class(win_hwnd),
                title,
                is_visible,
                is_maximized,
                is_self: is_tauri_window(win_hwnd)?,
                is_taskbar: is_taskbar_window(win_hwnd)?,
                rect: rect_opt,
                monitor: Some(monitor),
            })
        }
    }

    pub fn unmaximize(&self) -> Result<(), String> {
        if self.hwnd == 0 {
            return Err("Hwnd can't be 0".into());
        }

        unsafe {
            let hwnd = HWND(self.hwnd as *mut _);

            // Mostra la finestra minimizzata
            let success = ShowWindow(hwnd, SW_MINIMIZE);
            if !success.as_bool() {
                return Err("Failed to maximize window".into());
            }
        }

        Ok(())
    }

    pub fn maximize(&self) -> Result<(), String> {
        if self.hwnd == 0 {
            return Err("Hwnd can't be 0".into());
        }

        unsafe {
            let hwnd = HWND(self.hwnd as *mut _);

            // Mostra la finestra massimizzata
            let success = ShowWindow(hwnd, SW_MAXIMIZE);
            if !success.as_bool() {
                return Err("Failed to maximize window".into());
            }
        }

        Ok(())
    }

    pub fn set_rect(&self, rect: &Rect) -> Result<(), String> {
        if self.hwnd == 0 {
            return Err("Hwnd can't be 0".into());
        }

        unsafe {
            let hwnd = HWND(self.hwnd as *mut _);

            // Ripristina se massimizzata
            let _ = ShowWindow(hwnd, SW_RESTORE);

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

    /// Notifica alla finestra che la workarea del monitor è cambiata.
    pub fn notify_workarea_change(&self) -> Result<(), String> {
        if self.hwnd == 0 {
            return Err("Hwnd can't be 0".into());
        }

        unsafe {
            let hwnd = HWND(self.hwnd as *mut _);

            // WM_SETTINGCHANGE con "Shell_TrayWnd" per notificare la workarea
            let result = SendMessageW(
                hwnd,
                WM_SETTINGCHANGE,
                Some(WPARAM(0)),
                Some(LPARAM("Shell_TrayWnd\0".as_ptr() as isize)),
            );

            // Non tutte le finestre restituiscono qualcosa di significativo
            // Quindi di solito non serve controllare il valore di ritorno
        }

        Ok(())
    }

    /// Forza la finestra a ridisegnarsi / reagire a cambiamenti di dimensione
    pub fn refresh_window(&self) -> Result<(), String> {
        if self.hwnd == 0 {
            return Err("Hwnd can't be 0".into());
        }

        unsafe {
            let hwnd = HWND(self.hwnd as *mut _);

            // Notifica che la posizione della finestra è cambiata
            SendMessageW(hwnd, WM_WINDOWPOSCHANGED, Some(WPARAM(0)), Some(LPARAM(0)));
            
            // Notifica un ridimensionamento (anche se non cambia)
            SendMessageW(hwnd, WM_SIZE, Some(WPARAM(SIZE_RESTORED as _)), Some(LPARAM(0)));
        }

        Ok(())
    }

    /// Porta temporaneamente la finestra in foreground per forzare ridisegno o ridimensionamento.
    pub fn force_focus(&self) -> Result<(), String> {
        if self.hwnd == 0 {
            return Err("Hwnd can't be 0".into());
        }

        unsafe {
            let hwnd = HWND(self.hwnd as *mut _);

            // Porta la finestra in foreground
            if !SetForegroundWindow(hwnd).as_bool() {
                return Err("Failed to set window to foreground".into());
            }

            // Facoltativo: inviare un WM_ACTIVATE per essere sicuri che riceva focus
            SendMessageW(hwnd, WM_ACTIVATE, Some(WPARAM(1)), Some(LPARAM(0)));
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

        let friendly_name = get_monitor_friendly_name(&device_name);

        Some(Monitor {
            hmonitor: hmon.0 as isize,
            is_primary: (monitor_info.monitorInfo.dwFlags & 1) != 0,
            rect: monitor_info.monitorInfo.rcMonitor.into(),
            workarea: monitor_info.monitorInfo.rcWork.into(),
            device_name,
            friendly_name
        })
    } else {
        None
    };

    let window = Window {
        hwnd: hwnd.0 as isize,
        class: get_window_class(hwnd),
        title,
        is_visible,
        is_maximized,
        is_self: is_tauri_window(hwnd).unwrap_or(false),
        is_taskbar: is_taskbar_window(hwnd).unwrap_or(false),
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

pub fn get_visible_windows() -> Result<Vec<Window>, String> {
    let all = get_all_windows()?;
    Ok(all
        .into_iter()
        .filter(|w| w.is_visible)
        .collect()
    )
}

pub fn get_monitor_maximized_window(monitor: &Monitor) -> Result<Option<Window>, String> {
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

pub fn get_monitor_windows(monitor: &Monitor) -> Result<Vec<Window>, String> {
    let all = get_all_windows()?;

    Ok(all
        .into_iter()
        .filter(|w| {
            if let Some(m) = &w.monitor {
                m.hmonitor == monitor.hmonitor
            } else {
                false
            }
        })
        .collect()
    )
}

pub fn get_monitor_visible_windows(monitor: &Monitor) -> Result<Vec<Window>, String> {
    let all = get_visible_windows()?;
    Ok(all
        .into_iter()
        .filter(|w| {
            if let Some(wm) = &w.monitor && wm.hmonitor != monitor.hmonitor { return false; }

            true
        })
        .collect()
    )
}

pub fn get_monitor_taskbar_windows(monitor: &Monitor) -> Result<Vec<Window>, String> {
    let all = get_taskbar_windows()?;
    Ok(all
        .into_iter()
        .filter(|w| {
            if let Some(wm) = &w.monitor && wm.hmonitor != monitor.hmonitor { return false; }

            true
        })
        .collect()
    )
}

pub fn get_taskbar_windows() -> Result<Vec<Window>, String> {
    let all = get_visible_windows()?;
    Ok(all
        .into_iter()
        .filter(|w| {
            if !w.is_taskbar { return false; }

            true
        })
        .collect()
    )
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

pub fn is_taskbar_window(hwnd: HWND) -> Result<bool, String> {
    unsafe {
        let style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
        let parent = GetParent(hwnd).unwrap_or(HWND::default());

        Ok(IsWindowVisible(hwnd).as_bool() && parent.0.is_null() && (style & WS_EX_TOOLWINDOW.0) == 0)
    }
}

fn force_window_style_refresh(hwnd: HWND) {
    unsafe {
        let _ = SetWindowPos(
            hwnd,
            None,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED,
        );
    }
} 

// Rendere il metodo generico che prende un HWND
pub fn remove_titlebar(hwnd: HWND) {
    unsafe {
        let style = GetWindowLongPtrW(hwnd, GWL_STYLE);
        let new_style = style & !(WS_CAPTION.0 as isize) & !(WS_THICKFRAME.0 as isize);

        SetWindowLongPtrW(hwnd, GWL_STYLE, new_style);
        force_window_style_refresh(hwnd); // forza il redraw senza titlebar
    }
}

/// Imposta la finestra come topmost, sopra anche alla taskbar
pub fn set_window_topmost(hwnd: HWND) -> Result<(), String> {
    unsafe {
        // Imposta gli stili
        let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_TOPMOST.0 as isize);

        // Imposta lo z-order
        SetWindowPos(
            hwnd,
            Some(HWND_TOPMOST),
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
        )
        .map_err(|e| format!("Errore durante l'impostazione della finestra come topmost: {e}"))?;

        // Forza il refresh degli stili
        force_window_style_refresh(hwnd);
    }
    Ok(())
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
