mod winapi;
use crate::winapi::events::start_event_listeners;
use crate::winapi::set_snap_flyout;
use crate::winapi::taskbar::apps::GroupedIcons;
use crate::winapi::window::{remove_titlebar, set_as_wallpaper_background};

mod bricks;
use crate::bricks::brick::Brick;

mod config;
use crate::config::settings::{Settings, TaskBarBehavior};

use std::sync::{Arc, Mutex};
use tauri::{Manager, State, WindowEvent};

mod state;
use state::BrickUIState;
use tauri_plugin_autostart::MacosLauncher;

#[tauri::command]
fn hide_taskbar(keep_taskbar_space: bool) -> Result<(), String> {
    winapi::taskbar::show_taskbar()?;
    winapi::taskbar::hide_taskbar(keep_taskbar_space)
}

#[tauri::command]
fn show_taskbar() -> Result<(), String> {
    winapi::taskbar::show_taskbar()
}

#[tauri::command]
fn get_taskbar_icons(
    state: State<'_, Arc<Mutex<BrickUIState>>>,
) -> Result<Vec<GroupedIcons>, String> {
    let state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    let path = state_guard.get_path();

    Ok(crate::winapi::taskbar::apps::get_taskbar_icons(&path))
}

#[tauri::command]
fn get_settings(state: State<'_, Arc<Mutex<BrickUIState>>>) -> Result<Settings, String> {
    let state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    Ok(state_guard.get_settings().clone())
}

#[tauri::command]
fn save_settings(
    state: State<'_, Arc<Mutex<BrickUIState>>>,
    settings: Settings,
) -> Result<(), String> {
    let mut state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    let path = state_guard.get_path();

    let current_settings = state_guard.get_mut_settings();
    *current_settings = settings.clone();

    crate::config::save_settings(&path, settings)
}

#[tauri::command]
fn get_bricks(state: State<'_, Arc<Mutex<BrickUIState>>>) -> Result<Vec<Brick>, String> {
    let state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;

    Ok(state_guard.get_bricks().to_vec())
}

#[tauri::command]
fn get_brick_by_name(state: State<'_, Arc<Mutex<BrickUIState>>>, name: String) -> Result<Option<Brick>, String> {
    let state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;

    Ok(state_guard.get_brick_by_name(&name))
}

#[tauri::command]
fn load_bricks(state: State<'_, Arc<Mutex<BrickUIState>>>) -> Result<Vec<Brick>, String> {
    let mut state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;

    let path = state_guard.get_path();
    let bricks = bricks::load_bricks(&path)?;

    let bricks_return = bricks.clone();
    *state_guard.get_mut_bricks() = bricks;

    Ok(bricks_return)
}

#[tauri::command]
fn save_brick(state: State<'_, Arc<Mutex<BrickUIState>>>, brick: Brick) -> Result<(), String> {
    let mut state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;

    let path = state_guard.get_path();

    bricks::save_brick(&path, &brick)?;

    // Aggiorna lo stato in memoria
    if let Some(existing) = state_guard
        .get_mut_bricks()
        .iter_mut()
        .find(|b| b.name == brick.name)
    {
        *existing = brick;
    } else {
        state_guard.get_mut_bricks().push(brick);
    }

    Ok(())
}

#[tauri::command]
fn rename_brick(
    state: State<'_, Arc<Mutex<BrickUIState>>>,
    old_name: String,
    new_name: String,
) -> Result<(), String> {
    let mut state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    let path = state_guard.get_path();

    bricks::rename_brick(&path, &old_name, &new_name)?;

    if let Some(brick) = state_guard.bricks.iter_mut().find(|b| b.name == old_name) {
        brick.name = new_name;
    }

    Ok(())
}

