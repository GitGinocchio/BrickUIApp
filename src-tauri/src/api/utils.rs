use serde::{Deserialize, Deserializer, Serialize, Serializer, de::DeserializeOwned};
use serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub error_code: Option<String>,
    pub msg: Option<String>,
    pub hint: Option<String>,
    pub message: Option<String>
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

    if let Some(code) = value.get("code").map(|v| {
        if let Some(s) = v.as_str() { s.to_string() } 
        else { v.to_string() }
    }) {
        let error_code = value
            .get("error_code")
            .and_then(|v| v.as_str().and_then(|v| Some(v.to_string())));
        let msg = value
            .get("msg")
            .and_then(|v| v.as_str().and_then(|v| Some(v.to_string())));

        let hint = value
            .get("hint")
            .and_then(|v| v.as_str().and_then(|v| Some(v.to_string())));

        let message = value
            .get("message")
            .and_then(|v| v.as_str().and_then(|v| Some(v.to_string())));

        Ok(ApiResponse::Error(ApiError {
            code: code,
            error_code,
            msg,
            hint,
            message
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

#[derive(Debug, Serialize, Deserialize)]
pub struct NonEmptyString(
    #[serde(deserialize_with = "empty_string_as_none")]
    pub Option<String>
);

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum UpdateField<T: Serialize> {
    Set(Option<T>),
    Skip,
}

impl<T: Serialize> Default for UpdateField<T> {
    fn default() -> Self {
        UpdateField::Skip
    }
}

// Serializza solo il valore interno o null, Skip = niente
impl<T: Serialize> Serialize for UpdateField<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            UpdateField::Set(Some(v)) => v.serialize(serializer), // valore normale
            UpdateField::Set(None) => serializer.serialize_none(), // null
            UpdateField::Skip => serializer.serialize_unit(), // viene skippato con Option + skip_serializing_if
        }
    }
}

pub fn is_skip<T: Serialize>(field: &UpdateField<T>) -> bool {
    matches!(field, UpdateField::Skip)
}

#[macro_export]
macro_rules! update_struct {
    ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub struct $name {
            $(
                #[serde(default, skip_serializing_if = "crate::api::utils::is_skip")]
                pub $field: crate::api::utils::UpdateField<$ty>,
            )*
        }
    };
}