use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Manager, State};

use crate::{state::BrickUIState, winapi::{self, taskbar::apps::App}};


#[tauri::command]
pub fn hide_taskbar(
    app_handle: AppHandle
) -> Result<(), String> {
    winapi::taskbar::hide_taskbar(&app_handle)
}

#[tauri::command]
pub fn show_taskbar(
    app_handle: AppHandle
) -> Result<(), String> {
    winapi::taskbar::show_taskbar(&app_handle)
}

#[tauri::command]
pub fn get_active_taskbar_apps(state: State<'_, Arc<Mutex<BrickUIState>>>) -> Result<Vec<App>, String> {
    let mut state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    let path = state_guard.get_path();

    let icon_cache_path = path.join("cache").join("icons");
    let icons_map = state_guard.get_mut_icons_map();

    // Qui max_files andrebbe sostituito con un impostazione presa dal file settings
    Ok(crate::winapi::taskbar::apps::get_active_taskbar_apps(
        &icon_cache_path,
        50,
        icons_map,
    ))
}

#[tauri::command]
pub fn get_pinned_taskbar_apps(
    app_handle: AppHandle,
    state: State<'_, Arc<Mutex<BrickUIState>>>,
) -> Result<Vec<App>, String> {
    let mut state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    let path = state_guard.get_path();

    let icon_cache_path = path.join("cache").join("icons");
    let icons_map = state_guard.get_mut_icons_map();

    let resolver = app_handle.path();
    let config_dir = resolver
        .config_dir()
        .map_err(|e| format!("error obtaining config dir: {e}"))?;

    Ok(crate::winapi::taskbar::apps::get_pinned_taskbar_apps(
        &icon_cache_path,
        &config_dir,
        50,
        icons_map,
    ))
}

#[tauri::command]
pub fn get_taskbar_apps(
    app_handle: AppHandle,
    state: State<'_, Arc<Mutex<BrickUIState>>>,
) -> Result<Vec<App>, String> {
    let mut state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    let path = state_guard.get_path();

    let icon_cache_path = path.join("cache").join("icons");
    let icons_map = state_guard.get_mut_icons_map();

    let resolver = app_handle.path();
    let config_dir = resolver
        .config_dir()
        .map_err(|e| format!("error obtaining config dir: {e}"))?;

    Ok(crate::winapi::taskbar::apps::get_taskbar_apps(
        &icon_cache_path,
        &config_dir,
        50,
        icons_map,
    ))
}

#[tauri::command]
pub fn is_taskbar_autohide() -> bool {
    crate::winapi::taskbar::is_taskbar_autohide()
}