#[tauri::command]
fn duplicate_brick(state: State<'_, Arc<Mutex<BrickUIState>>>, brick: Brick) -> Result<(), String> {
    let mut state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    let path = state_guard.get_path();

    let new_brick = bricks::duplicate_brick(&path, brick)?;

    state_guard.bricks.push(new_brick);

    Ok(())
}

// Questo non serve a molto potrebbe essere sostituito con il plugin opener e basta
#[tauri::command]
fn open_brick(
    state: State<'_, Arc<Mutex<BrickUIState>>>,
    brick_name: String,
) -> Result<(), String> {
    let state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    let path = state_guard.get_path();

    bricks::open_brick(&path, brick_name)
}

#[tauri::command]
fn delete_brick(state: State<'_, Arc<Mutex<BrickUIState>>>, brick: Brick) -> Result<(), String> {
    let mut state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    let path = state_guard.get_path();

    state_guard.bricks.retain(|b| b.name != brick.name);

    bricks::delete_brick(&path, &brick)
}

#[tauri::command]
fn new_brick(state: State<'_, Arc<Mutex<BrickUIState>>>, brick: Brick) -> Result<(), String> {
    let mut state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    let path = state_guard.get_path();

    bricks::create_brick(&path, &brick)?;
    state_guard.bricks.push(brick);

    Ok(())
}

#[tauri::command]
fn open_start_menu() -> Result<(), String> {
    crate::winapi::startmenu::open_start_menu();

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();

    /*
    if let tauri::Pattern::Isolation { schema, .. } = context.pattern() {
        dbg!(schema);
    }
    */

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(webview_window) = app.get_webview_window("main") {
                let _ = webview_window.unminimize();

                if !webview_window.is_visible().unwrap_or(false) {
                    let _ = webview_window.show();
                }

                let _ = webview_window.set_focus();
            } else {
                eprintln!("no main window");
            }
        }))
        .invoke_handler(tauri::generate_handler![
            hide_taskbar,
            show_taskbar,
            get_taskbar_icons,
            get_settings,
            save_settings,
            get_bricks,
            get_brick_by_name,
            load_bricks,
            duplicate_brick,
            delete_brick,
            rename_brick,
            save_brick,
            open_brick,
            new_brick,
            open_start_menu
        ])
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

            let wallpaperwv = app.get_webview("wallpaper").unwrap();
            let wallpaperw = app.get_window("wallpaper").unwrap();
            let hwnd = wallpaperw.hwnd().map_err(|e| format!("Errore durante l'ottenimento dell'HWND: {e}"))?;
            set_as_wallpaper_background(hwnd)?;

            wallpaperwv.open_devtools();

            set_snap_flyout(false).map_err(|e| format!("Errore set_snap_flyout: {e}"))?;

            start_event_listeners(app.handle().clone())?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event && window.label() == "main" {
                let app_handle = window.app_handle();
                let state = app_handle.state::<Arc<Mutex<BrickUIState>>>();
                let state_guard = match state.lock().map_err(|e| format!("errore lock: {e}")) {
                    Ok(guard) => guard,
                    Err(e) => panic!("{e}"),
                };
                let settings = state_guard.get_settings();

                if settings.systemtray.enabled 
                && settings.systemtray.hidetaskbaricon 
                && let Ok(true) = window.is_visible() { 
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
                        .map_err(|e| format!("Error while trying to hide the wallpaper window: {e}"))
                        .expect("");
                    window
                        .close()
                        .map_err(|e| format!("Error while trying to close the wallpaper window: {e}"))
                        .unwrap();
                }

                if settings.taskbar.behavior != TaskBarBehavior::WindowsDefault {
                    match show_taskbar() {
                        Err(e) => eprintln!("Errore nel mostrare la taskbar: {e:?}"),
                        _ => (),
                    };
                }

                set_snap_flyout(true)
                    .map_err(|e| format!("Errore set_snap_flyout: {e}"))
                    .expect("");

                app_handle.exit(0);
            }
        })
        .run(context)
        .expect("error while running tauri application");
}
