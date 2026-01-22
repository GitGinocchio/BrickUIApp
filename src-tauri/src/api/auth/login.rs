use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::{API_BASE_URL, CLIENT, auth::User, utils::{ApiResponse, value_to_api_response}};

#[derive(Serialize)]
struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: Option<u64>,
    pub expires_at: Option<i64>,
    pub token_type: String,

    pub user: User,

    pub weak_password: Option<Value>,
}

pub async fn login(email: String, password: String) -> Result<ApiResponse<LoginResponse>, String> {
    let payload = LoginPayload { email, password };
    let url = format!("{}/auth/login", *API_BASE_URL);

    let response: Value = CLIENT
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Error sending post login request: {e:#?}"))?
        .json()
        .await
        .map_err(|e| format!("Error deserializing json response: {e:#?}"))?;

    value_to_api_response::<LoginResponse>(response)
}