use std::sync::{atomic::{AtomicPtr, Ordering}, OnceLock};
use crossbeam::channel::Sender;
use windows::Win32::{
    Foundation::*,
    UI::WindowsAndMessaging::*,
};
use super::GlobalEvent;

static MOUSE_HOOK: AtomicPtr<HHOOK> = AtomicPtr::new(std::ptr::null_mut());

fn hiword(val: u32) -> u16 {
    (val >> 16) as u16
}

pub fn init_hook(tx: Sender<GlobalEvent>) -> Result<(), String> {
    static TX: OnceLock<Sender<GlobalEvent>> = OnceLock::new();

    extern "system" fn mouse_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        if n_code >= 0 {
            let ms = unsafe { &*(l_param.0 as *const MSLLHOOKSTRUCT) };
            if let Some(tx) = TX.get() {
                match w_param.0 as u32 {
                    WM_MOUSEMOVE => {
                        let _ = tx.send(GlobalEvent::MouseMove { x: ms.pt.x, y: ms.pt.y });
                    }
                    WM_LBUTTONDOWN => {
                        let _ = tx.send(GlobalEvent::MouseButtonDown("Left".into()));
                    }
                    WM_LBUTTONUP => {
                        let _ = tx.send(GlobalEvent::MouseButtonUp("Left".into()));
                    }
                    WM_RBUTTONDOWN => {
                        let _ = tx.send(GlobalEvent::MouseButtonDown("Right".into()));
                    }
                    WM_RBUTTONUP => {
                        let _ = tx.send(GlobalEvent::MouseButtonUp("Right".into()));
                    }
                    WM_MBUTTONDOWN => {
                        let _ = tx.send(GlobalEvent::MouseButtonDown("Middle".into()));
                    }
                    WM_MBUTTONUP => {
                        let _ = tx.send(GlobalEvent::MouseButtonUp("Middle".into()));
                    }
                    WM_XBUTTONDOWN => {
                        let button = if hiword(ms.mouseData) == 1 { "XButton1" } else { "XButton2" };
                        let _ = tx.send(GlobalEvent::MouseButtonDown(button.into()));
                    }
                    WM_XBUTTONUP => {
                        let button = if hiword(ms.mouseData) == 1 { "XButton1" } else { "XButton2" };
                        let _ = tx.send(GlobalEvent::MouseButtonUp(button.into()));
                    }
                    WM_MOUSEWHEEL => {
                        let delta = (hiword(ms.mouseData) as i16) - 120;
                        let notches = delta.signum();
                        let _ = tx.send(GlobalEvent::MouseWheel(notches));
                    }
                    _ => {}
                }
            }
        }
        unsafe { CallNextHookEx(Some(HHOOK::default()), n_code, w_param, l_param) }
    }

    TX.set(tx.clone()).unwrap();

    let hook = unsafe { SetWindowsHookExW(
        WH_MOUSE_LL, 
        Some(mouse_proc), 
        Some(HINSTANCE::default()), 
        0
    ) }.map_err(|e| format!("Errore durante la creazione dell'hook: {e}"))?;

    let hook_box = Box::into_raw(Box::new(hook));
 
    MOUSE_HOOK.store(hook_box, Ordering::SeqCst);

    Ok(())
}

pub fn unmount_hook() -> Result<(), String> {
    let hook = MOUSE_HOOK.swap(std::ptr::null_mut(), Ordering::SeqCst);
    if !hook.is_null() {
        unsafe { UnhookWindowsHookEx(*Box::from_raw(hook))
            .map_err(|e| format!("Errore durenate l'unmount dell'hook: {e}"))?
        };
    }

    Ok(())
}