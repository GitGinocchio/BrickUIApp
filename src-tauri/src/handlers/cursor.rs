use std::sync::Arc;
use tokio::sync::Mutex;

use tauri::{AppHandle, Manager as _, State};

use crate::state::generic::BrickUIGenericState;

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn hide_all_cursors(
    app_handle: AppHandle
) -> Result<(), String> {
    let resolver = app_handle.path();
    let resource_path = resolver.resource_dir().map_err(|e| format!("Error resolving appdata dir: {e}"))?;

    let cursor_path = resource_path.join("assets").join("transparent.cur");

    crate::winapi::cursor::hide_cursors(cursor_path.to_string_lossy().to_string().as_str())
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn restore_all_cursors(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
) -> Result<(), String> {
    let state_guard = state.lock().await;
    let backup = state_guard.get_backup();

    crate::winapi::cursor::restore_cursors(&backup.cursors)
}
