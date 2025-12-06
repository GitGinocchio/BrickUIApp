use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::{
    SPI_SETWORKAREA, SPIF_SENDCHANGE, SPIF_SENDWININICHANGE, SPIF_UPDATEINIFILE, SWP_NOACTIVATE,
    SWP_NOZORDER, SetWindowPos, SystemParametersInfoW,
};

use crate::winapi::taskbar::is_taskbar_autohide;
use crate::winapi::window::get_monitor_taskbar_windows;
use crate::winapi::{
    desktop::refresh_desktop_icons,
    rect::{OptionalRect, Rect},
    taskbar::get_taskbar_rect,
};

use super::{Monitor, get_all_monitors, get_primary_monitor};

#[cfg_attr(feature = "profiling", tracing::instrument)]
fn notify_all_monitor_windows(monitor: &Monitor) -> Result<(), String> {
    let mut windows = get_monitor_taskbar_windows(monitor)?;

    for window in &mut windows {
        if window.is_self {
            continue;
        }
        if let Some(rect) = window.rect.as_mut() {
            let win_hwnd = HWND(window.hwnd as *mut _);

            // Cambia la dimensione delle finestre in modo da triggerare un update
            // e un redraw, in modo che la finestra si adatti alla nuova workarea
            unsafe {
                match SetWindowPos(
                    win_hwnd,
                    None,
                    rect.left,
                    rect.top,
                    rect.width() + 1,
                    rect.height(),
                    SWP_NOZORDER | SWP_NOACTIVATE,
                ) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Could not resize window: {e}"),
                };
                match SetWindowPos(
                    win_hwnd,
                    None,
                    rect.left,
                    rect.top,
                    rect.width(),
                    rect.height(),
                    SWP_NOZORDER | SWP_NOACTIVATE,
                ) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Could not resize window: {e}"),
                };
            }
        }
    }

    Ok(())
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
fn apply_workarea(monitor: &Monitor, rect: &Rect) -> Result<Rect, String> {
    let mut rect = rect.clone();

    // Limita il bottom se la taskbar non è in autohide
    if monitor.is_primary && !is_taskbar_autohide() {
        if let Some(tb_rect) = get_taskbar_rect() {
            rect.bottom = rect.bottom.min(tb_rect.top);
        }
    }

    let mut win_rect: RECT = rect.into();

    unsafe {
        // Aggiorna il registro
        SystemParametersInfoW(
            SPI_SETWORKAREA,
            0,
            Some((&mut win_rect) as *mut _ as *mut _),
            SPIF_UPDATEINIFILE,
        )
        .map_err(|e| format!("Error updating work area for {}: {e}", monitor.device_name))?;

        // Notifica le applicazioni moderne
        SystemParametersInfoW(
            SPI_SETWORKAREA,
            0,
            Some((&mut win_rect) as *mut _ as *mut _),
            SPIF_SENDCHANGE,
        )
        .map_err(|e| format!("Error sending change for {}: {e}", monitor.device_name))?;

        // Compatibilità legacy
        SystemParametersInfoW(
            SPI_SETWORKAREA,
            0,
            Some((&mut win_rect) as *mut _ as *mut _),
            SPIF_SENDWININICHANGE,
        )
        .map_err(|e| {
            format!(
                "Error sending WININICHANGE for {}: {e}",
                monitor.device_name
            )
        })?;
    }

    // Refresh desktop e finestre massimizzate
    refresh_desktop_icons()?;

    notify_all_monitor_windows(&monitor)?;

    Ok(rect)
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn set_workareas(workarea: &Rect) -> Result<(), String> {
    for monitor in get_all_monitors()? {
        apply_workarea(&monitor, workarea)?;
    }
    Ok(())
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn set_workareas_margins(margins: &OptionalRect) -> Result<(), String> {
    for monitor in get_all_monitors()? {
        let rect = monitor.workarea.apply_margins(margins, &monitor.rect);
        apply_workarea(&monitor, &rect)?;
    }
    Ok(())
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn reset_workareas() -> Result<(), String> {
    for monitor in get_all_monitors()?.iter_mut() {
        if monitor.is_primary {
            let tbrect = get_taskbar_rect().ok_or("No taskbar rect".to_string())?;
            monitor.rect.bottom = monitor.rect.bottom - tbrect.height();
        }

        monitor.set_workarea(&monitor.rect.clone())?;
    }
    Ok(())
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn set_workarea(workarea: &Rect, monitor: Option<&Monitor>) -> Result<Rect, String> {
    let monitor = match monitor {
        Some(monitor) => monitor,
        None => &get_primary_monitor()?,
    };
    let applied = apply_workarea(monitor, workarea)?;

    Ok(applied)
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn set_workarea_margins(
    margins: &OptionalRect,
    monitor: Option<&Monitor>,
) -> Result<Rect, String> {
    let monitor = match monitor {
        Some(monitor) => monitor,
        None => &get_primary_monitor()?,
    };
    let rect = monitor.workarea.apply_margins(margins, &monitor.rect);
    apply_workarea(monitor, &rect)
}

impl Monitor {
    pub fn set_workarea(&mut self, workarea: &Rect) -> Result<(), String> {
        let applied = set_workarea(workarea, Some(self))?;
        self.workarea = applied;
        Ok(())
    }

    pub fn set_workarea_margins(&mut self, margins: &OptionalRect) -> Result<(), String> {
        let applied = set_workarea_margins(margins, Some(self))?;
        self.workarea = applied;
        Ok(())
    }
}
