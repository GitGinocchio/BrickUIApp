use tauri::{AppHandle, Manager};

use crate::winapi::startmenu::favorites::Favorites;






#[tauri::command]
pub fn open_start_menu() -> Result<(), String> {
    crate::winapi::startmenu::open_start_menu();

    Ok(())
}

#[tauri::command]
pub fn get_start_menu_favorites(app_handle: AppHandle) -> Result<Vec<Favorites>, String> {
    let resolver = app_handle.path();
    let app_data_dir = resolver
        .config_dir()
        .map_err(|e| format!("error obtaining config dir: {e}"))?;

    crate::winapi::startmenu::favorites::get_start_menu_favorites(&app_data_dir)
}