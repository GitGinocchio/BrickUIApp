use std::sync::Arc;

use chrono::{DateTime, Duration, Utc};
use serde_json::Value;
use tokio::sync::Mutex;

use crate::{
    api::{auth::{UserIdentity, login::LoginResponse, refresh::RefreshResponse}, users::User, utils::{ApiError, ApiResponse}}, keyring::{
        load_refresh_token, 
        save_refresh_token
    }
};

#[derive(Clone, Debug)]
pub struct BrickUIUserState {
    access_token: Option<String>,
    refresh_token: Option<String>,
    token_type: Option<String>,
    pub expires_in: Option<u64>,
    pub expires_at: Option<i64>,
    pub weak_password: Option<Value>,

    pub identity: Option<UserIdentity>,

    user: Option<User>,
    pub last_user_update: Option<DateTime<Utc>>
}

impl BrickUIUserState {
    pub fn new() -> Result<Self, String> {
        let refresh_token = load_refresh_token()?;

        Ok(Self {
            access_token: None,
            refresh_token: refresh_token,
            expires_in: None,
            expires_at: None,
            token_type: None,
            weak_password: None,
            identity: None,
            user: None,
            last_user_update: None
        })
    }

    pub fn update_from_login(&mut self, response: &LoginResponse) -> Result<(), String> {
        save_refresh_token(&response.refresh_token)?;
        
        self.access_token = Some(response.access_token.clone());
        self.refresh_token = Some(response.refresh_token.clone());
        self.expires_in = response.expires_in;
        self.expires_at = response.expires_at;
        self.token_type = Some(response.token_type.clone());
        self.weak_password = response.weak_password.clone();
        self.identity = Some(response.user.clone());

        println!("access token: {:#?}", self.access_token);

        Ok(())
    }

    pub fn update_from_refresh(&mut self, response: &RefreshResponse) -> Result<(), String> {
        save_refresh_token(&response.refresh_token)?;
        
        self.access_token = Some(response.access_token.clone());
        self.refresh_token = Some(response.refresh_token.clone());
        self.expires_in = response.expires_in;
        self.expires_at = response.expires_at;
        self.token_type = Some(response.token_type.clone());
        self.identity = Some(response.user.clone());

        println!("access token: {:#?}", self.access_token);

        Ok(())
    }

    pub fn is_session_expired(&self) -> bool {
        if self.access_token.is_some() && let Some(expires_at) = self.expires_at {
            return Utc::now().timestamp() >= expires_at;
        }

        true
    }

    pub fn is_logged_in(&self) -> bool {
        self.access_token.is_some()
    }

    pub fn can_refresh_session(&self) -> bool {
        self.refresh_token.is_some()
    }

    pub fn get_access_token(&self) -> &Option<String> {
        &self.access_token
    }

    pub fn get_user(&self) -> &Option<User> {
        &self.user
    }

    pub fn update_user(&mut self, user: User) {
        self.user = Some(user);
        self.last_user_update = Some(Utc::now());
    }

    pub fn can_update_user(&self) -> bool {
        match self.last_user_update {
            Some(last) => Utc::now() >= last + Duration::minutes(30),
            None => true, // se non c'è mai stato un update, possiamo aggiornare subito
        }
    }
}

pub async fn refresh_session_if_present(state_ref: Arc<Mutex<BrickUIUserState>>) -> Result<(), String> {
    let refresh_token = {
        let state_guard = state_ref.lock().await;
        if let Some(refresh_token) = &state_guard.refresh_token {
            refresh_token.clone()
        }
        else {
            return Ok(())
        }
    };
    
    match crate::api::auth::refresh::refresh(&refresh_token).await? {
        ApiResponse::Error(e) => {
            let mut state_guard = state_ref.lock().await;
            state_guard.refresh_token = None;
            Err(format!("Error refreshing session '{}' ({}): {}", e.error_code, e.code, e.msg))
        },
        ApiResponse::Success(response) => {
            let mut state_guard = state_ref.lock().await;
            state_guard.update_from_refresh(&response)
        }
    }
}