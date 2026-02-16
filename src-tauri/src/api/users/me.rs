use crate::api::{
    API_BASE_URL,
    CLIENT, 
    users::{
        UpdateUser, 
        User
    }, 
    utils::{
        ApiResponse,
        value_to_api_response
    }
};

pub async fn get_me(access_token: &str) -> Result<ApiResponse<User>, String> {
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

    println!("response: {response:#?}");

    value_to_api_response::<User>(response)
}

pub async fn update_me(access_token: &str, updated: UpdateUser) -> Result<ApiResponse<User>, String> {
    let url = format!("{}/users/me", *API_BASE_URL);

    let response = CLIENT
        .patch(&url)
        .header("Authorization", format!("Bearer {access_token}"))
        .json(&updated)
        .send()
        .await
        .map_err(|e| format!("Error sending post refresh request: {e:#?}"))?
        .json()
        .await
        .map_err(|e| format!("Error deserializing json response: {e:#?}"))?;

    println!("response: {response:#?}");

    value_to_api_response::<User>(response)
}