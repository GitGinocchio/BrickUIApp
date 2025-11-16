use std::sync::Arc;
use tokio::sync::Mutex;

use tauri::{AppHandle, Manager, State};

use crate::{state::BrickUIState, winapi::explorer::recents::Recent};

#[tauri::command(async)]
pub async fn get_explorer_recents(
    app_handle: AppHandle,
    state: State<'_, Arc<Mutex<BrickUIState>>>,
) -> Result<Vec<Recent>, String> {
    let mut state_guard = state.lock().await;
    let path = state_guard.get_path();

    let icon_cache_path = path.join("cache").join("icons");
    let icons_map = state_guard.get_mut_icons_map();

    let resolver = app_handle.path();
    let app_data_dir = resolver
        .config_dir()
        .map_err(|e| format!("error obtaining config dir: {e}"))?;

    crate::winapi::explorer::recents::get_explorer_recents(
        &app_data_dir,
        &icon_cache_path,
        icons_map,
        50,
    )
    .await
}
