use std::{collections::HashMap, sync::Arc};

use serde_json::json;
use tauri::{AppHandle, Emitter as _, Manager as _};
use tokio::sync::Mutex;
use url::{Url, form_urlencoded};

use crate::{state::user::UserState, utils::focus_window};

#[derive(Debug, Clone)]
pub struct DeepLinkAuthArgs {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: Option<u64>,
    pub expires_at: Option<i64>,
    pub token_type: String,
}

#[derive(Debug, Clone)]
pub enum DeepLinkRoute {
    Auth { args: DeepLinkAuthArgs },
    Open,
    Unhandled
}

impl DeepLinkRoute {
    pub fn parse(url: &str) -> Result<Self, String> {
        let url = Url::parse(url).map_err(|e| e.to_string())?;
        
        let mut segments: Vec<&str> = url
            .host_str()
            .into_iter()
            .chain(url.path_segments().into_iter().flatten())
            .collect();

        if matches!(segments.last(), Some(&"")) {
            segments.pop();
        }

        println!("segments: {segments:?}");

        match segments.as_slice() {
            ["auth", "confirmed"] => {
                let fragment = match url.fragment() {
                    Some(f) => f,
                    None => return Ok(Self::Unhandled),
                };
        
                let params: HashMap<_, _> = form_urlencoded::parse(fragment.as_bytes())
                    .into_owned()
                    .collect();
                
                // Se mancano access_token o refresh_token → fallback Unhandled
                let access_token = match params.get("access_token") {
                    Some(t) => t.clone(),
                    None => return Ok(Self::Unhandled),
                };

                let refresh_token = match params.get("refresh_token") {
                    Some(t) => t.clone(),
                    None => return Ok(Self::Unhandled),
                };

                let token_type = params.get("token_type").cloned().unwrap_or_else(|| "bearer".to_string());
                let expires_in = params.get("expires_in").and_then(|v| v.parse::<u64>().ok());
                let expires_at = params.get("expires_at").and_then(|v| v.parse::<i64>().ok());

                Ok(Self::Auth { args: DeepLinkAuthArgs {
                    access_token,
                    refresh_token,
                    token_type,
                    expires_in,
                    expires_at,
                }})
            },
            [] => Ok(Self::Open),
            _ => Ok(Self::Unhandled)
        }
    }
}

pub async fn handle_deeplink(app: &AppHandle, url: &str) -> Result<(), String> {
    let route = DeepLinkRoute::parse(url)?;

    println!("route: {route:?}");

    match route {
        DeepLinkRoute::Auth { args } => {
            focus_window(&app, "main").map_err(|e| format!("Error focusing window: {e}"))?; 

            let state = app.state::<Arc<Mutex<UserState>>>();
            let mut guard = state.lock().await;

            if guard.get_access_token().as_ref() != Some(&args.access_token) {
                guard
                    .update_from_deeplink(&args)
                    .map_err(|e| format!("Error updating state from deeplink: {e}"))?;
            }

            app.emit_to("main", "goto", json!({ "view": "user" }))
                .map_err(|e| format!("Error while sending 'goto' event: {e}"))?;
        },
        DeepLinkRoute::Open | _ => {
            focus_window(&app, "main")
                .map_err(|e| format!("Error focusing window: {e}"))?;
        }
    }

    Ok(())
}