use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::winapi::{cursor::{backup_cursors, CursorType}, taskbar::is_taskbar_autohide};

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum DefaultTaskbarAlignment {
    Left,
    Center
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct Taskbar {
    // alignment: DefaultTaskbarAlignment,
    autohide: bool
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "A Backup file containing all windows settings before Brick UI customizations")]
pub struct Backup {
    pub cursors: HashMap<CursorType, String>,
    pub taskbar: Taskbar
}

impl Default for Backup {
    fn default() -> Self {
        Self {
            cursors: backup_cursors(),
            taskbar: Taskbar { 
                autohide: is_taskbar_autohide()
            }
        }
    }
}