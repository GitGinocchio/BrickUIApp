use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::{API_BASE_URL, CLIENT, utils::{ApiResponse, value_to_api_response}};



#[derive(Debug, Serialize, Deserialize)]
pub struct ResendPayload {
    pub email: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResendResponse {
    #[serde(flatten)]
    extra: HashMap<String, Value>
}

pub async fn resend(email: String) -> Result<ApiResponse<ResendResponse>, String> {
    let payload = ResendPayload { email: email };
    let url = format!("{}/auth/resend", *API_BASE_URL);

    let response: Value = CLIENT
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Error sending post resend request: {e:#?}"))?
        .json()
        .await
        .map_err(|e| format!("Error deserializing json response: {e:#?}"))?;

    value_to_api_response::<ResendResponse>(response)
}