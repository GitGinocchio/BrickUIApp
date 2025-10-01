use windows::Win32::UI::WindowsAndMessaging::{SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_SENDWININICHANGE, SPIF_UPDATEINIFILE, SPI_SETWORKAREA};
use windows::Win32::Foundation::RECT;

use crate::winapi::monitor::{get_all_monitors, get_primary_monitor, Monitor};
use crate::winapi::taskbar::{get_taskbar_rect, is_taskbar_autohide};
use crate::winapi::Rect;


fn notify_changes(monitor: &Monitor, rect: &mut RECT) -> Result<(), String> {
    if !is_taskbar_autohide() {
        if let Some(tb_rect) = get_taskbar_rect() {
            rect.bottom = rect.bottom.min(tb_rect.top);
        }
    }
    
    // 1. Aggiorna il registro (modifica work area specifica)
    unsafe {
        SystemParametersInfoW(
            SPI_SETWORKAREA,
            0,
            Some(&mut (rect.clone()) as *mut _ as *mut _),
            SPIF_UPDATEINIFILE,
        )
        .map_err(|e| format!("Error while updating work area for {}: {e}", monitor.device_name))?;
    }

    // 2. Notifica le applicazioni (broadcast moderno)
    unsafe {
        SystemParametersInfoW(
            SPI_SETWORKAREA,
            0,
            Some(&mut (rect.clone()) as *mut _ as *mut _),
            SPIF_SENDCHANGE,
        )
        .map_err(|e| format!("Error while updating work area for {}: {e}", monitor.device_name))?;
    }

    // 3. Compatibilità legacy
    unsafe {
        SystemParametersInfoW(
            SPI_SETWORKAREA,
            0,
            Some(&mut (rect.clone()) as *mut _ as *mut _),
            SPIF_SENDWININICHANGE,
        )
        .map_err(|e| format!("Error while updating work area for {}: {e}", monitor.device_name))?;
    }

    // 4. Notifica tutte le finestre
    /*
    unsafe {
        let mut result: usize = 0;
        let _ = SendMessageTimeoutW(
            HWND(HWND_BROADCAST.0),
            WM_SETTINGCHANGE,
            WPARAM(0),
            LPARAM(0),
            SMTO_ABORTIFHUNG,
            1000,
            Some(&mut result as *mut _),
        );
    };
    */

    Ok(())
}

pub fn set_workarea_for_all_monitors(margins: Rect) -> Result<(), String> {
    let monitors = get_all_monitors()?;

    for monitor in monitors {
        let rect = RECT {
            left: monitor.monitor.left + margins.left as i32,
            top: monitor.monitor.top + margins.top as i32,
            right: monitor.monitor.right - margins.right as i32,
            bottom: monitor.monitor.bottom - margins.bottom as i32,
        };

        notify_changes(&monitor, &mut rect.into())?;
    }

    Ok(())
}

pub fn set_monitor_workarea(margins: Rect, monitor: Option<Monitor>) -> Result<(), String> {
    let monitor = match monitor {
        Some(monitor) => monitor,
        None => get_primary_monitor()?
    };

    let mut rect = RECT {
        left: monitor.monitor.left + margins.left as i32,
        top: monitor.monitor.top + margins.top as i32,
        right: monitor.monitor.right - margins.right as i32,
        bottom: monitor.monitor.bottom - margins.bottom as i32,
    };

    notify_changes(&monitor, &mut rect)?;

    Ok(())
}
