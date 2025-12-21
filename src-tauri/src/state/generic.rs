use std::{fs, path::PathBuf};

use crate::{
    bricks::brick::Brick,
    config::{backup::Backup, load_yaml, settings::Settings},
    winapi::icons::IconsMap,
};

use super::generate_schemas_if_missing;
use super::generate_templates_if_missing;
use super::generate_types_if_missing;

#[derive(Clone, Debug)]
pub struct BrickUIGenericState {
    resource_path: PathBuf,
    path: PathBuf,
    settings: Settings,
    backup: Backup,
    bricks: Vec<Brick>,
    icons_map: IconsMap,
}

impl BrickUIGenericState {
    #[cfg_attr(feature = "profiling", tracing::instrument)]
    pub fn new(path: &PathBuf, resource_path: &PathBuf) -> Result<Self, String> {
        fs::create_dir_all(path)
            .map_err(|e| format!("Errore nella creazione della directory di dati: {e}"))?;
        fs::create_dir_all(path.join("bricks")).map_err(|e| {
            format!("Errore nella creazione della directory per i widgets (bricks): {e}")
        })?;
        fs::create_dir_all(path.join("walls")).map_err(|e| {
            format!("Errore nella creazione della directory per i widgets (walls): {e}")
        })?;
        fs::create_dir_all(path.join(".schemas"))
            .map_err(|e| format!("Errore nella creazione della directory per gli schemas: {e}"))?;
        fs::create_dir_all(path.join("cache").join("icons"))
            .map_err(|e| format!("Errore nella creazione della directory per la cache: {e}"))?;

        generate_schemas_if_missing(path)
            .map_err(|e| format!("Errore durante la creazione degli schemas: {e}"))?;
        generate_templates_if_missing(path)
            .map_err(|e| format!("Errore durante la creazione dei template: {e}"))?;
        generate_types_if_missing(resource_path, path)
            .map_err(|e| format!("Errore durante la creazione dei tipi: {e}"))?;

        let settings = load_yaml::<Settings>(&path.join("settings.yml"))
            .map_err(|e| format!("Errore durante il caricamento dei settings: {e}"))?;

        let icons_map =
            load_yaml::<IconsMap>(&path.join("cache").join("icons").join("icons.map.yml"))
                .map_err(|e| format!("Errore durante il caricamento dell'icon map: {e}"))?;

        let backup = load_yaml::<Backup>(&path.join("backup.yml"))
            .map_err(|e| format!("Errore durante il caricamento del file backup: {e}"))?;

        Ok(Self {
            resource_path: resource_path.clone(),
            path: path.clone(),
            settings,
            backup,
            bricks: vec![],
            icons_map: icons_map,
        })
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
    pub fn get_resource_path(&self) -> &PathBuf {
        &self.resource_path
    }

    // Bricks

    pub fn get_bricks(&self) -> &[Brick] {
        &self.bricks
    }

    pub fn get_brick_by_name(&self, name: &str) -> Option<Brick> {
        self.bricks.iter().find(|brick| brick.name == name).cloned()
    }

    pub fn get_mut_bricks(&mut self) -> &mut Vec<Brick> {
        &mut self.bricks
    }

    // Settings

    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }
    pub fn get_mut_settings(&mut self) -> &mut Settings {
        &mut self.settings
    }

    // Backup

    pub fn get_backup(&self) -> &Backup {
        &self.backup
    }
    pub fn get_mut_backup(&mut self) -> &mut Backup {
        &mut self.backup
    }

    // Icons

    pub fn get_icons_map(&self) -> &IconsMap {
        &self.icons_map
    }
    pub fn get_mut_icons_map(&mut self) -> &mut IconsMap {
        &mut self.icons_map
    }
}
