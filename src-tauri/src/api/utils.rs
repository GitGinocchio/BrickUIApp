use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};
use serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: u16,
    pub error_code: String,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum ApiResponse<T: Serialize> {
    Success(T),
    Error(ApiError),
}

/// Converte un `Value` in `ApiResponse<T>`: se c'è "code" diventa `Error`, altrimenti deserializza in T
pub(crate) fn value_to_api_response<T: DeserializeOwned + Serialize>(value: Value) -> Result<ApiResponse<T>, String> {
    if value.is_null() {
        return Err("Received null response from API".to_string());
    }

    if let Some(code) = value.get("code").and_then(|c| c.as_u64()) {
        let error_code = value
            .get("error_code")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let msg = value
            .get("msg")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown API error")
            .to_string();

        Ok(ApiResponse::Error(ApiError {
            code: code as u16,
            error_code,
            msg,
        }))
    } else {
        match serde_json::from_value::<T>(value) {
            Ok(data) => Ok(ApiResponse::Success(data)),
            Err(e) => Err(format!("Failed to deserialize Success payload: {e:#?}")),
        }
    }
}

pub(crate) fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|s| !s.is_empty()))
}