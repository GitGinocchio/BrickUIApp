#[cfg(feature = "profiling")]
use tracing::instrument;
#[cfg(feature = "profiling")]
use tracing_flame::FlameLayer;
#[cfg(feature = "profiling")]
use tracing_flame::FlushGuard;
#[cfg(feature = "profiling")]
use tracing_subscriber::prelude::*;
#[cfg(feature = "profiling")]
use tracing_subscriber::registry::Registry;
#[cfg(feature = "profiling")]
use once_cell::sync::Lazy;
#[cfg(feature = "profiling")]
use std::fs::File;
#[cfg(feature = "profiling")]
use std::io::BufWriter;

use std::sync::Arc;
use tauri::{Emitter, Manager, WindowEvent};
use tokio::sync::{RwLock, Mutex};

use tauri_plugin_autostart::MacosLauncher;

mod state;
use crate::state::bluetooth::BrickUIBluetoothState;
use crate::state::generic::BrickUIGenericState;

mod winapi;
use crate::winapi::com::{initialize_com, uninitialize_com};
use crate::winapi::sock::inititalize_sockets;
use crate::winapi::cursor::restore_cursors;
use crate::winapi::events::start_event_listeners;
use crate::winapi::taskbar::{hide_taskbar, show_taskbar};
//use crate::winapi::window::{remove_titlebar, set_window_topmost};

mod config;
use crate::config::settings::TaskBarBehavior;

mod bricks;
mod utils;

mod handlers;
use crate::handlers::generate_handlers;
use crate::winapi::monitor::workarea::reset_workareas;

#[cfg(feature = "profiling")]
static PROFILER: Lazy<std::sync::Mutex<Option<FlushGuard<BufWriter<File>>>>> = Lazy::new(|| std::sync::Mutex::new(None));

#[cfg(feature = "profiling")]
fn setup_global_subscriber() -> tracing_flame::FlushGuard<std::io::BufWriter<std::fs::File>> {
    let fmt_layer = tracing_subscriber::fmt::Layer::default();
    let (flame_layer, guard) = tracing_flame::FlameLayer::with_file("./trace.folded").unwrap();

    let subscriber = tracing_subscriber::registry::Registry::default()
        .with(fmt_layer)
        .with(flame_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Could not set global default");

    guard
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(feature = "profiling")]
    {
        let guard = setup_global_subscriber();
        *PROFILER.lock().unwrap() = Some(guard);
    }


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
            inititalize_sockets()?;

            let app_handle = app.app_handle();
            let resolver = app_handle.path();
            let resource_path = resolver.resource_dir()?;
            let path = resolver.app_data_dir()?;

            // General App State
            let state = BrickUIGenericState::new(&path, &resource_path)?;
            let settings = state.get_settings().clone();
            
            app.manage(Arc::new(Mutex::new(state)));

            // Bluetooth State
            let bt_state = BrickUIBluetoothState::new()?;
            app.manage(Arc::new(RwLock::new(bt_state)));

            // Mostra o nascondi taskbar secondo settings
            if settings.taskbar.behavior == TaskBarBehavior::Show {
                show_taskbar(&app_handle).expect("Errore mostrando la taskbar:")
            } else {
                hide_taskbar(&app_handle).expect("Errore nascondendo la taskbar:");
            }

            let app_handle_clone = app_handle.clone();
            
            // Avvia event listeners async
            tauri::async_runtime::spawn(async move {
                start_event_listeners(app_handle_clone).await
            });

            Ok(())
        })
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
            println!("event: {event:?}");
            if let WindowEvent::CloseRequested { api, .. } = event && window.label() == "main" {
                static HANDLED: once_cell::sync::Lazy<Mutex<bool>> = once_cell::sync::Lazy::new(|| Mutex::new(false));

                let mut handled = tauri::async_runtime::block_on(async {
                    HANDLED.lock().await 
                });

                if *handled {
                    // già gestito, non fare nulla
                    #[cfg(feature = "profiling")]
                    std::process::exit(0);
                    #[cfg(not(feature = "profiling"))]
                    return;
                }

                *handled = true;

                api.prevent_close();

                let app_handle = window.app_handle();
                let window_label = window.label().to_string();
                let state = app_handle.state::<Arc<Mutex<BrickUIGenericState>>>().clone();

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

                if let Some(window) = app_handle.get_window(&window_label) {
                    window.close().expect("Error closing main window:");
                }

                #[cfg(feature = "profiling")]
                {
                    if let Some(guard) = PROFILER.lock().unwrap().take() {
                        drop(guard); // qui sei sicuro che il FlameLayer venga droppato
                    }
                }

                app_handle.exit(0);
            }
            
        })
        .run(context)
        .expect("error while running brickui application");
}
