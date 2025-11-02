use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicPtr, Ordering},
};
use tokio::sync::Mutex;

use crossbeam::channel::Sender;
use tauri::{AppHandle, Manager};
use windows::Win32::{
    Foundation::*,
    UI::{
        Input::KeyboardAndMouse::{GetAsyncKeyState, VK_CONTROL, VK_ESCAPE, VK_LWIN, VK_RWIN},
        WindowsAndMessaging::*,
    },
};

use crate::{
    config::settings::{Settings, StartMenuBehavior},
    state::BrickUIState,
};

use super::GlobalEvent;

static KEYBOARD_HOOK: AtomicPtr<HHOOK> = AtomicPtr::new(std::ptr::null_mut());

pub async fn init_hook<R: tauri::Runtime>(
    tx: Sender<GlobalEvent>,
    app_handle: &AppHandle<R>,
) -> Result<(), String> {
    static TX: OnceLock<Sender<GlobalEvent>> = OnceLock::new();

    let state = app_handle.state::<Arc<Mutex<BrickUIState>>>();
    let state_guard = state
        .lock()
        .await;
    static SETTINGS: OnceLock<Settings> = OnceLock::new();
    SETTINGS
        .set(state_guard.get_settings().clone())
        .map_err(|e| format!("Errore durante oncelock su settings: {e:?}"))?;

    extern "system" fn keyboard_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        if n_code >= 0 {
            let kb = unsafe { &*(l_param.0 as *const KBDLLHOOKSTRUCT) };

            if let Some(tx) = TX.get() {
                let key_name = vk_to_string(kb.vkCode);

                match w_param.0 as u32 {
                    WM_KEYDOWN => {
                        let _ = tx.send(GlobalEvent::KeyDown(key_name));
                    }
                    WM_KEYUP => {
                        let _ = tx.send(GlobalEvent::KeyUp(key_name));
                    }
                    _ => (),
                }
            }

            let behavior = if let Some(settings) = SETTINGS.get() {
                settings.startmenu.behavior.clone()
            } else {
                return unsafe { CallNextHookEx(None, n_code, w_param, l_param) };
            };

            // Blocca tasto Windows sinistro/destra
            if (behavior == StartMenuBehavior::DisableWin
                || behavior == StartMenuBehavior::DisableBoth)
                && (kb.vkCode == VK_LWIN.0 as u32 || kb.vkCode == VK_RWIN.0 as u32)
            {
                if w_param.0 as u32 == WM_KEYDOWN {
                    return LRESULT(1); // intercetta l'apertura
                } else {
                    // per keyup lascia passare il messaggio
                    return unsafe { CallNextHookEx(None, n_code, w_param, l_param) };
                }
            }

            // Blocca Ctrl+Esc
            if (behavior == StartMenuBehavior::DisableCtrlEsc
                || behavior == StartMenuBehavior::DisableBoth)
                && (kb.vkCode == VK_ESCAPE.0 as u32)
                && unsafe { (GetAsyncKeyState(VK_CONTROL.0 as i32) & 0x8000u16 as i16) != 0 }
            {
                if w_param.0 as u32 == WM_KEYDOWN {
                    return LRESULT(1); // intercetta l'apertura
                } else {
                    // per keyup lascia passare il messaggio
                    return unsafe { CallNextHookEx(None, n_code, w_param, l_param) };
                }
            }
        }
        unsafe { CallNextHookEx(Some(HHOOK::default()), n_code, w_param, l_param) }
    }

    TX.set(tx.clone()).unwrap();

    let hook = unsafe {
        SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(keyboard_proc),
            Some(HINSTANCE::default()),
            0,
        )
    }
    .map_err(|e| format!("Errore durante la creazione dell'hook: {e}"))?;

    let hook_box = Box::into_raw(Box::new(hook));

    KEYBOARD_HOOK.store(hook_box, Ordering::SeqCst);

    // Nota: il message loop deve partire nel thread che chiama init_hooks

    Ok(())
}

