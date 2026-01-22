use std::sync::Arc;
use tokio::sync::Mutex;

use tauri::{AppHandle, Manager, State};

use crate::{state::generic::BrickUIGenericState, winapi::explorer::recents::Recent};

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn get_explorer_recents(
    app_handle: AppHandle,
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
) -> Result<Vec<Recent>, String> {
    let resolver = app_handle.path();
    let path = resolver
        .app_data_dir()
        .map_err(|e| format!("error obtaining config dir: {e}"))?;

    let icon_cache_path = path.join("cache").join("icons");

    let mut state_guard = state.lock().await;
    let icons_map = state_guard.get_mut_icons_map();

    let app_data_dir = path
        .parent()
        .ok_or("Could not get app_data dir")?
        .to_path_buf();

    crate::winapi::explorer::recents::get_explorer_recents(
        &app_data_dir,
        &icon_cache_path,
        icons_map,
        50,
    )
    .await
}
