pub mod apps;

use windows::{
    core::{Error as WinError, PCSTR},
    Win32::{
        Foundation::{HWND, LPARAM, RECT}, UI::{
            Shell::{SHAppBarMessage, ABM_SETSTATE, ABS_ALWAYSONTOP, ABS_AUTOHIDE, APPBARDATA},
            WindowsAndMessaging::{
                FindWindowA,  GetWindowRect, SetWindowPos, ShowWindow, HWND_BOTTOM, HWND_TOPMOST, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SW_HIDE, SW_SHOW
            },
        }
    },
};

pub fn hide_taskbar(keep_taskbar_space: bool) -> Result<(), String> {
    unsafe {
        let class_name = b"Shell_TrayWnd\0".as_ptr();
        let taskbar = FindWindowA(PCSTR(class_name), PCSTR::null())
            .map_err(|e| e.to_string())?;

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
        let taskbar = FindWindowA(PCSTR(class_name), PCSTR::null())
            .map_err(|e| e.to_string())?;

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
