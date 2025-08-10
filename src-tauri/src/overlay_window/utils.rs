use windows::{
    core::{Error as WinError, PCSTR},
    Win32::{
        Foundation::{HWND, LPARAM, RECT}, UI::{
            Shell::{SHAppBarMessage, ABM_SETSTATE, ABS_ALWAYSONTOP, ABS_AUTOHIDE, APPBARDATA},
            WindowsAndMessaging::{
                FindWindowA, GetWindowLongPtrW, GetWindowRect, SetWindowLongPtrW, 
                SetWindowPos, ShowWindow, GWL_STYLE, HWND_TOPMOST, SWP_FRAMECHANGED, SWP_NOACTIVATE, 
                SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SW_HIDE, SW_SHOW, WS_CAPTION, 
                WS_THICKFRAME
            },
        }
    },
};

pub fn hide_taskbar(keep_taskbar_space: bool) -> Result<(), WinError> {
    unsafe {
        let class_name = b"Shell_TrayWnd\0".as_ptr();
        let taskbar = FindWindowA(PCSTR(class_name), PCSTR::null())?;

        if taskbar.0.is_null() {
            return Err(WinError::from_win32());
        }

        let _ = ShowWindow(taskbar, SW_HIDE);

        if keep_taskbar_space {
            return Ok(())
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
            Some(HWND_TOPMOST),
            0,
            -100,
            0,
            0,
            SWP_NOSIZE | SWP_NOMOVE,
        )?;

        Ok(())
    }
}

pub fn show_taskbar() -> Result<(), WinError> {
    unsafe {
        let class_name = b"Shell_TrayWnd\0".as_ptr();
        let taskbar = FindWindowA(PCSTR(class_name), PCSTR::null())?;

        if taskbar.0.is_null() {
            return Err(WinError::from_win32());
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
        )?;

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

pub fn remove_titlebar(window: &tauri::Window) {
    unsafe {
        let hwnd = window.hwnd().unwrap() as HWND;
        let style = GetWindowLongPtrW(hwnd, GWL_STYLE);
        let new_style = style & !(WS_CAPTION.0 as isize) & !(WS_THICKFRAME.0 as isize);

        SetWindowLongPtrW(hwnd, GWL_STYLE, new_style);
        force_window_style_refresh(hwnd); // forza il redraw senza titlebar
    }
}