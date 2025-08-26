use std::time::Duration;
use std::time::Instant;

use rdev::{listen, EventType};
use tauri::{Emitter};

use std::sync::Arc;
use std::sync::Mutex;

pub mod handlers;

pub fn start_global_input_listener<R: tauri::Runtime>(app_handle: tauri::AppHandle<R>) {
    std::thread::spawn(move || {
        let last_move_emit = Arc::new(Mutex::new(Instant::now()));

        if let Err(error) = listen(move |event| {
            match event.event_type {
                EventType::MouseMove { x, y } => {
                    let mut last = last_move_emit.lock().unwrap();
                    let now = Instant::now();
                    
                    // throttling: max 1 evento ogni 16ms (≈60 fps)
                    if now.duration_since(*last) >= Duration::from_millis(32) {
                        // Invia evento alla WebView
                        let _ = app_handle.emit_to(
                            "overlay",
                            "global_mouse_moved", 
                            (x, y)
                        );
                        *last = now;
                    }
                },
                EventType::ButtonPress(button) => {
                    let _ = app_handle.emit_to(
                        "overlay", 
                        "global_mouse_pressed", 
                        format!("{:?}", button)
                    );
                },
                EventType::ButtonRelease(button) => {
                    let _ = app_handle.emit_to(
                        "overlay", 
                        "global_mouse_released", 
                        format!("{:?}", button)
                    );
                },
                EventType::KeyPress(key) => {
                    let _ = app_handle.emit_to(
                        "overlay", 
                        "global_key_pressed", 
                        format!("{:?}", key)
                    );
                },
                EventType::KeyRelease(key) => {
                    let _ = app_handle.emit_to(
                        "overlay", 
                        "global_key_released", 
                        format!("{:?}", key)
                    );
                },
                EventType::Wheel { delta_x, delta_y } => {
                    let _ = app_handle.emit_to(
                        "overlay", 
                        "global_wheel", 
                        (delta_x, delta_y)
                    );
                }
            };
        }) {
            eprintln!("Errore nell'ascolto eventi: {:?}", error);
        }
    });
}