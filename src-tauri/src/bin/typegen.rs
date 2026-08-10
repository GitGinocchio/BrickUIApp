use std::{fs, path::PathBuf};

use brickui_lib::{api::users::User, bricks::brick::Brick, config::settings::Settings};
use ts_rs::{Config, TS};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = manifest_dir.join("../src/interfaces/generated");
    fs::create_dir_all(&output_dir)?;

    let config = Config::default().with_out_dir(output_dir);

    Brick::export_all(&config)?;
    Settings::export_all(&config)?;
    User::export_all(&config)?;

    Ok(())
}
