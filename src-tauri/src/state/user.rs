use serde_json::Value;

use crate::{
    api::auth::{User, login::LoginResponse}, keyring::{
        load_refresh_token, 
        save_refresh_token
    }
};

#[derive(Clone, Debug)]
pub struct BrickUIUserState {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub expires_at: Option<i64>,
    pub token_type: Option<String>,
    pub weak_password: Option<Value>,

    pub user: Option<User>
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

    pub fn update_from_login(&mut self, response: &LoginResponse) -> Result<(), String> {
        save_refresh_token(&response.refresh_token)?;
        
        self.access_token = Some(response.access_token.clone());
        self.refresh_token = Some(response.refresh_token.clone());
        self.expires_in = response.expires_in;
        self.expires_at = response.expires_at;
        self.token_type = Some(response.token_type.clone());
        self.weak_password = response.weak_password.clone();
        self.user = Some(response.user.clone());

        Ok(())
    }

    pub fn refresh_session(&mut self) -> Result<(), String> {
        Ok(())
    }

    pub fn is_session_expired(&mut self) -> Result<(), String> {
        Ok(())
    }

}