use rdev::{listen, EventType};
use tauri::{Emitter};

pub mod handlers;

pub fn start_global_input_listener<R: tauri::Runtime>(app_handle: tauri::AppHandle<R>) {
    std::thread::spawn(move || {
        if let Err(error) = listen(move |event| {
            match event.event_type {
                EventType::MouseMove { x, y } => {
                    // Invia evento alla WebView
                    let _ = app_handle.emit_to(
                        "overlay",
                        "global_mouse_moved", 
                        (x, y)
                    );
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