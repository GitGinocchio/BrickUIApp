use std::{collections::HashSet, sync::mpsc::{Receiver, Sender, channel}, thread, time::Instant};

use tokio::sync::RwLockReadGuard;
use windows::{Win32::{
    Foundation::{COLORREF, GetLastError, HINSTANCE, HWND, LPARAM, LRESULT, POINT, SIZE, WPARAM}, Graphics::Gdi::{AC_SRC_ALPHA, AC_SRC_OVER, BI_RGB, BITMAPINFO, BITMAPINFOHEADER, BLENDFUNCTION, ClientToScreen, CreateCompatibleDC, CreateDIBSection, DIB_RGB_COLORS, DeleteDC, DeleteObject, GetDC, HGDIOBJ, ReleaseDC, ScreenToClient, SelectObject}, System::LibraryLoader::GetModuleHandleW, UI::{Input::KeyboardAndMouse::{EnableWindow, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEINPUT, SendInput}, Shell::DefSubclassProc, WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, DispatchMessageW, GW_HWNDNEXT, GWL_EXSTYLE, GetWindow, GetWindowLongPtrW, HTCLIENT, HTTRANSPARENT, HWND_BOTTOM, HWND_TOP, HWND_TOPMOST, LWA_ALPHA, LWA_COLORKEY, MSG, PM_REMOVE, PeekMessageW, PostMessageW, PostQuitMessage, RegisterClassW, SW_SHOW, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SetLayeredWindowAttributes, SetWindowLongPtrW, SetWindowPos, ShowWindow, TranslateMessage, ULW_ALPHA, UpdateLayeredWindow, WINDOW_STYLE, WM_DESTROY, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_NCHITTEST, WM_QUIT, WNDCLASSW, WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT, WS_OVERLAPPED, WS_POPUP, WS_POPUPWINDOW, WS_VISIBLE, WindowFromPoint
    }}
}, core::{PCWSTR, w}};

use crate::winapi::events::hitboxes::{ClickableRect, HITBOXES, get_x_lparam, get_y_lparam, is_point_clickable};

unsafe fn window_below_overlay(hwnd_overlay: HWND, pt: POINT) -> Option<HWND> {
    let mut hwnd = unsafe { WindowFromPoint(pt) };

    while hwnd == hwnd_overlay {
        hwnd = unsafe { GetWindow(hwnd, GW_HWNDNEXT).ok()? };
        if hwnd.is_invalid() {
            return None;
        }
    }

    Some(hwnd)
}

unsafe fn old_forward_click(hwnd: HWND) {
    let inputs = [
        INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: 0,
                    dy: 0,
                    mouseData: 0,
                    dwFlags: MOUSEEVENTF_LEFTDOWN,
                    time: 0,
                    dwExtraInfo: 0,
                }
            },
        },
        INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: 0,
                    dy: 0,
                    mouseData: 0,
                    dwFlags: MOUSEEVENTF_LEFTUP,
                    time: 0,
                    dwExtraInfo: 0,
                }
            },
        }
    ];

    unsafe {
        let _ = SetWindowPos(
            hwnd,
            Some(HWND_BOTTOM),
            0, 0,
            0, 0,
            SWP_NOMOVE | SWP_NOSIZE
        );
    }

    unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32); }
    
    unsafe {
        let _ = SetWindowPos(
            hwnd,
            Some(HWND_TOPMOST),
            0, 0,
            0, 0,
            SWP_NOMOVE | SWP_NOSIZE
        );
    }
}

