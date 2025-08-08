use rdev::{listen, EventType};
use tauri::{Emitter};

pub fn start_input_listener<R: tauri::Runtime>(app_handle: tauri::AppHandle<R>) {
    std::thread::spawn(move || {
        if let Err(error) = listen(move |event| {
            if let EventType::MouseMove { x, y } = event.event_type {
                // Invia evento alla WebView
                let _ = app_handle.emit_to("overlay","global_mouse_move", (x, y));
            }

            if let EventType::ButtonPress(button) = event.event_type {
                // Invia click
                let _ = app_handle.emit_to("overlay", "global_mouse_click", format!("{:?}", button));
            }
        }) {
            eprintln!("Errore nell'ascolto eventi: {:?}", error);
        }
    });
}