use windows::Win32::UI::WindowsAndMessaging::{SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_SENDWININICHANGE, SPIF_UPDATEINIFILE, SPI_SETWORKAREA};
use windows::Win32::Foundation::RECT;

use crate::winapi::desktop::refresh_desktop_icons;
use crate::winapi::taskbar::{get_taskbar_rect, is_taskbar_autohide};
use crate::winapi::window::get_maximized_window_for_monitor;
use crate::winapi::rect::{OptionalRect, Rect};

use super::{get_all_monitors, get_primary_monitor, Monitor};


fn notify_changes(monitor: &Monitor, rect: &mut Rect) -> Result<(), String> {
    if !is_taskbar_autohide() {
        if let Some(tb_rect) = get_taskbar_rect() {
            rect.bottom = rect.bottom.min(tb_rect.top);
        }
    }

    let mut win_rect: RECT = (*rect).into();
    
    // 1. Aggiorna il registro (modifica work area specifica)
    unsafe {
        SystemParametersInfoW(
            SPI_SETWORKAREA,
            0,
            Some((&mut win_rect) as *mut _ as *mut _),
            SPIF_UPDATEINIFILE,
        )
        .map_err(|e| format!("Error while updating work area for {}: {e}", monitor.device_name))?;
    }

    // 2. Notifica le applicazioni (broadcast moderno)
    unsafe {
        SystemParametersInfoW(
            SPI_SETWORKAREA,
            0,
            Some((&mut win_rect) as *mut _ as *mut _),
            SPIF_SENDCHANGE,
        )
        .map_err(|e| format!("Error while updating work area for {}: {e}", monitor.device_name))?;
    }

    // 3. Compatibilità legacy
    unsafe {
        SystemParametersInfoW(
            SPI_SETWORKAREA,
            0,
            Some((&mut win_rect) as *mut _ as *mut _),
            SPIF_SENDWININICHANGE,
        )
        .map_err(|e| format!("Error while updating work area for {}: {e}", monitor.device_name))?;
    }

    refresh_desktop_icons()?;

    if let Some(window) = get_maximized_window_for_monitor(monitor)? {
        window.set_rect(&rect)?;
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

pub fn set_workarea_for_all_monitors(margins: &OptionalRect) -> Result<(), String> {
    let monitors = get_all_monitors()?;

    for monitor in monitors {
        let rect = monitor.workarea.apply_margins(margins, &monitor.rect);

        notify_changes(&monitor, &mut rect.into())?;
    }

    Ok(())
}

pub fn set_monitor_workarea(margins: &OptionalRect, monitor: Option<&Monitor>) -> Result<(), String> {
    let monitor = match monitor {
        Some(monitor) => monitor,
        None => &get_primary_monitor()?
    };

    let mut rect = monitor.workarea.apply_margins(margins, &monitor.rect);

    notify_changes(&monitor, &mut rect)?;

    Ok(())
}


impl Monitor {
    pub fn set_workarea(&self, margins: &OptionalRect) -> Result<(), String> {
        set_monitor_workarea(margins, Some(&self))
    }
}