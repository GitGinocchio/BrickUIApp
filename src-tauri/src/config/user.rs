use std::collections::HashMap;

use serde::{self, Deserialize, Deserializer, Serialize};
use chrono::{DateTime, Utc};
use serde_json::Value;

fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.is_empty()))
}

#[derive(Clone, Debug, Deserialize)]
pub struct SupabaseAuthUser {
    pub id: String,
    pub role: String,
    pub aud: String,

    #[serde(deserialize_with = "empty_string_as_none")]
    pub email: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub phone: Option<String>,

    pub email_confirmed_at: Option<DateTime<Utc>>,
    pub last_sign_in_at: Option<DateTime<Utc>>,
    pub app_metadata: HashMap<String, Value>,
    pub user_metadata: HashMap<String, Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub is_anonymous: bool
}

#[derive(Clone, Debug, Deserialize)]
pub struct SupabaseAuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: Option<u64>,
    pub expires_at: Option<i64>,
    pub token_type: Option<String>,
    pub weak_password: Option<Value>,

    pub user: SupabaseAuthUser
}