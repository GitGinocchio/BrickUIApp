use once_cell::sync::Lazy;
use windows::Win32::Graphics::Gdi::ScreenToClient;
use windows::Win32::UI::Shell::{RemoveWindowSubclass, SetWindowSubclass};
use windows::Win32::UI::{Shell::DefSubclassProc, WindowsAndMessaging::*};
use windows::Win32::Foundation::*;
use windows::core::BOOL;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClickableRect {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
    pub top: i64,
    pub right: i64,
    pub bottom: i64,
    pub left: i64,

    pub border_radius_top_left: i64,
    pub border_radius_top_right: i64,
    pub border_radius_bottom_left: i64,
    pub border_radius_bottom_right: i64,
}

/// Stato globale dei rettangoli cliccabili
pub static HITBOXES: Lazy<Arc<RwLock<HashSet<ClickableRect>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(HashSet::new()))
});

/// Per tenere traccia delle finestre su cui abbiamo installato l'hook
static HOOKED_WINDOWS: Lazy<Mutex<HashSet<usize>>> = Lazy::new(|| Mutex::new(HashSet::new()));

/// Controlla se un punto è cliccabile
pub fn is_point_clickable(x: i64, y: i64) -> bool {
    let hitboxes = HITBOXES.clone();
    let lock = hitboxes.try_read().expect("Lock poisoned");
    lock.iter().any(|rect| x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom)
}

/// Estrae la coordinata X da LPARAM
#[inline]
pub fn get_x_lparam(lparam: isize) -> i32 {
    (lparam & 0xFFFF) as i16 as i32
}

/// Estrae la coordinata Y da LPARAM
#[inline]
pub fn get_y_lparam(lparam: isize) -> i32 {
    ((lparam >> 16) & 0xFFFF) as i16 as i32
}

/// La finestra hookata
unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    _u_subclass: usize,
    _dw_ref_data: usize,
) -> LRESULT {
    match msg {
        WM_NCHITTEST => {
            println!("evento corretto!");

            let screen_x = get_x_lparam(lparam.0);
            let screen_y = get_y_lparam(lparam.0);

            let mut point = POINT { x: screen_x, y: screen_y };
            unsafe { ScreenToClient(hwnd, &mut point).expect("Error converting screen to client point") };

            let x = point.x as i64;
            let y = point.y as i64;

            if is_point_clickable(x, y) {
                return LRESULT(HTCLIENT as isize);
            }

            //return LRESULT(HTTRANSPARENT as isize);
        }
        event => {
            println!("event: {event}")
        }
    }

    unsafe { DefSubclassProc(hwnd, msg, wparam, lparam) }
}

/// Installa l'hook su una finestra singola
pub fn install_hitbox_hook(hwnd: HWND) -> Result<(), String> {
    let hwnd_val = hwnd.0 as usize;
    let mut hooked = HOOKED_WINDOWS.lock().unwrap();
    if hooked.contains(&hwnd_val) {
        return Err("Hook già installato su questa finestra".into());
    }

    let res = unsafe {
        SetWindowSubclass(
            hwnd,
            Some(wnd_proc),
            1, // idSubclass
            0, // dwRefData
        )
    };

    if res.as_bool() {
        hooked.insert(hwnd_val);
        Ok(())
    } else {
        Err("Fallita l'installazione del subclass hook".into())
    }
}

/// Rimuove l'hook da una finestra
pub fn remove_hitbox_hook(hwnd: HWND) -> Result<(), String> {
    let hwnd_val = hwnd.0 as usize;
    let mut hooked = HOOKED_WINDOWS.lock().unwrap();
    if !hooked.contains(&hwnd_val) {
        return Err("Hook non installato su questa finestra".into());
    }

    let res = unsafe { RemoveWindowSubclass(hwnd, Some(wnd_proc), 1) };
    if res.as_bool() {
        hooked.remove(&hwnd_val);
        Ok(())
    } else {
        Err("Fallita la rimozione del subclass hook".into())
    }
}

/// Installa l'hook su tutte le finestre figlie di una root
pub fn install_hook_recursive(root_hwnd: HWND) -> Result<(), String> {
    unsafe extern "system" fn enum_child_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let res = install_hitbox_hook(hwnd);
        if let Err(e) = &res {
            eprintln!("Failed to hook window {hwnd:?}: {e}");
        }

        unsafe { EnumChildWindows(Some(hwnd), Some(enum_child_proc), LPARAM(0)) };
        BOOL(1)
    }

    unsafe { EnumChildWindows(Some(root_hwnd), Some(enum_child_proc), LPARAM(0)) };
    install_hitbox_hook(root_hwnd)?;
    Ok(())
}
