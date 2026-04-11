pub mod brick;
pub mod props;

use fs_extra::dir::{CopyOptions, copy};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use zip::ZipArchive;
use zip::write::SimpleFileOptions;

use crate::bricks::brick::Brick;

// TODO: Rendere async le sezioni IO di questi metodi (dove possibile)

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn load_brick(path: &PathBuf) -> Result<Brick, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;

    let brick = serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse YAML: {}", e))?;

    Ok(brick)
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn load_bricks(path: &PathBuf) -> Result<Vec<Brick>, String> {
    let bricks_dir = path.join("bricks");

    // Legge la lista delle sottocartelle nella directory bricks
    let entries = fs::read_dir(&bricks_dir)
        .map_err(|e| format!("Failed to read directory {:?}: {}", bricks_dir, e))?;

    let mut bricks = Vec::new();

    for entry_res in entries {
        let entry = entry_res.map_err(|e| format!("Failed to read directory entry: {}", e))?;

        let subdir_path = entry.path();

        // Controlla che sia una directory
        if subdir_path.is_dir() {
            // Costruisce il percorso del file brick.yml all'interno della sottocartella
            let brick_file = subdir_path.join("brick.yml");

            if brick_file.is_file() {
                let brick =
                    load_brick(&brick_file).map_err(|e| format!("Failed to load brick: {}", e))?;

                bricks.push(brick);
            }
        }
    }

    Ok(bricks)
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn create_brick(path: &PathBuf, res_path: &PathBuf, brick: &Brick) -> Result<(), String> {
    let src = res_path.join("assets").join("brick-template");
    let dst = path.join("bricks").join(brick.name.as_str());

    let mut options = CopyOptions::new();
    options.overwrite = true; // sovrascrive i file se esistono
    options.copy_inside = true; // copia il contenuto della cartella, non la cartella stessa
    options.content_only = false;

    copy(src, &dst, &options)
        .map_err(|e| format!("Errore durante la duplicazione del brick: {e}"))?;

    fs::create_dir_all(&dst.join("components")).map_err(|e| format!("Error creating dirs: {e}"))?;
    fs::create_dir_all(&dst.join("data")).map_err(|e| format!("Error creating dirs: {e}"))?;

    save_brick(path, brick)?;

    Ok(())
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn delete_brick(path: &PathBuf, brick: &Brick) -> Result<(), String> {
    let brick_dir = path.join("bricks").join(brick.name.as_str());
    fs::remove_dir_all(brick_dir)
        .map_err(|e| format!("Error while deleting brick directory: {e}"))?;

    Ok(())
}

pub fn rename_brick(path: &PathBuf, old_name: &String, new_name: &String) -> Result<(), String> {
    let old_dir = path.join("bricks").join(old_name);
    let new_dir = path.join("bricks").join(new_name.as_str());

    fs::rename(old_dir, &new_dir).map_err(|e| format!("Failed to rename brick dir: {e}"))?;

    let mut brick = load_brick(&new_dir.join("brick.yml"))?;
    brick.name = new_name.to_string();

    save_brick(&path, &brick)?;

    Ok(())
}

pub fn duplicate_brick(path: &PathBuf, brick: &Brick) -> Result<Brick, String> {
    let brick_name = brick.name.as_str();
    let src = path.join("bricks").join(brick_name);
    let dst = path.join("bricks").join(format!("{brick_name}Copy"));

    // Opzioni di copia
    let mut options = CopyOptions::new();
    options.overwrite = true; // sovrascrive i file se esistono
    options.copy_inside = true; // copia il contenuto della cartella, non la cartella stessa
    options.content_only = false;

    copy(src, &dst, &options)
        .map_err(|e| format!("Errore durante la duplicazione del brick: {e}"))?;

    let mut brick = load_brick(&dst.join("brick.yml"))?;
    brick.name = format!("{brick_name}Copy");
    brick.enabled = false;

    save_brick(&path, &brick)?;

    Ok(brick)
}

pub fn save_brick(path: &PathBuf, brick: &Brick) -> Result<(), String> {
    let brick_dir = path.join("bricks").join(brick.name.as_str());
    fs::create_dir_all(brick_dir).map_err(|e| format!("Failed to create brick dir: {e}"))?;

    let yaml_path = path
        .join("bricks")
        .join(brick.name.clone())
        .join("brick.yml");

    let brick_schema = brick.schema.clone();

    let yaml_string = serde_yaml::to_string(&brick)
        .map_err(|e| format!("Failed to parse brick to YAML: {}", e))?;

    let content = format!(
        "# yaml-language-server: $schema={}\n$schema: {}\n\n{}",
        brick_schema, brick_schema, yaml_string
    );

    fs::write(yaml_path, content).map_err(|e| format!("Failed to save brick: {}", e))?;

    Ok(())
}

pub fn open_brick(path: &PathBuf, brick_name: String) -> Result<(), String> {
    let path = path.join("bricks").join(brick_name);
    use std::os::windows::process::CommandExt;

    std::process::Command::new("cmd")
        .creation_flags(0x08000000)
        .args(&["/C", "start", "/B", "", &path.to_string_lossy()])
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Aggiunge ricorsivamente una cartella allo zip
fn add_directory_to_zip<W: Write + std::io::Seek>(
    path: &PathBuf,
    zip: &mut zip::ZipWriter<W>,
    base_path: &PathBuf,
    options: SimpleFileOptions,
) -> zip::result::ZipResult<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            add_directory_to_zip(&entry_path, zip, base_path, options)?;
        }
    } else {
        // Calcola il percorso relativo rispetto alla cartella base
        let name_in_zip = entry_relative_path(path, base_path);

        zip.start_file(name_in_zip, options)?;
        let mut f = File::open(path)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        zip.write_all(&buffer)?;
    }
    Ok(())
}

/// Restituisce il percorso relativo da inserire nello zip
fn entry_relative_path<'a>(path: &'a PathBuf, base_path: &'a PathBuf) -> String {
    path.strip_prefix(base_path)
        .unwrap()
        .to_string_lossy()
        .replace("\\", "/") // importante per compatibilità ZIP su Windows
}

pub fn pack_brick(path: &PathBuf, brick_name: String, output_path: &PathBuf) -> Result<(), String> {
    let brick_path = path.join("bricks").join(&brick_name);

    let file =
        File::create(output_path).map_err(|e| format!("Error while creating .brick file: {e}"))?;

    let mut zip = zip::ZipWriter::new(file);

    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    add_directory_to_zip(&brick_path, &mut zip, &brick_path, options)
        .map_err(|e| format!("Error while adding dir to zip: {e}"))?;

    zip.finish()
        .map_err(|e| format!("Error while ending zip creation: {e}"))?;

    Ok(())
}

pub fn unpack_brick(input_path: &PathBuf, output_dir: &PathBuf) -> Result<(), String> {
    let file = File::open(input_path).map_err(|e| format!("Error opening .brick file: {e}"))?;

    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("Error reading zip archive: {e}"))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Error accessing file in archive: {e}"))?;

        let outpath = output_dir.join(file.name());

        if file.name().ends_with('/') {
            // È una directory
            fs::create_dir_all(&outpath)
                .map_err(|e| format!("Error creating directory {outpath:?}: {e}"))?;
        } else {
            // Assicurati che la cartella esista
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Error creating parent dirs for {outpath:?}: {e}"))?;
            }

            let mut outfile = File::create(&outpath)
                .map_err(|e| format!("Error creating file {outpath:?}: {e}"))?;

            io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("Error writing to {outpath:?}: {e}"))?;
        }
    }

    Ok(())
}
