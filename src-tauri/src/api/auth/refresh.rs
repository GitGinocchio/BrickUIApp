use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::{API_BASE_URL, CLIENT, utils::{ApiResponse, value_to_api_response}};

#[derive(Serialize)]
struct RefreshPayload {
    refresh_token: String
}

#[derive(Serialize, Deserialize)]
pub struct RefreshResponse {
    #[serde(flatten)]
    extra: HashMap<String, Value>
}

pub async fn refresh(refresh_token: String) -> Result<ApiResponse<RefreshResponse>, String> {
    let payload = RefreshPayload { refresh_token };
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