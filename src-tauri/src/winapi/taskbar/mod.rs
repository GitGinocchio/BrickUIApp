pub mod apps;
pub mod tray;

use std::{thread, time::Duration};

use tauri::{AppHandle, Manager, PhysicalSize, PhysicalPosition};
use windows::{
    Win32::{
        Foundation::LPARAM,
        UI::{
            Shell::{ABM_GETSTATE, ABM_GETTASKBARPOS, ABM_SETSTATE, ABS_ALWAYSONTOP, ABS_AUTOHIDE, APPBARDATA, SHAppBarMessage},
            WindowsAndMessaging::{
                FindWindowA, GWL_EXSTYLE, GetWindowLongA, HWND_BOTTOM, HWND_TOPMOST, LWA_ALPHA, SW_HIDE, SW_SHOW, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SetLayeredWindowAttributes, SetWindowLongA, SetWindowPos, ShowWindow, WS_EX_LAYERED
            },
        },
    }, core::{Error as WinError, PCSTR}
};

use crate::winapi::{monitor::get_primary_monitor, rect::Rect, window::{remove_titlebar, set_window_topmost}};

pub fn get_taskbar_rect() -> Option<Rect> {
    let mut data = APPBARDATA {
        cbSize: std::mem::size_of::<APPBARDATA>() as u32,
        ..Default::default()
    };
    let res = unsafe { SHAppBarMessage(ABM_GETTASKBARPOS, &mut data) };
    if res != 0 { Some(data.rc.into()) } else { None }
}

pub fn restore_taskbar() -> Result<(), String> {
    unsafe {
        let class_name = b"Shell_TrayWnd\0".as_ptr();
        let taskbar = FindWindowA(PCSTR(class_name), PCSTR::null()).map_err(|e| e.to_string())?;

        if taskbar.is_invalid() {
            return Err("Taskbar hwnd is invalid".into());
        }

        // Disattiva l'autohide
        // TODO: Sarebbe da disattivare solo se non e' abilitato dall'utente dal sistema operativo
        let mut abd = APPBARDATA {
            cbSize: std::mem::size_of::<APPBARDATA>() as u32,
            hWnd: taskbar,
            uEdge: 0,
            rc: Default::default(),
            lParam: LPARAM(ABS_ALWAYSONTOP as isize), // torna a comportamento normale
            uCallbackMessage: 0,
        };

        SHAppBarMessage(ABM_SETSTATE, &mut abd);

        // Ottiene lo stile attuale della finestra
        let style = GetWindowLongA(taskbar, GWL_EXSTYLE);
        // Imposta lo stile della finestra con l'aggiunta di WS_EX_LAYERED
        SetWindowLongA(taskbar, GWL_EXSTYLE, style | WS_EX_LAYERED.0 as i32);
        // Imposta l'alpha a 255 (visibile)
        SetLayeredWindowAttributes(
            taskbar, 
            windows::Win32::Foundation::COLORREF(0), 
            255, 
            LWA_ALPHA
        ).map_err(|e| format!("Error setting layeredWindowAttributes: {e}"))?;

        // Imposta la taskbar come HWND_TOPMOST (sopra a tutte le finestre)
        SetWindowPos(
            taskbar,
            Some(HWND_TOPMOST),
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
        ).map_err(|e| format!("Error setting window position: {e}"))?;

        // Imposta comunque la taskbar "visibile"
        ShowWindow(taskbar, SW_SHOW)
            .ok()
            .map_err(|e| format!("Error showing window: {e}"))?;
    }

    Ok(())
}

