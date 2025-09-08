use windows::Win32::{
    Foundation::{HWND, LPARAM, RECT, WPARAM}, Graphics::Gdi::{MonitorFromWindow, GetMonitorInfoA, MONITORINFO, MONITORINFOEXA, MONITOR_DEFAULTTONEAREST}, UI::WindowsAndMessaging::{
        FindWindowA, FindWindowExA, GetSystemMetrics, GetWindowLongA, GetWindowLongPtrW, 
        GetWindowRect, SendMessageTimeoutA, SetLayeredWindowAttributes, SetParent, 
        SetWindowLongA, SetWindowLongPtrW, SetWindowPos, 
        GWL_EXSTYLE, GWL_STYLE, HWND_TOPMOST, LWA_ALPHA, SMTO_NORMAL, 
        SM_CXSCREEN, SM_CYSCREEN, SWP_FRAMECHANGED, SWP_NOACTIVATE, 
        SWP_NOZORDER, WS_CAPTION, WS_CHILD, WS_EX_LAYERED, WS_EX_TOOLWINDOW, 
        WS_THICKFRAME, WS_VISIBLE
    }
};
use windows::core::{PCSTR};
use std::ptr::null_mut;


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
            None
        );
    }

    // scorri tutte le finestre WorkerW
    let mut hwnd = HWND(null_mut());
    loop {
        hwnd = unsafe {
            FindWindowExA(
                None, 
                Some(hwnd), 
                PCSTR(b"WorkerW\0".as_ptr()), 
                PCSTR(null_mut()))
                .unwrap_or(HWND(null_mut())
        )};
        if hwnd.0 == null_mut() { break; }

        let shell = unsafe {
            FindWindowExA(
                Some(hwnd), 
                None, 
                PCSTR(b"SHELLDLL_DefView\0".as_ptr()), 
                PCSTR(null_mut())).unwrap_or(HWND(null_mut()))
        };
        if shell.0 == null_mut() {
            workerw = hwnd; // questo WorkerW non ha DefView → è quello giusto
            break;
        }
    }

    unsafe {
        let shell = FindWindowExA(
            Some(progman), 
            None, 
            PCSTR(b"SHELLDLL_DefView\0".as_ptr()), 
            PCSTR(null_mut())
        ).map_err(|e| format!("errore durante la ricerca della finestra SHELLDLL_DefView: {e}"))?;
        
        if shell.0 != null_mut() {
            workerw = FindWindowExA(
                Some(progman), 
                None, 
                PCSTR(b"WorkerW\0".as_ptr()), 
                PCSTR(null_mut())
            ).map_err(|e| format!("Errore durante l'ottenimento della finestra WorkerW: {e}"))?;
        }
    }

    if workerw.0 == null_mut() {
        return Err("WorkerW non trovato!".into());
    }

    /*
    let style = unsafe { GetWindowLongA(hwnd_tauri, GWL_STYLE) };
    unsafe { SetWindowLongA(hwnd_tauri, GWL_STYLE, style | WS_CHILD.0 as i32 | WS_VISIBLE.0 as i32) };

    // Massimizza la finestra sullo schermo
    unsafe { 
        SetWindowPos(
            hwnd_tauri,
            Some(HWND(null_mut())),
            0,
            0,
            GetSystemMetrics(SM_CXSCREEN),
            GetSystemMetrics(SM_CYSCREEN),
            SWP_NOZORDER | SWP_NOACTIVATE,
        ).map_err(|e| format!("Errore durante la modifica della posizione della finestra: {e}"))?;
    }

    unsafe { 
        SetWindowLongA(hwnd_tauri, GWL_EXSTYLE, WS_EX_TOOLWINDOW.0 as i32 | WS_EX_LAYERED.0 as i32);
        SetLayeredWindowAttributes(
            hwnd_tauri, 
            windows::Win32::Foundation::COLORREF(0), 
            255, 
            LWA_ALPHA
        ).map_err(|e| format!("Errore durante l'impostazione di un attributo della finestra: {e}"))?; 
    };
    */

    let hmonitor = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST) };
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

    unsafe {
        SetWindowPos(
            hwnd,
            Some(HWND(null_mut())), // non davanti ad altre finestre
            mi.rcMonitor.left,
            mi.rcMonitor.top,
            mi.rcMonitor.right - mi.rcMonitor.left,
            mi.rcMonitor.bottom - mi.rcMonitor.top,
            SWP_NOZORDER | SWP_NOACTIVATE
        ).map_err(|e| format!("Errore durante la modifica della posizione della finestra: {e}"))?
    };


    // 4. Imposta la tua finestra come child di WorkerW
    unsafe {
        SetParent(
            hwnd_tauri, 
            Some(workerw)
        ).map_err(|e| format!("Errore durante il SetParent della finestra child: {e}"))?;
    }

    Ok(())
}
