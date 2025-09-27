use std::ptr::null_mut;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::{BOOL, PCSTR};

// Funzione che cerca il WorkerW nascosto
fn get_workerw() -> Result<HWND, String> {
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

    Ok(workerw)
}

// Callback usata da EnumDisplayMonitors
extern "system" fn enum_monitor_callback(
    hmonitor: HMONITOR,
    _: HDC,
    _: *mut RECT,
    lparam: LPARAM,
) -> BOOL {
    unsafe {
        let hwnd_tauri = HWND(lparam.0 as _); // passiamo HWND come parametro

        let mut mi = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            rcMonitor: RECT::default(),
            rcWork: RECT::default(),
            dwFlags: 0,
        };

        println!("{hmonitor:?}");

        if GetMonitorInfoA(hmonitor, &mut mi).as_bool() {
            // Qui ridimensioniamo/spostiamo la finestra sul rettangolo del monitor
            /*
            SetWindowPos(
                hwnd_tauri,
                Some(HWND(std::ptr::null_mut())),
                mi.rcMonitor.left,
                mi.rcMonitor.top,
                mi.rcMonitor.right - mi.rcMonitor.left,
                mi.rcMonitor.bottom - mi.rcMonitor.top,
                SWP_NOZORDER | SWP_NOACTIVATE,
            ).unwrap();
            */

            let workerw = get_workerw().unwrap();
            println!("{workerw:?}");

            SetParent(hwnd_tauri, Some(workerw)).unwrap();
        }
    }

    BOOL(1) // continua enumerazione
}

// Funzione principale
pub fn set_as_wallpaper_background_all_monitors(hwnd_tauri: HWND) -> Result<(), String> {
    unsafe {
        let lparam = LPARAM(hwnd_tauri.0 as isize);
        EnumDisplayMonitors(
            Some(HDC(std::ptr::null_mut())),
            None,
            Some(enum_monitor_callback),
            lparam,
        );
        /*
        .ok()
        .map_err(|e| format!("Errore durante EnumDisplayMonitors: {e}"))?;
        */
    }

    Ok(())
}
