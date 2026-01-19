use fs_extra::dir::{CopyOptions, copy};
use schemars::{JsonSchema, schema_for};
use std::{fs, path::PathBuf};

use crate::{
    bricks::brick::Brick,
    config::{backup::Backup, plugins::Plugins, settings::Settings},
    winapi::icons::IconsMap,
};
use serde::Serialize;

pub mod bluetooth;
pub mod generic;
pub mod user;

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn write_schema_if_missing<T>(dir: &PathBuf, filename: &str) -> std::io::Result<()>
where
    T: JsonSchema + Serialize,
{
    let file_path = dir.join(".schemas").join(filename);
    if !file_path.exists() {
        let schema = schema_for!(T);
        let schema_json = serde_json::to_string_pretty(&schema)?;
        fs::write(file_path, schema_json)?;
    }
    Ok(())
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn write_template_if_missing<T>(path: &PathBuf, filename: &str) -> std::io::Result<()>
where
    T: Serialize + Default,
{
    let file_path = path.join(filename);
    if file_path.exists() {
        return Ok(());
    }

    // Crea l’istanza di default
    let default_value = T::default();
    // Serializza in YAML
    let yaml_string = serde_yaml::to_string(&default_value)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let schema_filename = filename.replace(".yml", ".schema.json");

    let schema_uri = format!("../../.schemas/{schema_filename}");

    // Prepara il contenuto con $schema e commenti
    let content = format!(
        "# yaml-language-server: $schema={}\n$schema: {}\n\n{}",
        schema_uri, schema_uri, yaml_string
    );

    // Scrivi il file
    fs::write(file_path, content)?;

    Ok(())
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
fn generate_schemas_if_missing(path: &PathBuf) -> std::io::Result<()> {
    write_schema_if_missing::<Settings>(path, "settings.schema.json")?;
    write_schema_if_missing::<Plugins>(path, "plugins.schema.json")?;
    write_schema_if_missing::<Brick>(path, "brick.schema.json")?;
    write_schema_if_missing::<Backup>(path, "backup.schema.json")?;

    Ok(())
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
fn generate_templates_if_missing(path: &PathBuf) -> std::io::Result<()> {
    write_template_if_missing::<Settings>(path, "settings.yml")?;
    write_template_if_missing::<Plugins>(path, "plugins.yml")?;
    write_template_if_missing::<IconsMap>(&path.join("cache").join("icons"), "icons.map.yml")?;
    write_template_if_missing::<Backup>(&path, "backup.yml")?;

    Ok(())
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
fn generate_types_if_missing(resource_path: &PathBuf, path: &PathBuf) -> Result<(), String> {
    /*if path.join("bricks").join(".types").exists() {
        return Ok(());
    }*/

    // Opzioni di copia
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
}
