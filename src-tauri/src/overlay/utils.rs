use windows::{
    core::{Error as WinError, PCSTR},
    Win32::{
        Foundation::{HWND, LPARAM, RECT}, UI::{
            Shell::{SHAppBarMessage, ABM_SETSTATE, ABS_ALWAYSONTOP, ABS_AUTOHIDE, APPBARDATA},
            WindowsAndMessaging::{
                FindWindowA, GetWindowLongW, GetWindowRect, SetWindowLongW, SetWindowPos, ShowWindow, 
                GWL_EXSTYLE, GWL_STYLE, HWND_TOPMOST, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOMOVE, 
                SWP_NOSIZE, SWP_NOZORDER, SW_HIDE, SW_SHOW, WS_EX_LAYERED, WS_EX_NOACTIVATE, 
                WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP, WS_VISIBLE
            },
        }
    },
};

use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use tauri::{WebviewWindow};

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

pub fn enable_click_through(window: &WebviewWindow) {
    if let Ok(handle) = window.window_handle() {
        match handle.as_raw() {
            RawWindowHandle::Win32(win) => {
                unsafe {
                    let hwnd = HWND(win.hwnd.get() as isize as _);
                    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                    let new_style = ex_style | WS_EX_TRANSPARENT.0 as i32;
                    SetWindowLongW(hwnd, GWL_EXSTYLE, new_style);

                    force_window_style_refresh(hwnd);
                }
            }
            _ => eprintln!("Unsupported platform or window handle type."),
        }
    } else {
        eprintln!("Failed to get window handle.");
    }
}

pub fn disable_click_through(window: &WebviewWindow) {
    if let Ok(handle) = window.window_handle() {
        match handle.as_raw() {
            RawWindowHandle::Win32(win) => {
                unsafe {
                    let hwnd = HWND(win.hwnd.get() as isize as _);
                    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                    // Rimuove WS_EX_TRANSPARENT usando bitwise AND con NOT
                    let new_style = ex_style & !(WS_EX_TRANSPARENT.0 as i32);
                    SetWindowLongW(hwnd, GWL_EXSTYLE, new_style);

                    force_window_style_refresh(hwnd);
                }
            }
            _ => eprintln!("Unsupported platform or window handle type."),
        }
    } else {
        eprintln!("Failed to get window handle.");
    }
}

pub fn hide_titlebar(window: &WebviewWindow) {
    if let Ok(handle) = window.window_handle() {
        match handle.as_raw() {
            RawWindowHandle::Win32(win) => {
                unsafe {
                    let hwnd = HWND(win.hwnd.get() as isize as _);

                    // Rimuove bordi/titlebar
                    let style = WS_POPUP | WS_VISIBLE;
                    SetWindowLongW(hwnd, GWL_STYLE, style.0 as i32);

                    // Aggiunge WS_EX_TOOLWINDOW | WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_NOACTIVATE
                    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                    let new_ex_style = ex_style
                        | WS_EX_TOOLWINDOW.0 as i32
                        | WS_EX_LAYERED.0 as i32
                        | WS_EX_TOPMOST.0 as i32
                        | WS_EX_NOACTIVATE.0 as i32;

                    SetWindowLongW(hwnd, GWL_EXSTYLE, new_ex_style);

                    force_window_style_refresh(hwnd);
                }
            }
            _ => eprintln!("Unsupported platform or window handle type."),
        }
    } else {
        eprintln!("Failed to get window handle.");
    }
}
