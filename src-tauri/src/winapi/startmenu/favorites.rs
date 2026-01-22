use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::winapi::resolve_lnk;

#[derive(Serialize, Deserialize, Debug)]
pub struct Favorite {
    pub name: String,
    pub path: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Directory {
    pub name: String,
    pub favorites: Vec<Favorite>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Favorites {
    Directory(Directory),
    Favorite(Favorite),
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn get_start_menu_favorites(app_data_dir: &PathBuf) -> Result<Vec<Favorites>, String> {
    let mut results: Vec<Favorites> = Vec::new();

    let user_start_menu = app_data_dir.join("Microsoft\\Windows\\Start Menu\\Programs");

    // Questo non sembra funzionare
    let common_start_menu =
        app_data_dir.join("..\\..\\..\\Default\\Microsoft\\Windows\\Start Menu\\Programs");

    for dir in [common_start_menu, user_start_menu] {
        if dir.exists() {
            scan_directory(&dir, &mut results)?;
        }
    }

    Ok(results)
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
fn scan_directory(dir: &Path, results: &mut Vec<Favorites>) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_dir() {
            let mut sub_favorites = Vec::new();
            scan_directory(&path, &mut sub_favorites)?;

            results.push(Favorites::Directory(Directory {
                name: path.file_name().unwrap().to_string_lossy().to_string(),
                favorites: sub_favorites
                    .into_iter()
                    .filter_map(|fav| match fav {
                        Favorites::Favorite(f) => Some(f),
                        _ => None,
                    })
                    .collect(),
            }));
        } else if let Some(ext) = path.extension() {
            if !ext.eq_ignore_ascii_case("lnk") {
                continue;
            }

            match resolve_lnk(&path) {
                Ok(lnk) => {
                    let exe = if path.ends_with("File Explorer.lnk") {
                        "C:\\Windows\\explorer.exe".into()
                    } else {
                        match lnk.link_target() {
                            Some(exe) => exe,
                            None => {
                                continue;
                            }
                        }
                    };

                    let icon_location = match lnk.string_data().icon_location() {
                        Some(icon_location) => icon_location,
                        None => &exe,
                    };

                    results.push(Favorites::Favorite(Favorite {
                        name: path.file_stem().unwrap().to_string_lossy().to_string(),
                        path: exe.clone(),
                        icon: icon_location.to_string(),
                    }));
                }
                Err(error) => {
                    eprintln!("Error while resolving lnk: {error}");
                    continue;
                }
            }
        }
    }

    Ok(())
}
