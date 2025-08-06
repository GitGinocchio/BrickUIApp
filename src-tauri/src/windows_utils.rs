use windows::{
    core::{Error as WinError, PCSTR},
    Win32::{
        Foundation::{HWND, LPARAM},
        UI::{
            Shell::{SHAppBarMessage, ABM_SETSTATE, ABS_ALWAYSONTOP, ABS_AUTOHIDE, APPBARDATA},
            WindowsAndMessaging::{
                FindWindowA, GetWindowLongA, GetWindowLongPtrW, GetWindowLongW, SetWindowLongA, SetWindowLongPtrW, SetWindowLongW, SetWindowPos, ShowWindow, GWL_EXSTYLE, GWL_STYLE, HWND_TOPMOST, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SW_HIDE, SW_SHOW, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT, WS_POPUP, WS_VISIBLE
            },
        },
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

pub fn make_window_click_through(window: &WebviewWindow) {
    if let Ok(handle) = window.window_handle() {
        match handle.as_raw() {
            RawWindowHandle::Win32(win) => {
                let hwnd = HWND(win.hwnd.get() as isize as _);
                unsafe {
                    let ex_style = GetWindowLongA(hwnd, GWL_EXSTYLE);
                    let new_style = ex_style
                        | (WS_EX_LAYERED.0 as i32)
                        | (WS_EX_TRANSPARENT.0 as i32);
                    SetWindowLongA(hwnd, GWL_EXSTYLE, new_style);
                }
            }
            _ => {
                eprintln!("Unsupported platform or window handle type.");
            }
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
                    
                    // Stile base senza bordi
                    let style = WS_POPUP | WS_VISIBLE;
                    SetWindowLongW(hwnd, GWL_STYLE, style.0 as i32);

                    // Stili estesi per toolwindow + layered + trasparente
                    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                    SetWindowLongW(
                        hwnd,
                        GWL_EXSTYLE,
                        ex_style | WS_EX_TOOLWINDOW.0 as i32 | WS_EX_LAYERED.0 as i32 | WS_EX_TRANSPARENT.0 as i32,
                    );

                    // Aggiorna la finestra (obbligatorio per far vedere le modifiche)
                    let _ = SetWindowPos(hwnd, Some(HWND_TOPMOST), 0, 0, 0, 0,
                        SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED);
                }
            },
            _ => {
                eprintln!("Unsupported platform or window handle type.");
            }
        }
    }
}

pub fn disable_window_focus(window: &WebviewWindow) {
    if let Ok(handle) = window.window_handle() {
        match handle.as_raw() {
            RawWindowHandle::Win32(win) => {
                unsafe {
                    let hwnd = HWND(win.hwnd.get() as isize as _);
                    
                    let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                    SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_NOACTIVATE.0 as isize);
                }
            },
            _ => {
                eprintln!("Unsupported platform or window handle type.");
            }
        }
    }
}