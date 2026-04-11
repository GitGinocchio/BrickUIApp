use std::sync::Arc;
use tokio::sync::Mutex;

use tauri::{AppHandle, Manager as _, State};

use crate::{config::settings::Settings, state::generic::BrickUIGenericState};

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn get_settings(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
) -> Result<Settings, String> {
    let state_guard = state.lock().await;
    Ok(state_guard.get_settings().clone())
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn save_settings(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
    app_handle: AppHandle,
    settings: Settings,
) -> Result<(), String> {
    let resolver = app_handle.path();
    let resource_path = resolver
        .resource_dir()
        .map_err(|e| format!("error obtaining config dir: {e}"))?;

    settings.save(&resource_path).await?;

    let mut state_guard = state.lock().await;
    let current_settings = state_guard.get_mut_settings();
    *current_settings = settings;

    Ok(())
}
