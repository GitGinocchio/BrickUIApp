mod overlay_window;

use crate::overlay_window::utils::{
    hide_taskbar, 
    show_taskbar, 
    remove_titlebar
};

mod global_events;
use crate::global_events::start_global_input_listener;

mod winapi;
use crate::winapi::taskbar::apps::get_taskbar_icons;

mod bricks;
use crate::bricks::brick::Brick;

mod config;
use crate::config::settings::{Settings, TaskBarBehavior};

use tauri::{Manager, State, WindowEvent};

mod state;
use state::BrickUIState;

#[tauri::command]
fn get_bricks(state: State<BrickUIState>) -> Vec<Brick> {
    state.get_bricks().to_vec()
}

#[tauri::command]
fn get_settings(state: State<BrickUIState>) -> Settings {
    state.get_settings().clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            get_taskbar_icons,
            get_settings,
            get_bricks
        ])
        .setup(|app| {
            let path = app.app_handle().path().app_data_dir()?;

            let state = BrickUIState::new(&path);
            let settings = state.get_settings().clone();

            app.manage(state);

            if settings.taskbar.behavior != TaskBarBehavior::WindowsDefault {
                match show_taskbar() {
                    Ok(_) => println!("Taskbar mostrata con successo"),
                    Err(e) => eprintln!("Errore nel mostrare la taskbar: {e:?}"),
                }

                match hide_taskbar(settings.taskbar.behavior != TaskBarBehavior::HideAndFill) {
                    Ok(_) => println!("Taskbar nascosta con successo"),
                    Err(e) => eprintln!("Errore nel nascondere la taskbar: {e:?}"),
                }
            }

            let webview = app.get_webview_window("overlay").unwrap();
            let window = &webview.get_window("overlay").unwrap();
            remove_titlebar(window);

            start_global_input_listener(app.handle().clone());

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { .. } = event && window.label() == "settings" {
                let app_handle = window.app_handle();
                let state = app_handle.state::<BrickUIState>();
                let settings = state.get_settings();

                if settings.taskbar.behavior != TaskBarBehavior::WindowsDefault {
                    match show_taskbar() {
                        Ok(_) => println!("Taskbar mostrata con successo"),
                        Err(e) => eprintln!("Errore nel mostrare la taskbar: {e:?}"),
                    }
                }

                if let Some(app_handle) = app_handle.get_webview_window("overlay") {
                    let _ = app_handle.close();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

