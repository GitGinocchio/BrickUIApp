pub mod brick;
pub mod props;

use fs_extra::dir::{copy, CopyOptions};
use std::{fs, path::PathBuf};

use crate::bricks::brick::Brick;

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

pub fn create_brick(path: &PathBuf, brick: &Brick) -> Result<(), String> {
    save_brick(path, brick)?;

    let brick_data_dir = path.join("bricks").join(brick.name.as_str()).join("data");
    fs::create_dir_all(brick_data_dir).map_err(|e| format!("Failed to create brick data dir: {e}"))?;

    Ok(())
}

pub fn delete_brick(path: &PathBuf, brick: &Brick) -> Result<(), String> {
    let brick_dir = path.join("bricks").join(brick.name.as_str());
    fs::remove_dir_all(brick_dir).map_err(|e| format!("Error while deleting brick directory: {e}"))?;

    Ok(())
}

pub fn rename_brick(path: &PathBuf, old_name: String, new_name: String) -> Result<(), String> {
    let old_dir = path.join("bricks").join(old_name);
    let new_dir = path.join("bricks").join(new_name.as_str());

    fs::rename(old_dir, &new_dir).map_err(|e| format!("Failed to rename brick dir: {e}"))?;

    let mut brick = load_brick(&new_dir.join("brick.yml"))?;
    brick.name = new_name;

    save_brick(&path, &brick)?;

    Ok(())
}

pub fn duplicate_brick(path: &PathBuf, brick: Brick) -> Result<(), String> {
    let brick_name = brick.name.as_str();
    let src = path.join("bricks").join(brick_name);
    let dst = path.join("bricks").join(format!("{brick_name}-copy"));

    // Opzioni di copia
    let mut options = CopyOptions::new();
    options.overwrite = true; // sovrascrive i file se esistono
    options.copy_inside = true; // copia il contenuto della cartella, non la cartella stessa
    options.content_only = false;

    copy(src, &dst, &options).map_err(|e| format!("Errore durante la duplicazione del brick: {e}"))?;

    let mut brick = load_brick(&dst.join("brick.yml"))?;
    brick.name = format!("{brick_name}-copy");

    save_brick(&path, &brick)?;

    Ok(())
}

pub fn save_brick(path: &PathBuf, brick: &Brick) -> Result<(),String> {
    let brick_dir = path.join("bricks").join(brick.name.as_str());
    fs::create_dir_all(brick_dir).map_err(|e| format!("Failed to create brick dir: {e}"))?;

    let yaml_path = path.join("bricks").join(brick.name.clone()).join("brick.yml");

    let brick_schema = brick.schema.clone();

    let yaml_string = serde_yaml::to_string(&brick)
        .map_err(|e| format!("Failed to parse brick to YAML: {}", e))?;

    let content = format!(
        "# yaml-language-server: $schema={}\n$schema: {}\n\n{}",
        brick_schema,
        brick_schema,
        yaml_string
    );

    fs::write(yaml_path, content)
        .map_err(|e| format!("Failed to save brick: {}", e))?;

    Ok(())
}

pub fn open_brick(path: &PathBuf, brick_name: String) -> Result<(), String> {
    let path = path.join("bricks").join(brick_name);

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;

        std::process::Command::new("cmd") 
            .creation_flags(0x08000000) 
            .args(&["/C", "start", "/B", "", &path.to_string_lossy()]) 
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