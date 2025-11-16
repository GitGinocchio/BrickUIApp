use crate::{
    config::settings::TaskBarBehavior, state::BrickUIState, winapi::taskbar::hide_taskbar,
};
use crossbeam::channel;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::Mutex;

mod keyboard;
mod mouse;
mod window;

#[derive(Debug, Clone)]
pub enum GlobalEvent {
    MouseMove { x: i32, y: i32 },
    MouseButtonDown { x: i32, y: i32, button: String },
    MouseButtonUp { x: i32, y: i32, button: String },
    MouseWheel { x: i32, y: i32, notches: i16 },

    KeyDown(String),
    KeyUp(String),

    //WindowCreated { hwnd: usize },
    //WindowDestroyed { hwnd: usize },
    WindowEnteredFullscreen { hwnd: usize },
    WindowExitedFullscreen { hwnd: usize },
}

pub async fn start_event_listeners(app_handle: AppHandle) -> Result<(), String> {
    let (tx, rx) = channel::unbounded();

    let app_handle_clone = app_handle.clone();

    // Hook input (mouse, keyboard, window)
    // Avviamo gli hook in un thread separato, ma senza toccare Tauri state
    tauri::async_runtime::spawn(async move {
        if let Err(e) = mouse::init_hook(tx.clone()) {
            eprintln!("Errore nell'hook mouse: {:?}", e)
        }

        if let Err(e) = keyboard::init_hook(tx.clone(), &app_handle_clone).await {
            eprintln!("Errore nell'hook keyboard: {:?}", e);
        }

        if let Err(e) = window::init_hook(tx.clone()) {
            eprintln!("Errore nell'hook window: {:?}", e);
        }

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
                let _ = windows::Win32::UI::WindowsAndMessaging::TranslateMessage(&msg);
                windows::Win32::UI::WindowsAndMessaging::DispatchMessageW(&msg);
            }
        }

        let _ = keyboard::unmount_hook();
        let _ = mouse::unmount_hook();
        let _ = window::unmount_hook();
    });

    // Tokio task async: riceve eventi e parla con Tauri
    let app_handle_clone = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        while let Ok(event) = rx.recv() {
            match event {
                GlobalEvent::MouseMove { x, y } => {
                    let _ = app_handle_clone.emit_to("overlay", "global_mouse_moved", (x, y));
                }
                GlobalEvent::MouseButtonDown { x, y, button } => {
                    let _ =
                        app_handle_clone.emit_to("overlay", "global_mouse_pressed", (x, y, button));
                }
                GlobalEvent::MouseButtonUp { x, y, button } => {
                    let _ = app_handle_clone.emit_to(
                        "overlay",
                        "global_mouse_released",
                        (x, y, button),
                    );
                }
                GlobalEvent::MouseWheel { x, y, notches } => {
                    let _ =
                        app_handle_clone.emit_to("overlay", "global_mouse_wheel", (x, y, notches));
                }
                GlobalEvent::KeyDown(key) => {
                    let _ = app_handle_clone.emit_to("overlay", "global_key_pressed", key);
                }
                GlobalEvent::KeyUp(key) => {
                    let _ = app_handle_clone.emit_to("overlay", "global_key_released", key);
                }
                GlobalEvent::WindowEnteredFullscreen { hwnd } => {
                    println!("Window entered fullscreen!");
                    let _ = app_handle_clone.emit_to("overlay", "window_entered_fullscreen", hwnd);
                }
                GlobalEvent::WindowExitedFullscreen { hwnd } => {
                    println!("Window exited fullscreen!");

                    // Recupera stato solo ora, quando esiste
                    if let Some(state) = app_handle_clone.try_state::<Arc<Mutex<BrickUIState>>>() {
                        let settings = {
                            let state_guard = state.lock().await;
                            state_guard.get_settings().clone()
                        };

                        if settings.taskbar.behavior != TaskBarBehavior::Show {
                            if let Err(e) = hide_taskbar(&app_handle_clone) {
                                eprintln!("Errore hide_taskbar: {:?}", e);
                            }
                        }
                    } else {
                        eprintln!("State non ancora gestito (ignorato)");
                    }

                    let _ = app_handle_clone.emit_to("overlay", "window_exited_fullscreen", hwnd);
                }
                event => {
                    eprintln!("Evento non riconosciuto: {event:?}");
                }
            }
        }
    });

    Ok(())
}