pub fn hide_taskbar(app_handle: &AppHandle) -> Result<(), String> {
    let mut pmonitor = get_primary_monitor()?;
    let mut workarea = pmonitor.workarea;
    let tbrect = get_taskbar_rect().ok_or("No taskbar rect".to_string())?;

    println!("{workarea:?}");

    //restore_taskbar()?;

    let class_name = b"Shell_TrayWnd\0".as_ptr();
    let taskbar = unsafe { FindWindowA(PCSTR(class_name), PCSTR::null()).map_err(|e| e.to_string())? };
    if taskbar.is_invalid() {
        return Err("Invalid taskbar hwnd".into());
    }

    let overlay_window = app_handle.get_window("overlay")
        .ok_or("Error obtaining overlay window".to_string())?;

    let overlay_hwnd = overlay_window
        .hwnd()
        .map_err(|e| format!("Error obtaining overlay window handle: {e}"))?;

    remove_titlebar(overlay_hwnd);
    set_window_topmost(overlay_hwnd)?;

    unsafe {
        // Rendi la taskbar completamente trasparente ma attiva
        let style = GetWindowLongA(taskbar, GWL_EXSTYLE);
        SetWindowLongA(taskbar, GWL_EXSTYLE, style | WS_EX_LAYERED.0 as i32);
        SetLayeredWindowAttributes(
            taskbar, 
            windows::Win32::Foundation::COLORREF(0), 
            0, 
            LWA_ALPHA
        ).map_err(|e| format!("Error while setting layeredWindowAttributes: {e}"))?; 

        let mut abd = APPBARDATA {
            cbSize: std::mem::size_of::<APPBARDATA>() as u32,
            hWnd: taskbar,
            uEdge: 0,
            rc: Default::default(),
            lParam: LPARAM(ABS_AUTOHIDE as isize),
            uCallbackMessage: 0,
        };
        SHAppBarMessage(ABM_SETSTATE, &mut abd);

        // Mantienila visibile logicamente
        ShowWindow(taskbar, SW_SHOW)
            .ok()
            .map_err(|e| format!("Error showing window: {e}"))?;

        // Spostala sotto a tutte le altre applicazioni
        SetWindowPos(
            taskbar,
            Some(HWND_BOTTOM),
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW,
        ).map_err(|e| format!("Error settings taskbar position: {e}"))?;
    }

    if workarea.bottom >= pmonitor.rect.bottom - tbrect.height() {
        workarea.bottom = pmonitor.rect.bottom;
    }
    pmonitor.set_workarea(&workarea)?;

    // Set position to monitor top-left (0,0)
    overlay_window.set_position(PhysicalPosition {
        x: pmonitor.rect.left,
        y: pmonitor.rect.top
    }).map_err(|e| format!("Error setting window position: {e}"))?;

    let size = PhysicalSize {
        height: pmonitor.rect.height() as u32 - 1,
        width: pmonitor.rect.width() as u32 - 1
    };

    overlay_window.set_max_size(Some(size)).map_err(|e| format!("Error setting monitor max size: {e}"))?;

    // Force overlay window to use full monitor dimensions
    overlay_window.set_size(size).map_err(|e| format!("Error setting monitor size: {e}"))?;

    Ok(())
}

pub fn show_taskbar(app_handle: &AppHandle) -> Result<(), String> {
    let mut pmonitor = get_primary_monitor()?;
    let tbrect = get_taskbar_rect().ok_or("No taskbar rect".to_string())?;
    let mut workarea = pmonitor.workarea;

    println!("{workarea:?}");

    restore_taskbar()?;

    // Se la workarea arriva fino alla parte bassa dello schermo...
    if workarea.bottom > pmonitor.rect.bottom - tbrect.height() {
        // Fai si che arrivi a sopra alla taskbar (comportamento normale)
        workarea.bottom -= tbrect.height();
    }
    pmonitor.set_workarea(&workarea)?;

    let overlay_window = app_handle.get_window("overlay")
        .ok_or("Error obtaining overlay window".to_string())?;

    let overlay_hwnd = overlay_window
        .hwnd()
        .map_err(|e| format!("Error obtaining overlay window handle: {e}"))?;

    remove_titlebar(overlay_hwnd);
    set_window_topmost(overlay_hwnd)?;

    // Set position to monitor top-left (0,0)
    overlay_window.set_position(PhysicalPosition {
        x: pmonitor.rect.left,
        y: pmonitor.rect.top
    }).map_err(|e| format!("Error setting window position: {e}"))?;

    let size = PhysicalSize {
        height: pmonitor.rect.height() as u32,
        width: pmonitor.rect.width() as u32
    };

    overlay_window.set_max_size(Some(size)).map_err(|e| format!("Error setting monitor max size: {e}"))?;

    // Force overlay window to use full monitor dimensions
    overlay_window.set_size(size).map_err(|e| format!("Error setting monitor size: {e}"))?;

    Ok(())
}

pub fn is_taskbar_autohide() -> bool {
    let mut appbar_data = APPBARDATA {
        cbSize: std::mem::size_of::<APPBARDATA>() as u32,
        ..Default::default()
    };

    let state = unsafe { SHAppBarMessage(ABM_GETSTATE, &mut appbar_data) };

    if state == 0 {
        return false;
    }

    (state & ABS_AUTOHIDE as usize) != 0
}

// Legacy methods

pub fn legacy_reset_taskbar() -> Result<(), String> {
    unsafe {
        let class_name = b"Shell_TrayWnd\0".as_ptr();
        let taskbar = FindWindowA(PCSTR(class_name), PCSTR::null()).map_err(|e| e.to_string())?;

        if taskbar.is_invalid() {
            return Err(WinError::from_win32().to_string());
        }

        // Riporta la taskbar al suo posto
        SetWindowPos(
            taskbar,
            Some(HWND_TOPMOST),
            0,
            0,
            0,
            0,
            SWP_NOSIZE | SWP_NOMOVE,
        )
        .map_err(|e| e.to_string())?;

        // Disattiva l'autohide
        let mut abd = APPBARDATA {
            cbSize: std::mem::size_of::<APPBARDATA>() as u32,
            hWnd: taskbar,
            uEdge: 0,
            rc: Default::default(),
            lParam: LPARAM(ABS_ALWAYSONTOP as isize), // torna a comportamento normale
            uCallbackMessage: 0,
        };

        SHAppBarMessage(ABM_SETSTATE, &mut abd);

        // Mostra la barra delle applicazioni
        let _ = ShowWindow(taskbar, SW_SHOW);

        Ok(())
    }
}

