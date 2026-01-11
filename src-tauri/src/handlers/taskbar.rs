use std::sync::Arc;
use tokio::sync::Mutex;

use tauri::{AppHandle, Manager, State};

use crate::{
    state::generic::BrickUIGenericState,
    winapi::{self, taskbar::apps::App, taskbar::tray::TrayIcon},
};

#[tauri::command]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn hide_taskbar(app_handle: AppHandle) -> Result<(), String> {
    winapi::taskbar::hide_taskbar(&app_handle)
}

#[tauri::command]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn show_taskbar(app_handle: AppHandle) -> Result<(), String> {
    winapi::taskbar::show_taskbar(&app_handle)
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn get_active_taskbar_apps(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
    app_handle: AppHandle
) -> Result<Vec<App>, String> {
    let resolver = app_handle.path();
    let path = resolver
        .app_data_dir()
        .map_err(|e| format!("error obtaining config dir: {e}"))?;
    let icon_cache_path = path.join("cache").join("icons");

    let mut state_guard = state.lock().await;
    let icons_map = state_guard.get_mut_icons_map();

    Ok(crate::winapi::taskbar::apps::get_active_taskbar_apps(&icon_cache_path, 50, icons_map).await)
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn get_pinned_taskbar_apps(
    app_handle: AppHandle,
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
) -> Result<Vec<App>, String> {
    let resolver = app_handle.path();
    let path = resolver
        .app_data_dir()
        .map_err(|e| format!("error obtaining config dir: {e}"))?;
    let icon_cache_path = path.join("cache").join("icons");
    
    let mut state_guard = state.lock().await;
    let icons_map = state_guard.get_mut_icons_map();

    Ok(crate::winapi::taskbar::apps::get_pinned_taskbar_apps(
        &icon_cache_path,
        &path,
        50,
        icons_map,
    ).await)
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn get_taskbar_apps(
    app_handle: AppHandle,
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
) -> Result<Vec<App>, String> {
    let resolver = app_handle.path();
    let path = resolver
        .app_data_dir()
        .map_err(|e| format!("error obtaining config dir: {e}"))?;

    let icon_cache_path = path.join("cache").join("icons");

    let mut state_guard = state.lock().await;
    let icons_map = state_guard.get_mut_icons_map();

    Ok(crate::winapi::taskbar::apps::get_taskbar_apps(
        &icon_cache_path,
        &path,
        50,
        icons_map,
    ).await)
}

#[tauri::command]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn is_taskbar_autohide() -> bool {
    crate::winapi::taskbar::is_taskbar_autohide()
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn get_tray_icons(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
    app_handle: AppHandle
) -> Result<Vec<TrayIcon>, String> {
    let resolver = app_handle.path();
    let path = resolver
        .app_data_dir()
        .map_err(|e| format!("error obtaining config dir: {e}"))?;
    let icon_cache_path = path.join("cache").join("icons");
    
    let mut state_guard = state.lock().await;
    let icons_map = state_guard.get_mut_icons_map();
    crate::winapi::taskbar::tray::get_tray_icons(&icon_cache_path, icons_map, 50)
}
