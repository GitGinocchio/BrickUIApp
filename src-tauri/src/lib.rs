use std::sync::Arc;
use tauri::{Emitter, Manager, WindowEvent};
use tokio::sync::Mutex;

mod state;
use state::BrickUIState;
use tauri_plugin_autostart::MacosLauncher;

mod winapi;
use crate::winapi::com::{initialize_com, uninitialize_com};
use crate::winapi::cursor::restore_cursors;
use crate::winapi::events::start_event_listeners;
use crate::winapi::taskbar::{hide_taskbar, show_taskbar};
//use crate::winapi::window::{remove_titlebar, set_window_topmost};

mod config;
use crate::config::settings::TaskBarBehavior;

mod bricks;

mod handlers;
use crate::handlers::generate_handlers;
use crate::winapi::monitor::workarea::reset_workareas;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();
    /*
    if let tauri::Pattern::Isolation { schema, .. } = context.pattern() {
        dbg!(schema);
    }
    */

    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            initialize_com()?;

            let app_handle = app.app_handle();
            let resolver = app_handle.path();
            let resource_path = resolver.resource_dir()?;
            let path = resolver.app_data_dir()?;

            let state = BrickUIState::new(&path, &resource_path)?;
            let settings = state.get_settings().clone();
            app.manage(Arc::new(Mutex::new(state)));

            // Mostra o nascondi taskbar secondo settings
            if settings.taskbar.behavior == TaskBarBehavior::Show {
                show_taskbar(&app_handle).expect("Errore mostrando la taskbar:")
            } else {
                hide_taskbar(&app_handle).expect("Errore nascondendo la taskbar:");
            }

            // Avvia event listeners async
            tauri::async_runtime::block_on(async {
                start_event_listeners(app_handle.clone()).await
            })?;

            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_blec::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
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
                    .filter(|f| {
                        f.ends_with(".brick")
                            || f.ends_with(".brk")
                            || f.ends_with(".brck")
                            || f.ends_with(".bk")
                    })
                    .map(|b| {
                        (
                            b,
                            b.split("\\")
                                .last()
                                .unwrap_or("Brick")
                                .split(".")
                                .next()
                                .unwrap_or("Brick")
                                .to_string(),
                        )
                    })
                    .last();

                println!("{bricks:?}");

                app.emit_to("main", "open_brick", bricks)
                    .expect("Error while sending 'opened_brick' event:");
            }

            //println!("{_args:?}, {_cwd:?}");
        }))
        .invoke_handler(generate_handlers())
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event
                && window.label() == "main"
            {
                api.prevent_close();

                let app_handle = window.app_handle();
                let window_label = window.label().to_string();
                let state = app_handle.state::<Arc<Mutex<BrickUIState>>>().clone();

                let (settings, backup) = tauri::async_runtime::block_on(async {
                    let state_guard = state.lock().await;
                    (
                        state_guard.get_settings().clone(),
                        state_guard.get_backup().clone(),
                    )
                });

                if settings.systemtray.enabled && settings.systemtray.hidetaskbaricon {
                    if let Some(window) = app_handle.get_window(&window_label) {
                        window.hide().expect("Error hiding main window:");
                        return;
                    }
                }

                if let Some(window) = app_handle.get_window(&window_label) {
                    window.hide().expect("Error hiding main window:");
                }

                if let Some(overlay) = app_handle.get_window("overlay") {
                    overlay.close().expect("Error closing overlay window:");
                }

                if let Some(wallpaper) = app_handle.get_window("wallpaper") {
                    wallpaper.close().expect("Error closing wallpaper window:");
                }

                reset_workareas().unwrap();
                if settings.taskbar.behavior != TaskBarBehavior::Show {
                    show_taskbar(app_handle).expect("Error showing taskbar:");
                }

                restore_cursors(&backup.cursors).expect("Error restoring cursors:");
                uninitialize_com();

                //app_handle.cleanup_before_exit();

                tauri::async_runtime::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                    std::process::exit(0);
                });
            }
        })
        .run(context)
        .expect("error while running brickui application");
}
