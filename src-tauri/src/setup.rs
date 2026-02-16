use std::{error::Error, path::PathBuf, sync::Arc};

use tauri::{App, AppHandle, Emitter as _, Manager as _, Window, WindowEvent};
use tauri_plugin_deep_link::DeepLinkExt;
use tokio::sync::{Mutex, RwLock};

#[cfg(feature = "profiling")]
use crate::profiler::{
    drop_global_subscriber,
    setup_global_subscriber
};

use crate::{
    api::deeplink::handle_deeplink, config::settings::TaskBarBehavior, state::{
        bluetooth::BrickUIBluetoothState, 
        generic::BrickUIGenericState, 
        user::{
            BrickUIUserState, 
            refresh_session_if_present
        }
    }, utils::focus_window, winapi::{
        bluetooth::start_bluetooth_watcher, 
        com::{
            initialize_com, 
            uninitialize_com
        }, 
        cursor::restore_cursors, 
        events::start_event_listeners, 
        monitor::workarea::reset_workareas, 
        sock::inititalize_sockets, 
        taskbar::{
            hide_taskbar, 
            show_taskbar
        }
    }
};

async fn initialize_dirs(path: &PathBuf) -> Result<(), String> {
    tokio::fs::create_dir_all(path)
        .await
        .map_err(|e| format!("Errore nella creazione della directory di dati: {e}"))?;
    
    tokio::fs::create_dir_all(path.join("bricks"))
        .await
        .map_err(|e| format!("Errore nella creazione della directory per i widgets (bricks): {e}"))?;
    
    tokio::fs::create_dir_all(path.join("walls"))
        .await
        .map_err(|e| format!("Errore nella creazione della directory per i widgets (walls): {e}"))?;
    
    tokio::fs::create_dir_all(path.join(".schemas"))
        .await
        .map_err(|e| format!("Errore nella creazione della directory per gli schemas: {e}"))?;

    Ok(())
}

pub fn setup_single_instance(app: &AppHandle, args: Vec<String>, _cwd: String) {
    println!("App already opened and triggered with args: {args:?}");
    if let Some(slice) = args.get(1..) {
        if let Some(url) = slice.iter().find(|a| a.starts_with("brickui:/")) {
            let url = url.clone();
            let app = app.clone();
            tauri::async_runtime::spawn(async move {
                handle_deeplink(&app, &url)
                    .await
                    .expect("Error handling deeplink")
            });
            return;
        }

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

        if let Some(bricks) = bricks {
            app.emit_to("main", "open_brick", bricks)
                .expect("Error while sending 'opened_brick' event:");
        }
    }

    // Facciamo il focus della main window
    focus_window(&app, "main").expect("Error focusing window")
}

pub fn on_window_event(window: &Window, event: &WindowEvent) {
    //println!("event: {event:?}");
    if let WindowEvent::CloseRequested { api, .. } = event && window.label() == "main" {
        static HANDLED: once_cell::sync::Lazy<Mutex<bool>> =
            once_cell::sync::Lazy::new(|| Mutex::new(false));

        let mut handled = tauri::async_runtime::block_on(async { HANDLED.lock().await });

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
        let state = app_handle
            .state::<Arc<Mutex<BrickUIGenericState>>>()
            .clone();

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
        drop_global_subscriber();

        app_handle.exit(0);
    }
}

pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "profiling")]
    setup_global_subscriber();

    let deep_link = app.deep_link();
    deep_link.register("brickui").map_err(|e| format!("Error registering 'brickui' deeplink: {e}"))?;
    deep_link.register_all().map_err(|e| format!("Error registering all deeplink: {e}"))?;

    let app_handle = app.handle().clone();

    deep_link.on_open_url(move |event| {
        let urls = event.urls();
        println!("deep link URLs: {:?}", urls);

        let last_url = match urls.last() {
            None => return,
            Some(l) => l.to_string()
        };

        let app_handle = app_handle.clone();
        tauri::async_runtime::spawn(async move {
            if let Err(e) = handle_deeplink(&app_handle, &last_url).await {
                eprintln!("Error handling deeplink: {:?}", e);
            }
        });
    });

    initialize_com()?;
    inititalize_sockets()?;

    let app_handle = app.app_handle();
    let resolver = app_handle.path();
    let resource_path = resolver.resource_dir()?;
    let path = resolver.app_data_dir()?;

    // General App State
    let state = tauri::async_runtime::block_on(async move {
        initialize_dirs(&path).await?;
        BrickUIGenericState::new(&path, &resource_path).await
    })?;

    let settings = state.get_settings().clone();

    app.manage(Arc::new(Mutex::new(state)));

    // Bluetooth State
    let bt_state = Arc::new(RwLock::new(BrickUIBluetoothState::new()?));
    app.manage(bt_state.clone());

    // User State
    let user_state = Arc::new(Mutex::new(BrickUIUserState::new()?));
    app.manage(user_state.clone());

    // Mostra o nascondi taskbar secondo settings
    if settings.taskbar.behavior == TaskBarBehavior::Show {
        show_taskbar(&app_handle)?;
    } else {
        hide_taskbar(&app_handle)?;
    }

    let app_handle_clone = app_handle.clone();

    // Avvia metodi in background (evitando di bloccare la UI)
    tauri::async_runtime::spawn(
        async move {
            // Fa partire gli event listeners
            start_event_listeners(&app_handle_clone).await?;

            // Aggiorna la sessione se presente
            let mut guard = bt_state.write().await;
            guard.watcher = Some(start_bluetooth_watcher(bt_state.clone(), &app_handle_clone).await?);

            if let Err(e) = refresh_session_if_present(user_state).await {
                eprintln!("An error occurred while refreshing session on startup: {e:#?}");
            }

            Ok::<(), String>(())
        },
    );

    Ok(())
}