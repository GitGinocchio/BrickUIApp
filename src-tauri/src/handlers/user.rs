use std::sync::Arc;

use serde_json::{Value, from_value};
use tauri::State;
use tokio::sync::Mutex;

use crate::{
    config::user::SupabaseAuthResponse, 
    state::user::BrickUIUserState
};




#[tauri::command(async)]
pub async fn auth_complete_login(
    state: State<'_, Arc<Mutex<BrickUIUserState>>>,
    auth_response: Value
) -> Result<(), String> {
    let login_response: SupabaseAuthResponse = from_value(auth_response)
        .map_err(|e| format!("Errore deserializzando login response: {e}"))?;

    let mut state_guard = state.lock().await;
    state_guard.update_from_login(login_response)?;

    println!("{state_guard:?}");

    Ok(())
}