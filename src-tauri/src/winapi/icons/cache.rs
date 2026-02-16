use std::{path::PathBuf, sync::Arc};

use chrono::Local;
use sha2::{Digest as _, Sha256};
use tokio::sync::RwLock;

use crate::state::iconcache::BrickUIconCacheState;
use crate::config::icons::IconEntry;
use crate::winapi::icons::resolver::extract_icon_png_bytes;

#[cfg_attr(feature = "profiling", tracing::instrument)]
fn normalize_icon_key(path: &PathBuf, index: Option<i32>) -> String {
    let mut key = match std::fs::canonicalize(path) {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(_) => path.to_string_lossy().to_string(),
    };

    // Rimuove prefisso \\?\
    if let Some(stripped) = key.strip_prefix(r"\\?\") {
        key = stripped.to_string();
    }

    // Windows è case-insensitive
    key = key.to_lowercase();

    if let Some(i) = index {
        format!("{},{}", key, i)
    } else {
        key
    }
}

/// Ritorna il percorso dell’icona dalla cache, o la inserisce se non presente
pub async fn get_or_insert(
    icon_cache: Arc<RwLock<BrickUIconCacheState>>,
    key: String,
    png_bytes: Vec<u8>,
) -> Result<String, String> {
    let (maybe_hash, dir) = {
        let state_guard = icon_cache.read().await;

        let hash = state_guard
            .map
            .entries
            .get(&key)
            .map(|entry| entry.hash.clone());

        (hash, state_guard.dir.clone())
    };

    if let Some(hash) = maybe_hash {
        let cached_path = dir.join(format!("{}.png", hash));

        if tokio::fs::try_exists(&cached_path)
            .await
            .map_err(|e| e.to_string())?
        {
            println!("icon cache hit: {}", cached_path.to_string_lossy().to_string());
            return Ok(cached_path.to_string_lossy().to_string());
        }

        println!("icon cache miss: {}", cached_path.to_string_lossy().to_string());
    }

    let mut hasher = Sha256::new();
    hasher.update(&png_bytes);
    let hash = format!("{:x}", hasher.finalize());
    let icon_path = dir.join(format!("{hash}.png"));

    tokio::fs::write(&icon_path, &png_bytes)
        .await
        .map_err(|e| format!("Error writing PNG file: {e}"))?;

    let mut state_guard = icon_cache.write().await;

    // Aggiorna la mappa
    let now = Local::now();
    state_guard.map.entries.insert(
        key,
        IconEntry {
            hash,
            created_at: now.to_rfc3339(),
        },
    );

    // Salva YAML della cache
    state_guard.map.save(&dir).await?;

    Ok(icon_path.to_string_lossy().to_string())
}

/// Funzione helper: estrae e inserisce direttamente dall’HICON/FilePath
pub async fn get_icon_from_file(
    icon_cache: Arc<RwLock<BrickUIconCacheState>>,
    file_path: &PathBuf,
    icon_index: Option<i32>,
) -> Result<Option<String>, String> {
    let key = normalize_icon_key(file_path, icon_index);
    let file_path_clone = file_path.clone();

    let png_bytes = match tokio::task::spawn_blocking(move || {
        extract_icon_png_bytes(&file_path_clone, icon_index)
    })
    .await
    {
        Ok(Ok(bytes)) => Some(bytes),
        Ok(Err(_)) => None, // icona non esistente
        Err(e) => return Err(format!("spawn_blocking failed: {e}")), // errore critico
    };

    if let Some(bytes) = png_bytes {
        Ok(Some(get_or_insert(icon_cache, key, bytes).await?))
    } else {
        Ok(None)
    }
}