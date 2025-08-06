
mod windows_utils;
use tauri::{Manager, WindowEvent};

use crate::windows_utils::{
    disable_window_focus, hide_taskbar, hide_titlebar, make_window_click_through, show_taskbar
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            
        ])
        .setup(|app| {
            match hide_taskbar(false) {
                Ok(_) => println!("Taskbar nascosta con successo"),
                Err(e) => eprintln!("Errore nel nascondere la taskbar: {e:?}"),
            }

            let window = app.get_webview_window("overlay").unwrap();
            make_window_click_through(&window);
            hide_titlebar(&window);
            disable_window_focus(&window);

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { .. } = event && window.label() == "settings" {
                match show_taskbar() {
                    Ok(_) => println!("Taskbar mostrata con successo"),
                    Err(e) => eprintln!("Errore nel mostrare la taskbar: {e:?}"),
                }

                if let Some(app_handle) = window.app_handle().get_webview_window("overlay") {
                    let _ = app_handle.close();
                }
            }

            /*
            if let WindowEvent::Focused(false) = event && window.label() == "overlay" {
                if let Some(webview) = window.get_webview_window("overlay") {
                    hide_titlebar(&webview);
                }
            }
            */
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
