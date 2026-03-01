use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use tauri::{AppHandle, Manager, State};

use crate::{
    state::{
        iconcache::IconCacheState, 
        user::UserState
    }, 
    winapi::explorer::recents::Recent
};

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn get_explorer_recents(
    app_handle: AppHandle,
    icon_cache_state: State<'_, Arc<RwLock<IconCacheState>>>,
) -> Result<Vec<Recent>, String> {
    let resolver = app_handle.path();
    let path = resolver
        .app_data_dir()
        .map_err(|e| format!("error obtaining config dir: {e}"))?;

    let app_data_dir = path
        .parent()
        .ok_or("Could not get app_data dir")?
        .to_path_buf();

    crate::winapi::explorer::recents::get_explorer_recents(
        &app_data_dir, 
        icon_cache_state.inner().clone()
    ).await
}
