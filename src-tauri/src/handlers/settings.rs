use std::sync::{Arc, Mutex};

use tauri::{State};

use crate::{config::settings::Settings, state::BrickUIState};



#[tauri::command]
pub fn get_settings(state: State<'_, Arc<Mutex<BrickUIState>>>) -> Result<Settings, String> {
    let state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    Ok(state_guard.get_settings().clone())
}

#[tauri::command]
pub fn save_settings(
    state: State<'_, Arc<Mutex<BrickUIState>>>,
    settings: Settings,
) -> Result<(), String> {
    let mut state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;

    {
        let current_settings = state_guard.get_mut_settings();
        *current_settings = settings.clone();
    }

    let path = state_guard.get_path();

    crate::config::save_settings(&path, settings)
}