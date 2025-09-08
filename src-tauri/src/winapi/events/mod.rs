mod keyboard;
mod mouse;
mod window;

use crate::{
    config::settings::TaskBarBehavior,
    state::BrickUIState,
    winapi::{set_snap_flyout, taskbar::hide_taskbar},
};

use crossbeam::channel;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use tauri::{AppHandle, Emitter, Manager};

#[derive(Debug, Clone)]
pub enum GlobalEvent {
    MouseMove { x: i32, y: i32 },
    MouseButtonDown(String),
    MouseButtonUp(String),
    MouseWheel(i16),

    KeyDown(String),
    KeyUp(String),

    WindowEnteredFullscreen { hwnd: usize },
    WindowExitedFullscreen { hwnd: usize },
}

pub fn start_event_listeners<R: tauri::Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    let (tx, rx) = channel::unbounded();

    let app_handle_copy = app_handle.clone();
    // Thread unico che inizializza tutti gli hook
    thread::spawn(move || {
        // Lancia hook input
        if let Err(e) = mouse::init_hook(tx.clone()) {
            eprintln!("Errore nell'hook mouse: {:?}", e)
        }

        if let Err(e) = keyboard::init_hook(tx.clone(), &app_handle_copy) {
            eprintln!("Errore nell'hook keyboard: {:?}", e);
        }

        // Lancia hook finestre
        if let Err(e) = window::init_hook(tx.clone()) {
            eprintln!("Errore nell'hook window: {:?}", e);
        }

        // Message loop Windows necessario per mantenere hook vivi
        unsafe {
            let mut msg = windows::Win32::UI::WindowsAndMessaging::MSG::default();
            while windows::Win32::UI::WindowsAndMessaging::GetMessageW(
                &mut msg,
                Default::default(),
                0,
                0,
            )
            .into()
            {
                if let Err(e) = windows::Win32::UI::WindowsAndMessaging::TranslateMessage(&msg).ok()
                {
                    eprintln!("Errore durante TranslateMessage: {:?}", e);
                }
                windows::Win32::UI::WindowsAndMessaging::DispatchMessageW(&msg);
            }
        }

        if let Err(e) = keyboard::unmount_hook() {
            eprintln!("Errore nell'unmount dell'hook keyboard: {:?}", e);
        }

        if let Err(e) = mouse::unmount_hook() {
            eprintln!("Errore nell'unmount dell'hook mouse: {:?}", e);
        }

        if let Err(e) = window::unmount_hook() {
            eprintln!("Errore nell'unmount dell'hook window: {:?}", e);
        }
    });

    // Thread consumer: invia eventi al frontend Tauri
    thread::spawn(move || {
        while let Ok(event) = rx.recv() {
            match event {
                GlobalEvent::MouseMove { x, y } => {
                    let _ = app_handle.emit_to("overlay", "global_mouse_moved", (x, y));
                    let _ = app_handle.emit_to("wallpaper", "global_mouse_moved", (x, y));
                }
                GlobalEvent::MouseButtonDown(btn) => {
                    let _ = app_handle.emit_to("overlay", "global_mouse_pressed", btn);
                }
                GlobalEvent::MouseButtonUp(btn) => {
                    let _ = app_handle.emit_to("overlay", "global_mouse_released", btn);
                }
                GlobalEvent::MouseWheel(position) => {
                    let _ = app_handle.emit_to("overlay", "global_mouse_wheel", position);
                }
                GlobalEvent::KeyDown(key) => {
                    let _ = app_handle.emit_to("overlay", "global_key_pressed", key);
                }
                GlobalEvent::KeyUp(key) => {
                    let _ = app_handle.emit_to("overlay", "global_key_released", key);
                }
                GlobalEvent::WindowEnteredFullscreen { hwnd } => {
                    //set_snap_flyout(false).map_err(|e| format!("Errore set_snap_flyout: {e}")).unwrap();

                    let _ = app_handle.emit_to("overlay", "window_entered_fullscreen", hwnd);
                }
                GlobalEvent::WindowExitedFullscreen { hwnd } => {
                    let state = app_handle.state::<Arc<Mutex<BrickUIState>>>();
                    let state_guard = state
                        .lock()
                        .map_err(|e| format!("Mutex poisoned: {e}"))
                        .unwrap();
                    let settings = state_guard.get_settings();

                    if settings.taskbar.behavior != TaskBarBehavior::WindowsDefault {
                        hide_taskbar(settings.taskbar.behavior != TaskBarBehavior::HideAndFill)
                            .unwrap();
                    }

                    //set_snap_flyout(false).map_err(|e| format!("Errore set_snap_flyout: {e}")).unwrap();

                    let _ = app_handle.emit_to("overlay", "window_exited_fullscreen", hwnd);
                }
            }
        }
    });

    Ok(())
}
