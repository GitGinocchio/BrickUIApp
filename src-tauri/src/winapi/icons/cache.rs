use std::{path::PathBuf};

use chrono::Local;
use sha2::{Digest as _, Sha256};

use crate::config::icons::{IconEntry, IconsMap};

#[derive(Debug, Clone)]
pub struct IconCache {
    dir: PathBuf,
    map: IconsMap,
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
fn normalize_icon_key(path: &PathBuf, index: Option<i32>) -> String {
    let key = match std::fs::canonicalize(path) {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(_) => path.to_string_lossy().to_string(),
    };

    if let Some(i) = index {
        format!("{},{}", key, i)
    } else {
        key
    }
}

impl IconCache {
    pub async fn new(dir: &PathBuf, map: IconsMap) -> Result<Self, String> {
        let dir = dir.join("cache").join("icons");
        Ok(Self { dir: dir, map })
    }

    /// Ritorna il percorso dell’icona dalla cache, o la inserisce se non presente
    pub async fn get_or_insert(
        &mut self,
        key: String,
        png_bytes: Vec<u8>,
    ) -> Result<String, String> {
        // Controlla se esiste già nella mappa e sul filesystem
        if let Some(entry) = self.map.entries.get(&key) {
            let cached_path = self.dir.join(format!("{}.png", entry.hash));
            if tokio::fs::try_exists(&cached_path)
                .await
                .map_err(|e| e.to_string())?
            {
                return Ok(cached_path.to_string_lossy().to_string());
            }
        }

        let mut hasher = Sha256::new();
        hasher.update(&png_bytes);
        let hash = format!("{:x}", hasher.finalize());
        let icon_path = self.dir.join(format!("{hash}.png"));

        tokio::fs::write(&icon_path, &png_bytes)
            .await
            .map_err(|e| format!("Error writing PNG file: {e}"))?;

        // Aggiorna la mappa
        let now = Local::now();
        self.map.entries.insert(
            key.clone(),
            IconEntry {
                hash,
                created_at: now.to_rfc3339(),
            },
        );

        // Salva YAML della cache
        self.map.save(&self.dir).await?;

        Ok(icon_path.to_string_lossy().to_string())
    }

    /// Funzione helper: estrae e inserisce direttamente dall’HICON/FilePath
    pub async fn get_icon_from_file(
        &mut self,
        file_path: &PathBuf,
        icon_index: Option<i32>,
    ) -> Result<Option<String>, String> {
        use crate::winapi::icons::resolver::extract_icon_png_bytes;

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
            Ok(Some(self.get_or_insert(key, bytes).await?))
        } else {
            Ok(None)
        }
    }
}
