use std::sync::Arc;

use tauri::State;
use tokio::sync::Mutex;

use crate::{
    api::{
        auth::{UserIdentity, resend::ResendResponse}, users::User, utils::ApiResponse
    }, 
    state::user::{BrickUIUserState, refresh_session_if_present}
};

#[tauri::command(async)]
pub async fn auth_register(
    _state: State<'_, Arc<Mutex<BrickUIUserState>>>,
    email: String, 
    password: String
) -> Result<ApiResponse<()>, String> {
    match crate::api::auth::register::register(email, password).await? {
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
    match crate::api::auth::login::login(email, password).await? {
        ApiResponse::Success(response) => {
            let mut state_guard = state.lock().await;

            state_guard.update_from_login(&response)?;

            Ok(ApiResponse::Success(()))
        },
        ApiResponse::Error(e) => Ok(ApiResponse::Error(e))
    }
}

#[tauri::command(async)]
pub async fn auth_resend_email(
    _state: State<'_, Arc<Mutex<BrickUIUserState>>>,
    email: String
) -> Result<ApiResponse<ResendResponse>, String> {
    crate::api::auth::resend::resend(email).await
}

#[tauri::command(async)]
pub async fn auth_is_logged_in(
    state: State<'_, Arc<Mutex<BrickUIUserState>>>
) -> Result<bool, String> {
    let state_guard = state.lock().await;
    Ok(state_guard.is_logged_in())
}

#[tauri::command(async)]
pub async fn auth_is_session_expired(
    state: State<'_, Arc<Mutex<BrickUIUserState>>>
) -> Result<bool, String> {
    let state_guard = state.lock().await;
    Ok(state_guard.is_session_expired())
}

#[tauri::command(async)]
pub async fn auth_get_identity(
    state: State<'_, Arc<Mutex<BrickUIUserState>>>
) -> Result<Option<UserIdentity>, String> {
    let state_guard = state.lock().await;
    Ok(state_guard.identity.clone())
}

#[tauri::command(async)]
pub async fn users_get_me(
    state: State<'_, Arc<Mutex<BrickUIUserState>>>
) -> Result<ApiResponse<User>, String> {
    let (access_token, need_refresh) = {
        let guard = state.lock().await;

        if !guard.can_update_user() {
            let user = guard
                .get_user()
                .clone()
                .ok_or_else(|| "No user")?;

            return Ok(ApiResponse::Success(user))
        }

        let access_token = guard.get_access_token().clone();
        let need_refresh = guard.can_refresh_session() && guard.is_session_expired();

        (access_token, need_refresh)
    };

    if need_refresh {
        refresh_session_if_present(state.inner().clone()).await?;
    }

    let access_token = access_token.ok_or("No access token")?;

    match crate::api::users::me::me(&access_token).await? {
        ApiResponse::Success(user) => {
            let mut guard = state.lock().await;
            guard.update_user(user.clone());

            return Ok(ApiResponse::Success(user));
        },
        error => return Ok(error)
    }
}