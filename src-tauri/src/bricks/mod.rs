pub mod brick;
pub mod props;

use std::{fs, path::PathBuf};

use tauri::State;

use crate::bricks::brick::Brick;
use crate::state::BrickUIState;

pub fn load_brick(path: &PathBuf) -> Result<Brick, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;

    let brick = serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse YAML: {}", e))?;

    Ok(brick)
}

pub fn load_bricks(path: &PathBuf) -> Result<Vec<Brick>, String> {
    let bricks_dir = path.join("bricks");

    // Legge la lista delle sottocartelle nella directory bricks
    let entries = fs::read_dir(&bricks_dir)
        .map_err(|e| format!("Failed to read directory {:?}: {}", bricks_dir, e))?;

    let mut bricks = Vec::new();

    for entry_res in entries {
        let entry = entry_res
            .map_err(|e| format!("Failed to read directory entry: {}", e))?;

        let subdir_path = entry.path();

        // Controlla che sia una directory
        if subdir_path.is_dir() {
            // Costruisce il percorso del file brick.yml all'interno della sottocartella
            let brick_file = subdir_path.join("brick.yml");

            if brick_file.is_file() {
                let brick = load_brick(&brick_file)
                    .map_err(|e| format!("Failed to load brick: {}", e))?;

                bricks.push(brick);
            }
        }
    }

    Ok(bricks)
}

pub fn save_brick(path: &PathBuf, brick: &Brick) -> Result<(),String> {
    let path = path.join("bricks").join(brick.name.clone()).join("brick.yml");

    let brick_schema = brick.schema.clone();

    let yaml_string = serde_yaml::to_string(&brick)
        .map_err(|e| format!("Failed to parse brick to YAML: {}", e))?;

    let content = format!(
        "# yaml-language-server: $schema={}\n$schema: {}\n\n{}",
        brick_schema,
        brick_schema,
        yaml_string
    );

    fs::write(path, content)
        .map_err(|e| format!("Failed to save brick: {}", e))?;

    Ok(())
}

pub fn open_brick(path: &PathBuf, brick_name: String) -> Result<(), String> {
    let path = path.join("bricks").join(brick_name);

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/C", "start", "", &path.to_string_lossy()])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}