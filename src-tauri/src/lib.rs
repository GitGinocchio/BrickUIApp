mod overlay_utils;
use crate::overlay_utils::{
    disable_click_through, enable_click_through, hide_taskbar, 
    hide_titlebar, show_taskbar, start_input_listener
};

use tauri::{Manager, WindowEvent};

mod state;
use state::BrickUIState;

#[tauri::command]
async fn disable_click_through_command(app: tauri::AppHandle) -> Result<(), String> {
    let window = app.get_webview_window("overlay").ok_or("Overlay window not found")?;
    disable_click_through(&window);
    println!("Disabled click through");
    Ok(())
}

#[tauri::command]
async fn enable_click_through_command(app: tauri::AppHandle) -> Result<(), String> {
    let window = app.get_webview_window("overlay").ok_or("Overlay window not found")?;
    enable_click_through(&window);
    println!("Enabled click through");
    Ok(())
}

#[tauri::command]
async fn hide_titlebar_command(app: tauri::AppHandle) -> Result<(), String> {
    let window = app.get_webview_window("overlay").ok_or("Overlay window not found")?;
    hide_titlebar(&window);
    println!("Enabled click through");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            disable_click_through_command,
            enable_click_through_command,
            hide_titlebar_command
        ])
        .setup(|app| {
            let path = app.app_handle().path().app_data_dir()?;

            app.manage(BrickUIState::new(&path));

            start_input_listener(app.handle().clone());

            match hide_taskbar(false) {
                Ok(_) => println!("Taskbar nascosta con successo"),
                Err(e) => eprintln!("Errore nel nascondere la taskbar: {e:?}"),
            }

            let window = app.get_webview_window("overlay").unwrap();
            hide_titlebar(&window);
            enable_click_through(&window);
            //window.open_devtools();

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
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
