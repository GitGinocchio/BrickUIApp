use serde_json::Value;

use crate::api::{API_BASE_URL, CLIENT, users::User, utils::{ApiError, ApiResponse, value_to_api_response}};

pub async fn me(access_token: &str) -> Result<ApiResponse<User>, String> {
    let url = format!("{}/users/me", *API_BASE_URL);

    let response = CLIENT
        .get(&url)
        .header("Authorization", format!("Bearer {access_token}"))
        .send()
        .await
        .map_err(|e| format!("Error sending post refresh request: {e:#?}"))?
        .json()
        .await
        .map_err(|e| format!("Error deserializing json response: {e:#?}"))?;

    value_to_api_response::<User>(response)
}