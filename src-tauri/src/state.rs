use std::{fs, path::PathBuf};
use schemars::{schema_for, JsonSchema};

use serde::Serialize;
use tauri::Runtime;
use crate::{
    bricks::{brick::Brick, load_bricks}, 
    config::{
        externals::Externals, 
        load_from_yaml, 
        plugins::Plugins, 
        settings::Settings
    }, 
    overlay_window::window::Overlay
};

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

pub fn write_template_if_missing<T>(path: &PathBuf, filename: &str) -> std::io::Result<()>
where
    T: Serialize + Default,
{
    let file_path = path.join(filename);
    if file_path.exists() { return Ok(()); }

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
        schema_uri,
        schema_uri,
        yaml_string
    );

    // Scrivi il file
    fs::write(file_path, content)?;

    Ok(())
}

fn generate_schemas_if_missing(path: &PathBuf) -> std::io::Result<()> {
    write_schema_if_missing::<Settings>(path, "settings.schema.json")?;
    write_schema_if_missing::<Externals>(path, "externals.schema.json")?;
    write_schema_if_missing::<Plugins>(path, "plugins.schema.json")?;
    write_schema_if_missing::<Brick>(path, "brick.schema.json")?;

    Ok(())
}

fn generate_templates_if_missing(path: &PathBuf) -> std::io::Result<()> {
    write_template_if_missing::<Settings>(path, "settings.yml")?;
    write_template_if_missing::<Plugins>(path, "plugins.yml")?;
    write_template_if_missing::<Externals>(path, "externals.yml")?;

    Ok(())
}

//pub struct BrickUIState<R: Runtime> {
pub struct BrickUIState {
    path: PathBuf,
    settings: Settings,
    bricks: Vec<Brick>
    //overlay: Overlay<R>
}

//impl<R: Runtime> BrickUIState<R> {
impl BrickUIState {
    //pub fn new(path: &PathBuf, overlay: Overlay<R>) -> Self {
    pub fn new(path: &PathBuf) -> Self {
        fs::create_dir_all(&path).expect("Errore nella creazione della directory di dati");
        fs::create_dir_all(&path.join("bricks")).expect("Errore nella creazione della directory per i widgets");
        fs::create_dir_all(&path.join("walls")).expect("Errore nella creazione della directory per i widgets");
        fs::create_dir_all(&path.join(".schemas")).expect("Errore nella creazione della directory per gli schemas");

        generate_schemas_if_missing(path).expect("Errore durante la creazione degli schemas");
        generate_templates_if_missing(path).expect("Errore durante la creazione dei template");

        let bricks = load_bricks(path).expect("Errore durante il caricamento dei bricks");
        let settings = load_from_yaml::<Settings>(&path.join("settings.yml")).expect("Errore durante il caricamento dei settings");

        Self {
            path: path.clone(),
            bricks: bricks,
            settings: settings
            //overlay: overlay
        }
    }

    pub fn get_path(&self) -> PathBuf {
        return self.path.clone();
    }

    pub fn get_bricks(&self) -> &[Brick] {
        &self.bricks
    }

    pub fn get_mut_bricks(&mut self) -> &mut Vec<Brick> {
        &mut self.bricks
    }

    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }

    pub fn get_mut_settings(&mut self) -> &mut Settings {
        &mut self.settings
    }

    /*
    pub fn get_overlay(&self) -> PathBuf {
        return self.path.clone();
    }
    */
}

