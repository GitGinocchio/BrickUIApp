use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod get;
pub mod me;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub gender: Option<String>,
    pub id: String,
    pub locale: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub preferences: Option<serde_json::Value>,
    pub role: Option<String>,
    pub surname: Option<String>,
    pub timezone: Option<String>,
    pub username: Option<String>,
}