use crossbeam::channel::Sender;
use std::{
    collections::HashMap,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicPtr, Ordering},
    },
    time::{Duration, Instant},
};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GWL_STYLE, GetWindowLongPtrW, WS_CAPTION, WS_THICKFRAME,
};
use windows::Win32::{
    Foundation::*,
    UI::{Accessibility::*, WindowsAndMessaging::*},
};

use super::GlobalEvent;

static WINDOW_HOOK: AtomicPtr<HWINEVENTHOOK> = AtomicPtr::new(std::ptr::null_mut());

pub fn init_hook(tx: Sender<GlobalEvent>) -> Result<(), String> {
    static TX: OnceLock<Sender<GlobalEvent>> = OnceLock::new();
    static WINDOW_STATES: OnceLock<Mutex<HashMap<usize, (bool, Instant)>>> = OnceLock::new();

    TX.set(tx.clone()).unwrap();
    WINDOW_STATES.set(Mutex::new(HashMap::new())).unwrap();

    extern "system" fn win_event_proc(
        _hook: HWINEVENTHOOK,
        _event: u32,
        hwnd: HWND,
        _id_object: i32,
        _id_child: i32,
        _id_event_thread: u32,
        _time: u32,
    ) {
        if hwnd.0 == std::ptr::null_mut() {
            return;
        }

        if let (Some(tx), Some(states)) = (TX.get(), WINDOW_STATES.get()) {
            let mut rect = RECT::default();
            if unsafe { GetWindowRect(hwnd, &mut rect).is_ok() } {
                let screen_w = unsafe { GetSystemMetrics(SM_CXSCREEN) };
                let screen_h = unsafe { GetSystemMetrics(SM_CYSCREEN) };
                let width = rect.right - rect.left;
                let height = rect.bottom - rect.top;

                // Consideriamo fullscreen solo finestre quasi a tutto schermo
                if width < screen_w / 2 || height < screen_h / 2 {
                    return; // ignora finestre piccole o secondarie
                }

                let style = unsafe { GetWindowLongPtrW(hwnd, GWL_STYLE) } as u32;
                let is_borderless = (style & WS_CAPTION.0 == 0) && (style & WS_THICKFRAME.0 == 0);

                let is_fullscreen = is_borderless
                    && rect.left <= 0
                    && rect.top <= 0
                    && width >= screen_w
                    && height >= screen_h;

                let hwnd_key = hwnd.0 as usize;

                let mut states = states.lock().unwrap();
                let (prev_state, last_emit) = states
                    .get(&hwnd_key)
                    .copied()
                    .unwrap_or((false, Instant::now() - Duration::from_secs(10)));

                // invia evento solo se cambia stato e non è troppo vicino al precedente
                if is_fullscreen != prev_state && last_emit.elapsed() > Duration::from_millis(50) {
                    states.insert(hwnd_key, (is_fullscreen, Instant::now()));
                    if is_fullscreen {
                        let _ = tx.send(GlobalEvent::WindowEnteredFullscreen { hwnd: hwnd_key });
                    } else {
                        let _ = tx.send(GlobalEvent::WindowExitedFullscreen { hwnd: hwnd_key });
                    }
                }
            }
        }
    }

    // Hook combinato: foreground + location changes
    let hook = unsafe {
        SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_OBJECT_LOCATIONCHANGE,
            Some(HMODULE::default()),
            Some(win_event_proc),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        )
    };

    let hook_box = Box::into_raw(Box::new(hook));

    WINDOW_HOOK.store(hook_box, Ordering::SeqCst);

    // Nota: il message loop deve partire nel thread che chiama init_hooks
    Ok(())
}

pub fn unmount_hook() -> Result<(), String> {
    let hook_val = WINDOW_HOOK.swap(std::ptr::null_mut(), Ordering::SeqCst);

    if hook_val != std::ptr::null_mut() {
        let hook = HWINEVENTHOOK(hook_val as *mut _);
        unsafe {
            UnhookWinEvent(hook)
                .ok()
                .map_err(|e| format!("Errore durante l'unmount dell'hook: {:?}", e))?;
        }
    }

    Ok(())
}
