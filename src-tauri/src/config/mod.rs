pub mod backup;
pub mod settings;
pub mod icons;

use fs_extra::dir::{CopyOptions, copy};
use schemars::{JsonSchema, schema_for};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn load_yaml_async<T>(path: &PathBuf) -> Result<T, String>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let content = tokio::fs::read_to_string(path)
        .await
        .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;

    let data = serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse YAML: {}", e))?;

    Ok(data)
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn save_yaml_async<T>(path: &PathBuf, data: &T) -> Result<(), String>
where
    T: Serialize + std::fmt::Debug,
{
    // Controllo se la struct ha campo `schema`
    let content = if let Some(schema) = extract_schema_field(data) {
        // Serializziamo normalmente
        let yaml_string = serde_yaml::to_string(data)
            .map_err(|e| format!("Error serializing YAML: {e}"))?;

        format!(
            "# yaml-language-server: $schema={0}\n$schema: {0}\n\n{1}",
            schema, yaml_string
        )
    } else {
        serde_yaml::to_string(data)
            .map_err(|e| format!("Error serializing YAML: {e}"))?
    };

    // Preparazione temp file
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
    let tmp_path = path.parent().map(|p| p.join(&tmp_name)).unwrap_or(PathBuf::from(&tmp_name));

    tokio::fs::write(&tmp_path, &content)
        .await
        .map_err(|e| format!("Error while writing temp file {:?}: {e}", tmp_path))?;

    match tokio::fs::rename(&tmp_path, path).await {
        Ok(()) => Ok(()),
        Err(_) => {
            if tokio::fs::try_exists(path).await.map_err(|e| e.to_string())? {
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

// Helper: prova a leggere `schema` usando serde_json Value come intermediario
fn extract_schema_field<T: Serialize>(data: &T) -> Option<String> {
    // Convertiamo in serde_yaml::Value
    let value = serde_yaml::to_value(data).ok()?;
    match value.get("$schema").or_else(|| value.get("schema")) {
        Some(s) => s.as_str().map(|s| s.to_string()),
        None => None,
    }
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn write_schema_if_missing<T>(dir: &PathBuf, filename: &str) -> Result<(), String>
where
    T: JsonSchema + Serialize,
{
    let file_path = dir.join(".schemas").join(filename);
    
    if !tokio::fs::try_exists(&file_path)
        .await
        .map_err(|e| format!("Error checking if {file_path:?} exists: {e}"))? 
    {
        let schema = schema_for!(T);
        let schema_json = serde_json::to_string_pretty(&schema)
            .map_err(|e| format!("Error converting to string pretty: {e}"))?;
        tokio::fs::write(&file_path, schema_json)
            .await
            .map_err(|e| format!("Error writing schema for {filename} at {file_path:?}: {e}"))?;
    }

    Ok(())
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn write_template_if_missing<T>(path: &PathBuf, filename: &str) -> Result<(), String>
where
    T: Serialize + Default,
{
    let file_path = path.join(filename);

    // Se il file esiste già, esci subito
    if tokio::fs::try_exists(&file_path)
        .await
        .map_err(|e| format!("Error checking if {file_path:?} exists: {e}"))?
    {
        return Ok(());
    }

    // Crea l’istanza di default
    let default_value = T::default();

    // Verifica se la struct ha un campo schema
    let content = if let Some(schema) = extract_schema_field(&default_value) {
        let yaml_string = serde_yaml::to_string(&default_value)
            .map_err(|e| format!("Error serializing YAML: {e}"))?;

        format!(
            "# yaml-language-server: $schema={0}\n$schema: {0}\n\n{1}",
            schema, yaml_string
        )
    } else {
        // Fallback: usa il path predefinito come schema
        let yaml_string = serde_yaml::to_string(&default_value)
            .map_err(|e| format!("Error serializing YAML: {e}"))?;

        let schema_filename = filename.replace(".yml", ".schema.json");
        let schema_uri = format!("../../.schemas/{schema_filename}");
        format!("# yaml-language-server: $schema={0}\n$schema: {0}\n\n{1}", schema_uri, yaml_string)
    };

    // Crea la cartella padre se non esiste
    if let Some(parent) = file_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("Failed to create parent dir {:?}: {e}", parent))?;
    }

    // Scrittura atomica tramite temp file + rename
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or_default();

    let tmp_name = match file_path.file_name() {
        Some(name) => format!("{}.{}.tmp", name.to_string_lossy(), ts),
        None => format!("tmp.{}.yml", ts),
    };

    let tmp_path = file_path.parent().map(|p| p.join(&tmp_name)).unwrap_or(PathBuf::from(&tmp_name));

    tokio::fs::write(&tmp_path, &content)
        .await
        .map_err(|e| format!("Error writing temp file {:?}: {e}", tmp_path))?;

    tokio::fs::rename(&tmp_path, &file_path)
        .await
        .map_err(|e| format!("Failed to rename temp file to destination: {e}"))?;

    Ok(())
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn generate_types_if_missing(resource_path: &PathBuf, path: &PathBuf) -> Result<(), String> {
    let resource_path = resource_path.clone();
    let path = path.clone();

    tokio::task::spawn_blocking(move || {
        let mut options = CopyOptions::new();
        options.overwrite = true; // sovrascrive i file se esistono
        options.copy_inside = true; // copia il contenuto della cartella, non la cartella stessa
        options.content_only = true;
        options.skip_exist = true;
        options.depth = 0;

        copy(
            resource_path.join("assets").join("types"),
            path.join("bricks"),
            &options,
        )
        .map_err(|e| format!("Errore durante la duplicazione dei tipi: {e}"))?;

        Ok(())
    })
    .await
    .map_err(|e| format!("spawn_blocking fallito: {e}"))?
}