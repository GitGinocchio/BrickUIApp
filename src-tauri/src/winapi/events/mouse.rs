use super::GlobalEvent;
use crossbeam::channel::Sender;
use std::sync::{
    OnceLock,
    atomic::{AtomicPtr, Ordering},
};
use windows::Win32::{Foundation::*, UI::WindowsAndMessaging::*};

static MOUSE_HOOK: AtomicPtr<HHOOK> = AtomicPtr::new(std::ptr::null_mut());

fn hiword(val: u32) -> u16 {
    (val >> 16) as u16
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn init_hook(tx: Sender<GlobalEvent>) -> Result<(), String> {
    static TX: OnceLock<Sender<GlobalEvent>> = OnceLock::new();

    #[cfg_attr(feature = "profiling", tracing::instrument)]
    extern "system" fn mouse_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        if n_code >= 0 {
            let ms = unsafe { &*(l_param.0 as *const MSLLHOOKSTRUCT) };
            if let Some(tx) = TX.get() {
                match w_param.0 as u32 {
                    WM_MOUSEMOVE => {
                        let _ = tx.send(GlobalEvent::MouseMove {
                            x: ms.pt.x,
                            y: ms.pt.y,
                        });
                    }
                    WM_LBUTTONDOWN => {
                        let _ = tx.send(GlobalEvent::MouseButtonDown {
                            button: "Left".into(),
                            x: ms.pt.x,
                            y: ms.pt.y,
                        });
                    }
                    WM_LBUTTONUP => {
                        let _ = tx.send(GlobalEvent::MouseButtonUp {
                            button: "Left".into(),
                            x: ms.pt.x,
                            y: ms.pt.y,
                        });
                    }
                    WM_RBUTTONDOWN => {
                        let _ = tx.send(GlobalEvent::MouseButtonDown {
                            button: "Right".into(),
                            x: ms.pt.x,
                            y: ms.pt.y,
                        });
                    }
                    WM_RBUTTONUP => {
                        let _ = tx.send(GlobalEvent::MouseButtonUp {
                            button: "Right".into(),
                            x: ms.pt.x,
                            y: ms.pt.y,
                        });
                    }
                    WM_MBUTTONDOWN => {
                        let _ = tx.send(GlobalEvent::MouseButtonDown {
                            button: "Middle".into(),
                            x: ms.pt.x,
                            y: ms.pt.y,
                        });
                    }
                    WM_MBUTTONUP => {
                        let _ = tx.send(GlobalEvent::MouseButtonUp {
                            button: "Middle".into(),
                            x: ms.pt.x,
                            y: ms.pt.y,
                        });
                    }
                    WM_XBUTTONDOWN => {
                        let button = if hiword(ms.mouseData) == 1 {
                            "XButton1"
                        } else {
                            "XButton2"
                        };
                        let _ = tx.send(GlobalEvent::MouseButtonDown {
                            button: button.into(),
                            x: ms.pt.x,
                            y: ms.pt.y,
                        });
                    }
                    WM_XBUTTONUP => {
                        let button = if hiword(ms.mouseData) == 1 {
                            "XButton1"
                        } else {
                            "XButton2"
                        };
                        let _ = tx.send(GlobalEvent::MouseButtonUp {
                            button: button.into(),
                            x: ms.pt.x,
                            y: ms.pt.y,
                        });
                    }
                    WM_MOUSEWHEEL => {
                        let delta = (hiword(ms.mouseData) as i16) - 120;
                        let notches = delta.signum();
                        let _ = tx.send(GlobalEvent::MouseWheel {
                            notches: notches,
                            x: ms.pt.x,
                            y: ms.pt.y,
                        });
                    }
                    _ => {}
                }
            }
        }
        unsafe { CallNextHookEx(Some(HHOOK::default()), n_code, w_param, l_param) }
    }

    TX.set(tx.clone()).unwrap();

    let hook =
        unsafe { SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_proc), Some(HINSTANCE::default()), 0) }
            .map_err(|e| format!("Errore durante la creazione dell'hook: {e}"))?;

    let hook_box = Box::into_raw(Box::new(hook));

    MOUSE_HOOK.store(hook_box, Ordering::SeqCst);

    Ok(())
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn unmount_hook() -> Result<(), String> {
    let hook = MOUSE_HOOK.swap(std::ptr::null_mut(), Ordering::SeqCst);
    if !hook.is_null() {
        unsafe {
            UnhookWindowsHookEx(*Box::from_raw(hook))
                .map_err(|e| format!("Errore durenate l'unmount dell'hook: {e}"))?
        };
    }

    Ok(())
}
