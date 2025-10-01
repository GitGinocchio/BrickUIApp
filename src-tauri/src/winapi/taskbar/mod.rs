pub mod apps;

use windows::{
    core::{Error as WinError, PCSTR}, Win32::{
        Foundation::LPARAM,
        UI::{
            Shell::{SHAppBarMessage, ABM_GETSTATE, ABM_GETTASKBARPOS, ABM_SETSTATE, ABS_ALWAYSONTOP, ABS_AUTOHIDE, APPBARDATA},
            WindowsAndMessaging::{
                FindWindowA, SetWindowPos, ShowWindow, HWND_BOTTOM, HWND_TOPMOST, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SW_HIDE, SW_SHOW
            },
        },
    }
};

use crate::winapi::Rect;

pub fn get_taskbar_rect() -> Option<Rect> {
    let mut data = APPBARDATA {
        cbSize: std::mem::size_of::<APPBARDATA>() as u32,
        ..Default::default()
    };
    let res = unsafe { SHAppBarMessage(ABM_GETTASKBARPOS, &mut data) };
    if res != 0 { Some(data.rc.into()) } else { None }
}

pub fn hide_taskbar(keep_taskbar_space: bool) -> Result<(), String> {
    unsafe {
        let class_name = b"Shell_TrayWnd\0".as_ptr();
        let taskbar = FindWindowA(PCSTR(class_name), PCSTR::null()).map_err(|e| e.to_string())?;

        if taskbar.0.is_null() {
            return Err(WinError::from_win32().to_string());
        }

        let _ = ShowWindow(taskbar, SW_HIDE);

        if keep_taskbar_space {
            return Ok(());
        }

        let mut abd = APPBARDATA {
            cbSize: std::mem::size_of::<APPBARDATA>() as u32,
            hWnd: taskbar,
            uEdge: 0,
            rc: Default::default(),
            lParam: LPARAM(ABS_AUTOHIDE as isize),
            uCallbackMessage: 0,
        };

        let _ = SHAppBarMessage(ABM_SETSTATE, &mut abd);

        SetWindowPos(
            taskbar,
            Some(HWND_BOTTOM),
            0,
            -100,
            0,
            0,
            SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE,
        )
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}

pub fn show_taskbar() -> Result<(), String> {
    unsafe {
        let class_name = b"Shell_TrayWnd\0".as_ptr();
        let taskbar = FindWindowA(PCSTR(class_name), PCSTR::null()).map_err(|e| e.to_string())?;

        if taskbar.0.is_null() {
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