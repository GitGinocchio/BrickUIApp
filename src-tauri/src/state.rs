use fs_extra::dir::{CopyOptions, copy};
use schemars::{JsonSchema, schema_for};
use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    bricks::brick::Brick,
    config::{backup::Backup, load_yaml, plugins::Plugins, settings::Settings},
    winapi::{icons::IconsMap, monitor::Monitor},
};
use serde::Serialize;

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

fn generate_schemas_if_missing(path: &PathBuf) -> std::io::Result<()> {
    write_schema_if_missing::<Settings>(path, "settings.schema.json")?;
    write_schema_if_missing::<Plugins>(path, "plugins.schema.json")?;
    write_schema_if_missing::<Brick>(path, "brick.schema.json")?;
    write_schema_if_missing::<Backup>(path, "backup.schema.json")?;

    Ok(())
}

fn generate_templates_if_missing(path: &PathBuf) -> std::io::Result<()> {
    write_template_if_missing::<Settings>(path, "settings.yml")?;
    write_template_if_missing::<Plugins>(path, "plugins.yml")?;
    write_template_if_missing::<IconsMap>(&path.join("cache").join("icons"), "icons.map.yml")?;
    write_template_if_missing::<Backup>(&path, "backup.yml")?;

    Ok(())
}

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

//pub struct BrickUIState<R: Runtime> {
#[derive(Clone)]
pub struct BrickUIState {
    resource_path: PathBuf,
    path: PathBuf,
    settings: Settings,
    backup: Backup,
    bricks: Vec<Brick>, //overlay: Overlay<R>
    icons_map: IconsMap,
    monitors: HashMap<isize, Monitor>
}

//impl<R: Runtime> BrickUIState<R> {
impl BrickUIState {
    //pub fn new(path: &PathBuf, overlay: Overlay<R>) -> Self {
    pub fn new(path: &PathBuf, resource_path: &PathBuf) -> Self {
        fs::create_dir_all(&path)
            .expect("Errore nella creazione della directory di dati");
        fs::create_dir_all(&path.join("bricks"))
            .expect("Errore nella creazione della directory per i widgets");
        fs::create_dir_all(&path.join("walls"))
            .expect("Errore nella creazione della directory per i widgets");
        fs::create_dir_all(&path.join(".schemas"))
            .expect("Errore nella creazione della directory per gli schemas");
        fs::create_dir_all(&path.join("cache").join("icons"))
            .expect("Errore nella creazione della directory per la cache");

        generate_schemas_if_missing(path)
            .expect("Errore durante la creazione degli schemas");
        generate_templates_if_missing(path)
            .expect("Errore durante la creazione dei template");
        generate_types_if_missing(resource_path, path)
            .expect("Errore durante la creazione dei tipi");

        let settings = load_yaml::<Settings>(&path.join("settings.yml"))
            .expect("Errore durante il caricamento dei settings");

        let icons_map = load_yaml::<IconsMap>(&path.join("cache").join("icons").join("icons.map.yml"))
            .expect("Errore durante il caricamento dell'icon map");

        let backup = load_yaml::<Backup>(&path.join("backup.yml"))
            .expect("Errore durante il caricamento del file backup");

        Self {
            resource_path: resource_path.clone(),
            path: path.clone(),
            settings: settings,
            backup: backup,
            bricks: vec![], //overlay: overlay
            icons_map: icons_map,
            monitors: HashMap::new()
        }
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
    
    pub fn get_resource_path(&self) -> &PathBuf {
        &self.resource_path
    }

    pub fn get_bricks(&self) -> &[Brick] {
        &self.bricks
    }

    pub fn get_brick_by_name(&self, name: &str) -> Option<Brick> {
        self.bricks.iter().find(|brick| brick.name == name).cloned()
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

    pub fn get_backup(&self) -> &Backup {
        &self.backup
    }

    pub fn get_mut_backup(&mut self) -> &mut Backup {
        &mut self.backup
    }

    pub fn get_icons_map(&self) -> &IconsMap {
        &self.icons_map
    }

    pub fn get_mut_icons_map(&mut self) -> &mut IconsMap {
        &mut self.icons_map
    }

    /*
    pub fn get_overlay(&self) -> PathBuf {
        return self.path.clone();
    }
    */
}
