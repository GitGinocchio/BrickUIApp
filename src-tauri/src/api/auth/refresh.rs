use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::{API_BASE_URL, CLIENT, auth::UserIdentity, utils::{ApiResponse, value_to_api_response}};

#[derive(Serialize)]
struct RefreshPayload {
    refresh_token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: Option<u64>,
    pub expires_at: Option<i64>,
    pub token_type: String,

    pub user: UserIdentity,
}

pub async fn refresh(refresh_token: &String) -> Result<ApiResponse<RefreshResponse>, String> {
    let payload = RefreshPayload { refresh_token: refresh_token.clone() };
    let url = format!("{}/auth/refresh", *API_BASE_URL);

    let response: Value = CLIENT
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Error sending post refresh request: {e:#?}"))?
        .json()
        .await
        .map_err(|e| format!("Error deserializing json response: {e:#?}"))?;

    value_to_api_response::<RefreshResponse>(response)
}