use std::{fs, path::PathBuf};

fn copy_templates_if_missing(path: &PathBuf) -> std::io::Result<()> {
    let externals_file = path.join("externals.yml");
    let plugins_file = path.join("plugins.yml");

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

    Ok(())
}

pub struct BrickUIState {
    path: PathBuf
}

impl BrickUIState {
    pub fn new(path: &PathBuf) -> Self {
        fs::create_dir_all(&path).expect("Errore nella creazione della directory di dati");
        fs::create_dir_all(&path.join("bricks")).expect("Errore nella creazione della directory per i widgets");
        fs::create_dir_all(&path.join("walls")).expect("Errore nella creazione della directory per i widgets");

        copy_templates_if_missing(path).expect("Errore durante la creazione dei template");

        Self {
            path: path.clone()
        }
    }

    pub fn get_path(&self) -> PathBuf {
        return self.path.clone();
    }
}