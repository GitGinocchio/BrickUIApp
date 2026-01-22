pub mod backup;
pub mod plugins;
pub mod settings;

use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::settings::Settings;

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn load_yaml<T>(path: &PathBuf) -> Result<T, String>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;

    let data =
        serde_yaml::from_str(&content).map_err(|e| format!("Failed to parse YAML: {}", e))?;

    Ok(data)
}

// TODO: Passare una reference di T, evitando di droppare il valore
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn save_yaml<T>(path: &PathBuf, data: T) -> Result<(), String>
where
    T: Serialize + std::fmt::Debug,
{
    let contents =
        serde_yaml::to_string(&data).map_err(|e| format!("Error serializing YAML: {e}"))?;

    // Ensure parent dir exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create parent dir {:?}: {e}", parent))?;
    }

    // Write to temporary file then rename for atomicity
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or_default();
    let tmp_name = match path.file_name() {
        Some(name) => format!("{}.{}.tmp", name.to_string_lossy(), ts),
        None => format!("tmp.{}.yml", ts),
    };
    let tmp_path = match path.parent() {
        Some(parent) => parent.join(tmp_name),
        None => PathBuf::from(tmp_name),
    };

    std::fs::write(&tmp_path, &contents)
        .map_err(|e| format!("Error while writing temp file {:?}: {e}", tmp_path))?;

    // Try rename; on Windows, rename may fail if target exists so remove and retry
    match std::fs::rename(&tmp_path, path) {
        Ok(()) => Ok(()),
        Err(_) => {
            // attempt to remove destination then rename
            if path.exists() {
                std::fs::remove_file(path)
                    .map_err(|er| format!("Failed to remove existing file {:?}: {er}", path))?;
            }
            std::fs::rename(&tmp_path, path)
                .map_err(|er| format!("Failed to rename temp file to destination: {er}"))
        }
    }
}


// TODO: Passare una reference di T, evitando di droppare il valore
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn save_yaml_async<T>(path: &PathBuf, data: T) -> Result<(), String>
where
    T: Serialize + std::fmt::Debug,
{
    let contents =
        serde_yaml::to_string(&data).map_err(|e| format!("Error serializing YAML: {e}"))?;

    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("Failed to create parent dir {:?}: {e}", parent))?;
    }

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or_default();
    let tmp_name = match path.file_name() {
        Some(name) => format!("{}.{}.tmp", name.to_string_lossy(), ts),
        None => format!("tmp.{}.yml", ts),
    };
    let tmp_path = match path.parent() {
        Some(parent) => parent.join(tmp_name),
        None => PathBuf::from(tmp_name),
    };

    tokio::fs::write(&tmp_path, &contents)
        .await
        .map_err(|e| format!("Error while writing temp file {:?}: {e}", tmp_path))?;

    match tokio::fs::rename(&tmp_path, path).await {
        Ok(()) => Ok(()),
        Err(_) => {
            if tokio::fs::try_exists(path)
                .await
                .map_err(|e| e.to_string())?
            {
                tokio::fs::remove_file(path)
                    .await
                    .map_err(|e| format!("Failed to remove existing file {:?}: {e}", path))?;
            }
            tokio::fs::rename(&tmp_path, path)
                .await
                .map_err(|e| format!("Failed to rename temp file to destination: {e}"))
        }
    }
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn save_settings(path: &PathBuf, settings: &Settings) -> Result<(), String> {
    let settings_path = path.join("settings.yml");
    let settings_schema = settings.schema.clone();

    let yaml_string = serde_yaml::to_string(settings)
        .map_err(|e| format!("Failed to parse brick to YAML: {}", e))?;

    let content = format!(
        "# yaml-language-server: $schema={}\n$schema: {}\n\n{}",
        settings_schema, settings_schema, yaml_string
    );

    fs::write(settings_path, content).map_err(|e| format!("Failed to save settings: {}", e))?;

    Ok(())
}
