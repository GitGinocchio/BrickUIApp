use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use crate::{
    winapi::{
        cursors::{CursorType},
        taskbar::is_taskbar_autohide,
    }
};
use super::save_yaml_async;
use super::load_yaml_async;
use super::write_schema_if_missing; 
use super::write_template_if_missing;

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum DefaultTaskbarAlignment {
    Left,
    Center,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Taskbar {
    // alignment: DefaultTaskbarAlignment,
    autohide: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(
    description = "A Backup file containing all windows settings before Brick UI customizations"
)]
pub struct Backup {
    #[serde(default = "default_schema", rename = "$schema", skip)]
    #[schemars(description = "The JSON Schema version or URI for this Backup definition.")]
    pub schema: String,

    pub cursors: HashMap<CursorType, String>,
    pub taskbar: Taskbar,
}

fn default_schema() -> String {
    "./.schemas/settings.schema.json".to_string()
}

impl Backup {
    pub async fn load(dir: &PathBuf) -> Result<Self, String> {
        write_schema_if_missing::<Backup>(dir, "backup.schema.json").await?;
        write_template_if_missing::<Backup>(dir, "backup.yml").await?;
        load_yaml_async::<Backup>(&dir.join("backup.yml")).await
    }

    pub async fn save(&self, dir: &PathBuf) -> Result<(), String> {
        save_yaml_async(&dir.join("settings.yml"), self).await
    }
}

// TODO: Questo potrebbe rendere piu' lento lo startup, da monitorare
impl Default for Backup {
    fn default() -> Self {
        Self {
            schema: default_schema(),
            cursors: HashMap::new(), //backup_cursors(),
            taskbar: Taskbar {
                autohide: is_taskbar_autohide(),
            },
        }
    }
}
