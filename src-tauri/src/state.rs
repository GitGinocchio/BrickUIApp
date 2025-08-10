use std::{fs, path::PathBuf};

use tauri::Runtime;

use crate::overlay_window::window::Overlay;

fn copy_templates_if_missing(path: &PathBuf) -> std::io::Result<()> {
    let externals_file = path.join("externals.yml");
    let plugins_file = path.join("plugins.yml");
    let settings_file = path.join("settings.yml");
    let bricks_dir = path.join("bricks");
    let brick_file = bricks_dir.join("brick.yml");

    if !externals_file.exists() {
        const TEMPLATE: &str = include_str!("../assets/externals-template.yml");
        fs::write(externals_file, TEMPLATE)?;
        println!("externals.yml created from template");
    }

    if !plugins_file.exists() {
        const TEMPLATE: &str = include_str!("../assets/plugins-template.yml");
        fs::write(plugins_file, TEMPLATE)?;
        println!("plugins.yml created from template");
    }

    if !settings_file.exists() {
        const TEMPLATE: &str = include_str!("../assets/settings-template.yml");
        fs::write(&settings_file, TEMPLATE)?;
        println!("settings.yml created from template");
    }

    if !brick_file.exists() {
        const TEMPLATE: &str = include_str!("../assets/brick-template.yml");
        fs::write(&brick_file, TEMPLATE)?;
        println!("brick.yml created from template");
    }

    Ok(())
}

//pub struct BrickUIState<R: Runtime> {
pub struct BrickUIState {
    path: PathBuf,
    //overlay: Overlay<R>
}

//impl<R: Runtime> BrickUIState<R> {
impl BrickUIState {
    //pub fn new(path: &PathBuf, overlay: Overlay<R>) -> Self {
    pub fn new(path: &PathBuf) -> Self {
        fs::create_dir_all(&path).expect("Errore nella creazione della directory di dati");
        fs::create_dir_all(&path.join("bricks")).expect("Errore nella creazione della directory per i widgets");
        fs::create_dir_all(&path.join("walls")).expect("Errore nella creazione della directory per i widgets");

        copy_templates_if_missing(path).expect("Errore durante la creazione dei template");

        Self {
            path: path.clone(),
            //overlay: overlay
        }
    }

    pub fn get_path(&self) -> PathBuf {
        return self.path.clone();
    }

    /*
    pub fn get_overlay(&self) -> PathBuf {
        return self.path.clone();
    }
    */
}

