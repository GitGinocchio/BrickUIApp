use chrono::{DateTime, Utc};
use serde_json::Value;

use crate::{config::user::{SupabaseAuthResponse, SupabaseAuthUser}, user::auth::{load_refresh_token, save_refresh_token}};

#[derive(Clone, Debug)]
pub struct BrickUIUserState {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub expires_at: Option<i64>,
    pub token_type: Option<String>,
    pub weak_password: Option<Value>,

    pub user: Option<SupabaseAuthUser>,
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
            user: None,
        })
    }

    pub fn update_from_login(&mut self, response: SupabaseAuthResponse) -> Result<(), String> {
        save_refresh_token(&response.refresh_token)?;
        
        self.access_token = Some(response.access_token);
        self.refresh_token = Some(response.refresh_token);
        self.expires_in = response.expires_in;
        self.expires_at = response.expires_at;
        self.token_type = response.token_type;
        self.weak_password = response.weak_password;
        self.user = Some(response.user);

        Ok(())
    }
}