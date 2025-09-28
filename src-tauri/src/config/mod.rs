pub mod plugins;
pub mod settings;
pub mod backup;

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

use crate::config::settings::Settings;

pub fn load_from_yaml<T>(path: &PathBuf) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;

    let data =
        serde_yaml::from_str(&content).map_err(|e| format!("Failed to parse YAML: {}", e))?;

    Ok(data)
}

pub fn save_yaml<T>(path: &PathBuf, data: T) -> Result<(), String>
where
    T: Serialize
{
    let contents = serde_yaml::to_string(&data).unwrap();
    std::fs::write(path, contents).map_err(|e| format!("Error while writing file {path:?}: {e}"))
}

pub fn save_settings(path: &PathBuf, settings: Settings) -> Result<(), String> {
    let settings_path = path.join("settings.yml");
    let settings_schema = settings.schema.clone();

    let yaml_string = serde_yaml::to_string(&settings)
        .map_err(|e| format!("Failed to parse brick to YAML: {}", e))?;

    let content = format!(
        "# yaml-language-server: $schema={}\n$schema: {}\n\n{}",
        settings_schema, settings_schema, yaml_string
    );

    fs::write(settings_path, content).map_err(|e| format!("Failed to save settings: {}", e))?;

    Ok(())
}
