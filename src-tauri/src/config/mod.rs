pub mod externals;
pub mod plugins;
pub mod settings;

use std::path::PathBuf;
use std::fs;
use serde::de::DeserializeOwned;

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
