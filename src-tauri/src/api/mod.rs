use std::time::Duration;
use once_cell::sync::Lazy;
use reqwest::Client;

pub mod auth;
pub mod utils;

pub static API_BASE_URL: Lazy<String> = Lazy::new(|| {
    #[cfg(debug_assertions)]
    {
        dotenvy::dotenv().expect("Error loading dotenv file");

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
    Client::builder()
        .user_agent("BrickUIApp/1.0")
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to build HTTP client")
});