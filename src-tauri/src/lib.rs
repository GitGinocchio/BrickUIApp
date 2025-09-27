mod winapi;
use crate::handlers::generate_handlers;
use crate::winapi::events::start_event_listeners;
use crate::winapi::taskbar::{hide_taskbar, show_taskbar};
use crate::winapi::window::remove_titlebar;

mod bricks;

mod config;
use crate::config::settings::{TaskBarBehavior};

use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager, WindowEvent};

mod state;
use state::BrickUIState;
use tauri_plugin_autostart::MacosLauncher;

mod handlers;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();
    /*
    if let tauri::Pattern::Isolation { schema, .. } = context.pattern() {
        dbg!(schema);
    }
    */

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            if let Some(webview_window) = app.get_webview_window("main") {
                let _ = webview_window.unminimize();

                if !webview_window.is_visible().unwrap_or(false) {
                    let _ = webview_window.show();
                }

                let _ = webview_window.set_focus();
            } else {
                panic!("Can't get the main window");
            }

            if let Some(slice) = args.get(1..) {
                let bricks: Option<(&String, String)> = slice
                    .iter()
                    .filter(|f| f.ends_with(".brick") || 
                                f.ends_with(".brk") ||
                                f.ends_with(".brck") ||
                                f.ends_with(".bk")
                    )
                    .map(|b| (b, b.split("\\")
                                    .last()
                                    .unwrap_or("Brick")
                                    .split(".")
                                    .next()
                                    .unwrap_or("Brick")
                                    .to_string())
                    )
                    .last();

                println!("{bricks:?}");

                app.emit_to("main", "open_brick", bricks)
                    .expect("Error while sending 'opened_brick' event:");
            }

            //println!("{_args:?}, {_cwd:?}");

        }))
        .invoke_handler(generate_handlers())
        .setup(|app| {
            let resolver = app.app_handle().path();
            let resource_path = resolver.resource_dir()?;
            let path = resolver.app_data_dir()?;

            let state = BrickUIState::new(&path, &resource_path);
            app.manage(Arc::new(Mutex::new(state)));

            let state = app.state::<Arc<Mutex<BrickUIState>>>();
            let state_guard = state.lock().map_err(|e| format!("errore lock: {e}"))?;

            let settings = state_guard.get_settings().clone();

            if settings.taskbar.behavior != TaskBarBehavior::WindowsDefault {
                show_taskbar()?;
                hide_taskbar(settings.taskbar.behavior != TaskBarBehavior::HideAndFill)?;
            }

            let overlayw = app.get_window("overlay").unwrap();
            remove_titlebar(&overlayw);

            //let wallpaperwv = app.get_webview("wallpaper").unwrap();
            //let wallpaperw = app.get_window("wallpaper").unwrap();
            //let hwnd = wallpaperw
                //.hwnd()
                //.map_err(|e| format!("Errore durante l'ottenimento dell'HWND: {e}"))?;
            //set_as_wallpaper_background_all_monitors(hwnd)?;

            //wallpaperwv.open_devtools();
            //set_snap_flyout(false).map_err(|e| format!("Errore set_snap_flyout: {e}"))?;
            start_event_listeners(app.handle().clone())?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event
                && window.label() == "main"
            {
                let app_handle = window.app_handle();
                let state = app_handle.state::<Arc<Mutex<BrickUIState>>>();
                let state_guard = match state.lock().map_err(|e| format!("errore lock: {e}")) {
                    Ok(guard) => guard,
                    Err(e) => panic!("{e}"),
                };
                let settings = state_guard.get_settings();

                if settings.systemtray.enabled
                    && settings.systemtray.hidetaskbaricon
                    && let Ok(true) = window.is_visible()
                {
                    api.prevent_close();
                    window
                        .hide()
                        .map_err(|e| format!("Error while trying to hide the main window: {e}"))
                        .expect("");
                    return;
                };

                window
                    .hide()
                    .map_err(|e| format!("Error while trying to hide the main window: {e}"))
                    .expect("");

                if let Some(window) = app_handle.get_window("overlay")
                    && window.is_closable().is_ok()
                {
                    window
                        .hide()
                        .map_err(|e| format!("Error while trying to hide the overlay window: {e}"))
                        .expect("");
                    window
                        .close()
                        .map_err(|e| format!("Error while trying to close the overlay window: {e}"))
                        .unwrap();
                }

                if let Some(window) = app_handle.get_window("wallpaper")
                    && window.is_closable().is_ok()
                {
                    window
                        .hide()
                        .map_err(|e| {
                            format!("Error while trying to hide the wallpaper window: {e}")
                        })
                        .expect("");
                    window
                        .close()
                        .map_err(|e| {
                            format!("Error while trying to close the wallpaper window: {e}")
                        })
                        .unwrap();
                }

                if settings.taskbar.behavior != TaskBarBehavior::WindowsDefault {
                    match show_taskbar() {
                        Err(e) => eprintln!("Errore nel mostrare la taskbar: {e:?}"),
                        _ => (),
                    };
                }

                /*
                set_snap_flyout(true)
                    .map_err(|e| format!("Errore set_snap_flyout: {e}"))
                    .expect("");
                */

                app_handle.exit(0);
            }
        })
        .run(context)
        .expect("error while running tauri application");
}
