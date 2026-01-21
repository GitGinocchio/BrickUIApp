use serde::Serialize;
use serde_json::Value;

use crate::api::auth::User;
use crate::api::{API_BASE_URL, CLIENT};
use crate::api::utils::{ApiResponse, value_to_api_response};

#[derive(Serialize)]
struct RegisterPayload {
    email: String,
    password: String,
}

pub async fn register(email: String, password: String) -> Result<ApiResponse<User>, String> {
    let payload = RegisterPayload { email, password };
    let url = format!("{}/auth/register/classic", *API_BASE_URL);

    let response: Value = CLIENT
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Error sending post register request: {e:#?}"))?
        .json()
        .await
        .map_err(|e| format!("Error deserializing json response: {e:#?}"))?;

    value_to_api_response::<User>(response)
}