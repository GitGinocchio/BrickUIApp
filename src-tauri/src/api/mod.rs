use std::time::Duration;
use once_cell::sync::Lazy;
use reqwest::Client;
use tauri::http::{HeaderMap, HeaderValue};

pub mod auth;
pub mod users;
pub mod utils;

pub static API_BASE_URL: Lazy<String> = Lazy::new(|| {
    #[cfg(debug_assertions)]
    {
        if let Err(e) = dotenvy::dotenv() {
            eprintln!("Error loading dotenv file: {e}");
        }

        std::env::var("API_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:5173/api".to_string())
            .trim()
            .to_string()
    }
    #[cfg(not(debug_assertions))]
    {
        "https://brickui.app/api".to_string()
    }
});

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    let mut headers = HeaderMap::new();
    headers.append("Accept", HeaderValue::from_static("application/json"));
    headers.append("Content-Type", HeaderValue::from_static("application/json"));

    Client::builder()
        .user_agent("BrickUIApp/1.0")
        .default_headers(headers)
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to build HTTP client")
});