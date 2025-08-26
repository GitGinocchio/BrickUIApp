pub mod plugins;
pub mod settings;

use std::path::PathBuf;
use std::fs;
use serde::de::DeserializeOwned;

use crate::config::settings::Settings;

pub fn load_from_yaml<T>(path: &PathBuf) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;

    let data = serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse YAML: {}", e))?;

    Ok(data)
}

pub fn save_settings(path: &PathBuf, settings: Settings) -> Result<(), String> {
    let settings_path = path.join("settings.yml");
    let settings_schema = settings.schema.clone();

    let yaml_string = serde_yaml::to_string(&settings)
        .map_err(|e| format!("Failed to parse brick to YAML: {}", e))?;
    
    let content = format!(
        "# yaml-language-server: $schema={}\n$schema: {}\n\n{}",
        settings_schema,
        settings_schema,
        yaml_string
    );

    fs::write(settings_path, content)
        .map_err(|e| format!("Failed to save settings: {}", e))?;

    Ok(())
}