fn forward_click(hwnd_overlay: HWND, point: POINT) -> Result<(), String> {
    unsafe {
        let target = window_below_overlay(hwnd_overlay, point);

        if let Some(hwnd) = target {
            let mut client_point = point;
            ScreenToClient(hwnd, &mut client_point)
                .ok()
                .map_err(|e| format!("ScreenToClient Error: {e}"))?;

            let lparam = LPARAM(
                ((client_point.y as u32) << 16 | (client_point.x as u32 & 0xFFFF)) as isize
            );

            PostMessageW(Some(hwnd), WM_LBUTTONDOWN, WPARAM(0), lparam)
                .map_err(|e| format!("PostMessageW Error: {e}"))?;
            PostMessageW(Some(hwnd), WM_LBUTTONUP, WPARAM(0), lparam)
                .map_err(|e| format!("PostMessageW Error: {e}"))?;
        }

        Ok(())
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM
) -> LRESULT {
    match msg {
        WM_NCHITTEST => {
            let screen_x = get_x_lparam(lparam.0);
            let screen_y = get_y_lparam(lparam.0);

            let mut point = POINT { x: screen_x, y: screen_y };
            unsafe { ScreenToClient(hwnd, &mut point).expect("Error converting screen to client point") };

            let x = point.x as i64;
            let y = point.y as i64;

            if is_point_clickable(x, y) {
                println!("Il punto ({x}, {y}) e' cliccabile!");
                return LRESULT(HTCLIENT as isize);
            }

            println!("Il punto ({x}, {y}) NON e' cliccabile!");

            return LRESULT(HTTRANSPARENT as isize);
        },
        WM_LBUTTONDOWN => {
            println!("Tasto sinistro premuto!");
            let client_x = get_x_lparam(lparam.0);
            let client_y = get_y_lparam(lparam.0);

            let mut point = POINT { x: client_x, y: client_y };
            unsafe { ClientToScreen(hwnd, &mut point).ok().expect("ClientToScreen error") };

            forward_click(hwnd, point).expect("Error forwarding click");
            return LRESULT(0);
        },
        WM_LBUTTONUP => {
            return LRESULT(0);
        },
        WM_DESTROY => {
            println!("Finestra chiusa!");
            unsafe { PostQuitMessage(0) };
            return LRESULT(0)
        }
        event => {
            println!("event: {event}");
            unsafe { return DefWindowProcW(hwnd, msg, wparam, lparam); }
        }
    }
}

pub enum InputOverlayCommand {
    Close
}

#[derive(Debug)]
pub struct InputOverlay {
    thread: thread::JoinHandle<()>,
    tx: Sender<InputOverlayCommand>,
    hwnd: isize
}

impl InputOverlay {
    pub fn new(parent: Option<isize>) -> Result<Self, String> {
        let (cmd_tx, cmd_rx) = channel();
        let (res_tx, res_rx) = channel::<Result<isize, String>>();

        let thread = thread::spawn(move || { InputOverlay::spawn_input_overlay(res_tx, cmd_rx, parent) });

        println!("prima");
        let hwnd = res_rx.recv().map_err(|_| "Overlay thread died".to_string())??;
        println!("dopo");

        Ok(Self { 
            thread, 
            tx: cmd_tx,
            hwnd
        })
    }

    fn spawn_input_overlay(
        res_tx: Sender<Result<isize, String>>, 
        cmd_rx: Receiver<InputOverlayCommand>,
        parent: Option<isize>
    ) {
        unsafe {
            let mut last_toggle = Instant::now();
            let mut clickable = true;
            let clickable_ms: u64 = 10;
            let non_clickable_ms: u64 = 10;

            let hinstance = match GetModuleHandleW(None) {
                Ok(hmodule) => HINSTANCE(hmodule.0),
                Err(_) => {
                    if let Err(e) = res_tx.send(Err(format!("GetModuleHandleW failed: {:?}", GetLastError()))) {
                        eprintln!("Error sending response: {e}");
                    }
                    return;
                }
            };

            let wc = WNDCLASSW {
                lpfnWndProc: Some(wnd_proc),
                hInstance: hinstance,
                lpszClassName: w!("InputOverlayClass"),
                ..Default::default()
            };

            RegisterClassW(&wc);

            let hwnd = match CreateWindowExW(
                WS_EX_LAYERED | WS_EX_TOOLWINDOW,
                w!("InputOverlayClass"),
                w!("BrickUI-InputOverlay"),
                WS_POPUP | WS_VISIBLE,
                0,
                0,
                1920,
                1080,
                parent.map(|p| HWND(p as *mut _)),
                None,
                Some(hinstance),
                None,
            ) {
                Ok(hwnd) => hwnd,
                Err(_) => {
                    if let Err(e) = res_tx.send(Err(format!("CreateWindowExW failed: {:?}", GetLastError()))) {
                        eprintln!("Error sending response: {e}");
                    }
                    return;
                }
            };

            let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), 1, LWA_ALPHA).ok();

            /*
                if let Err(e) = res_tx.send(Err(format!("SetLayeredWindowAttributes failed: {:?}, {e}", GetLastError()))) {
                    eprintln!("Error sending response: {e}");
                }
                return;
            }
            */

            /*
            let res = ShowWindow(hwnd, SW_SHOW);
            if res.0 != 0 && let Err(e) = res_tx.send(Err(format!("ShowWindow failed: {:?}", GetLastError()))) {
                eprintln!("Error sending response: {e}");
                return;
            }
            */

            let _ = SetWindowPos(
                hwnd,
                Some(HWND_TOPMOST),  // rende la finestra topmost
                0, 0,
                0, 0,
                SWP_NOMOVE | SWP_NOSIZE
            );

            let mut msg = MSG::default();

            if let Err(e) = res_tx.send(Ok(hwnd.0 as isize)) {
                eprintln!("Error sending message: {e}");
            }

            loop {
                // Controllo del tempo trascorso per cambiare stato
                let elapsed = last_toggle.elapsed().as_millis() as u64;
                if clickable && elapsed >= clickable_ms {
                    // Passa a non cliccabile
                    let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                    SetWindowLongPtrW(hwnd, GWL_EXSTYLE, (WS_EX_LAYERED.0 | WS_EX_TOOLWINDOW.0 | WS_EX_TRANSPARENT.0) as isize);
                    clickable = false;
                    last_toggle = Instant::now();
                } else if !clickable && elapsed >= non_clickable_ms {
                    // Passa a cliccabile
                    let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                    SetWindowLongPtrW(hwnd, GWL_EXSTYLE, (WS_EX_LAYERED.0 | WS_EX_TOOLWINDOW.0) as isize);
                    clickable = true;
                    last_toggle = Instant::now();
                }

                while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() {
                    if msg.message == WM_QUIT {
                        return;
                    }

                    println!("Window message: {}", msg.message);

                    let _ = TranslateMessage(&msg);
                    let _ = DispatchMessageW(&msg);
                    
                }

                if let Ok(cmd) = cmd_rx.try_recv() {
                    match cmd {
                        InputOverlayCommand::Close => {
                            PostQuitMessage(0);
                        }
                    }
                }
            }
        }
    }
}