use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Cursor {

}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Taskbar {

}

#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(description = "A Backup file containing all windows settings before Brick UI customizations")]
pub struct Backup {
    cursors: Vec<Cursor>,
    taskbar: Taskbar
}