use std::path::PathBuf;

use crate::{
    bricks::brick::Brick,
    config::{backup::Backup,icons::IconsMap, settings::Settings},
};

#[derive(Debug)]
pub struct BrickUIGenericState {
    settings: Settings,
    backup: Backup,
    // TODO: Forse sarebbe meglio trasformare questo Vec in un HashMap
    bricks: Vec<Brick>
}

impl BrickUIGenericState {
    #[cfg_attr(feature = "profiling", tracing::instrument)]
    pub async fn new(resource_path: &PathBuf) -> Result<Self, String> {
        let settings = Settings::load(resource_path).await?;
        let backup = Backup::load(resource_path).await?;

        Ok(Self {
            settings,
            backup,
            bricks: vec![]
        })
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
}
