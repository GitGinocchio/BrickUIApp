use std::path::PathBuf;

use crate::{
    bricks::brick::Brick,
    config::{backup::Backup, icons::IconsMap, plugins::Plugins, settings::Settings, write_schema_if_missing, write_template_if_missing},
};

pub mod bluetooth;
pub mod generic;
pub mod user;