pub fn legacy_hide_taskbar(app_handle: &AppHandle) -> Result<(), String> {
    let mut pmonitor = get_primary_monitor()?;
    let tbrect = get_taskbar_rect().ok_or("No taskbar rect".to_string())?;
    let mut workarea = pmonitor.workarea;

    legacy_reset_taskbar()?;

    let class_name = b"Shell_TrayWnd\0".as_ptr();
    let taskbar = unsafe { FindWindowA(PCSTR(class_name), PCSTR::null()).map_err(|e| e.to_string())? };

    if taskbar.is_invalid() {
        return Err(WinError::from_win32().to_string());
    }

    // Sposta la taskbar fuori dallo schermo
    unsafe {
        SetWindowPos(
            taskbar,
            Some(HWND_BOTTOM),
            0,
            -100,
            0,
            0,
            SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE,
        ).map_err(|e| e.to_string())?;
    }

    // Abilita l'autohide
    let mut abd = APPBARDATA {
        cbSize: std::mem::size_of::<APPBARDATA>() as u32,
        hWnd: taskbar,
        uEdge: 0,
        rc: Default::default(),
        lParam: LPARAM(ABS_AUTOHIDE as isize),
        uCallbackMessage: 0,
    };

    let _ = unsafe { SHAppBarMessage(ABM_SETSTATE, &mut abd) };

    let _ = unsafe { ShowWindow(taskbar, SW_HIDE) };

    if workarea.bottom >= pmonitor.rect.bottom - tbrect.height() {
        workarea.bottom = pmonitor.rect.bottom;
    }
    pmonitor.set_workarea(&workarea)?;

    let overlay_window = app_handle.get_window("overlay")
        .ok_or("Error obtaining overlay window".to_string())?;

    let overlay_hwnd = overlay_window
        .hwnd()
        .map_err(|e| format!("Error obtaining overlay window handle: {e}"))?;

    remove_titlebar(overlay_hwnd);
    set_window_topmost(overlay_hwnd)?;

    // Set position to monitor top-left (0,0)
    overlay_window.set_position(PhysicalPosition {
        x: pmonitor.rect.left,
        y: pmonitor.rect.top
    }).map_err(|e| format!("Error setting window position: {e}"))?;

    let size = PhysicalSize {
        height: pmonitor.rect.height() as u32,
        width: pmonitor.rect.width() as u32
    };

    overlay_window.set_max_size(Some(size)).map_err(|e| format!("Error setting monitor max size: {e}"))?;

    // Force overlay window to use full monitor dimensions
    overlay_window.set_size(size).map_err(|e| format!("Error setting monitor size: {e}"))?;

    Ok(())
}

pub fn legacy_show_taskbar(app_handle: &AppHandle) -> Result<(), String> {
    let mut pmonitor = get_primary_monitor()?;
    let tbrect = get_taskbar_rect().ok_or("No taskbar rect".to_string())?;
    let mut workarea = pmonitor.workarea;

    legacy_reset_taskbar()?;

    // Se la workarea arriva fino alla parte bassa dello schermo...
    if workarea.bottom > pmonitor.rect.bottom - tbrect.height() {
        workarea.bottom -= tbrect.height();
    }
    pmonitor.set_workarea(&workarea)?;

    let overlay_window = app_handle.get_window("overlay")
        .ok_or("Error obtaining overlay window".to_string())?;

    let overlay_hwnd = overlay_window
        .hwnd()
        .map_err(|e| format!("Error obtaining overlay window handle: {e}"))?;

    remove_titlebar(overlay_hwnd);
    set_window_topmost(overlay_hwnd)?;

    // Set position to monitor top-left (0,0)
    overlay_window.set_position(PhysicalPosition {
        x: pmonitor.rect.left,
        y: pmonitor.rect.top
    }).map_err(|e| format!("Error setting window position: {e}"))?;

    let size = PhysicalSize {
        height: pmonitor.rect.height() as u32,
        width: pmonitor.rect.width() as u32
    };

    overlay_window.set_max_size(Some(size)).map_err(|e| format!("Error setting monitor max size: {e}"))?;

    // Force overlay window to use full monitor dimensions
    overlay_window.set_size(size).map_err(|e| format!("Error setting monitor size: {e}"))?;

    Ok(())
}