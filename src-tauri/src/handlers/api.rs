use std::sync::Arc;

use tauri::State;
use tokio::sync::Mutex;

use crate::{
    api::{
        auth::{login::login, register::register},
        utils::ApiResponse
    }, 
    state::user::BrickUIUserState
};

#[tauri::command(async)]
pub async fn auth_register(
    _state: State<'_, Arc<Mutex<BrickUIUserState>>>,
    email: String, 
    password: String
) -> Result<ApiResponse<()>, String> {
    match register(email, password).await? {
        ApiResponse::Success(_response) => Ok(ApiResponse::Success(())),
        ApiResponse::Error(e) => Ok(ApiResponse::Error(e))
    }
}

#[tauri::command(async)]
pub async fn auth_login(
    state: State<'_, Arc<Mutex<BrickUIUserState>>>,
    email: String,
    password: String
) -> Result<ApiResponse<()>, String> {
    match login(email, password).await? {
        ApiResponse::Success(response) => {
            let mut state_guard = state.lock().await;

            state_guard.update_from_login(&response)?;

            Ok(ApiResponse::Success(()))
        },
        ApiResponse::Error(e) => Ok(ApiResponse::Error(e))
    }
}