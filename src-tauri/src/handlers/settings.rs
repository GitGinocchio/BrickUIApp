use std::sync::Arc;
use tokio::sync::Mutex;

use tauri::State;

use crate::{config::settings::Settings, state::BrickUIState};

#[tauri::command(async)]
pub async fn get_settings(state: State<'_, Arc<Mutex<BrickUIState>>>) -> Result<Settings, String> {
    let state_guard = state.lock().await;
    Ok(state_guard.get_settings().clone())
}

#[tauri::command(async)]
pub async fn save_settings(
    state: State<'_, Arc<Mutex<BrickUIState>>>,
    settings: Settings,
) -> Result<(), String> {
    let mut state_guard = state.lock().await;

    {
        let current_settings = state_guard.get_mut_settings();
        *current_settings = settings.clone();
    }

    let path = state_guard.get_path();

    crate::config::save_settings(&path, settings)
}