pub fn unmount_hook() -> Result<(), String> {
    let hook = KEYBOARD_HOOK.swap(std::ptr::null_mut(), Ordering::SeqCst);
    if !hook.is_null() {
        unsafe {
            UnhookWindowsHookEx(*Box::from_raw(hook))
                .map_err(|e| format!("Errore durenate l'unmount dell'hook: {e}"))?
        };
    }

    Ok(())
}

// Mancano ancora dei tasti da mappare
/// Converte un Virtual-Key code in una stringa leggibile
pub fn vk_to_string(vk: u32) -> String {
    match vk {
        // Controllo base
        0x08 => "Backspace".into(),
        0x09 => "Tab".into(),
        0x0C => "Clear".into(),
        0x0D => "Enter".into(),
        0x10 => "Shift".into(),
        0x11 => "Ctrl".into(),
        0x12 => "Alt".into(),
        0x13 => "Pause".into(),
        0x14 => "CapsLock".into(),
        0x1B => "Esc".into(),
        0x20 => "Space".into(),
        0x21 => "PageUp".into(),
        0x22 => "PageDown".into(),
        0x23 => "End".into(),
        0x24 => "Home".into(),
        0x25 => "Left".into(),
        0x26 => "Up".into(),
        0x27 => "Right".into(),
        0x28 => "Down".into(),
        0x2C => "PrintScreen".into(),
        0x2D => "Insert".into(),
        0x2E => "Delete".into(),
        0x2A => "Multiply".into(),
        0x6A => "*".into(),
        0x6B => "+".into(),
        0x6C => "Separator".into(),
        0x6D => "-".into(),
        0x6E => ".".into(),
        0x6F => "/".into(),
        0x5B => "LWin".into(),
        0x5C => "RWin".into(),
        0x5D => "Apps".into(),
        0x5F => "Sleep".into(),

        // Numeri 0-9
        0x30..=0x39 => (((vk - 0x30) as u8 + b'0') as char).to_string(),
        // Lettere A-Z
        0x41..=0x5A => (((vk - 0x41) as u8 + b'A') as char).to_string(),
        // Tasti funzione F1-F24
        0x70..=0x87 => format!("F{}", vk - 0x6F),

        // Modificatori
        0xA0 => "LShift".into(),
        0xA1 => "RShift".into(),
        0xA2 => "LCtrl".into(),
        0xA3 => "RCtrl".into(),
        0xA4 => "LAlt".into(),
        0xA5 => "RAlt".into(),

        // Blocchi
        0x90 => "NumLock".into(),
        0x91 => "ScrollLock".into(),

        // Numpad numerici
        0x60..=0x69 => format!("Num{}", vk - 0x60),

        // Tasti Browser
        0xA6 => "BrowserBack".into(),
        0xA7 => "BrowserForward".into(),
        0xA8 => "BrowserRefresh".into(),
        0xA9 => "BrowserStop".into(),
        0xAA => "BrowserSearch".into(),
        0xAB => "BrowserFavorites".into(),
        0xAC => "BrowserHome".into(),

        // Tasti Media
        0xB0 => "VolumeMute".into(),
        0xB1 => "VolumeDown".into(),
        0xB2 => "VolumeUp".into(),
        0xB3 => "MediaNextTrack".into(),
        0xB4 => "MediaPrevTrack".into(),
        0xB5 => "MediaStop".into(),
        0xB6 => "MediaPlayPause".into(),

        // Tasti speciali aggiuntivi
        0xBA => ";".into(),  // OEM_1
        0xBB => "=".into(),  // OEM_PLUS
        0xBC => ",".into(),  // OEM_COMMA
        0xBD => "-".into(),  // OEM_MINUS
        0xBE => ".".into(),  // OEM_PERIOD
        0xBF => "/".into(),  // OEM_2
        0xC0 => "`".into(),  // OEM_3
        0xDB => "[".into(),  // OEM_4
        0xDC => "\\".into(), // OEM_5
        0xDD => "]".into(),  // OEM_6
        0xDE => "'".into(),  // OEM_7
        0xDF => "OEM_8".into(),
        0xE2 => "OEM_102".into(),
        0xE5 => "OEM_CLEAR".into(),

        _ => format!("VK_{vk}"),
    }
}
