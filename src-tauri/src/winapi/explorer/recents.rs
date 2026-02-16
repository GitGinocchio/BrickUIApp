use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use windows::Win32::{
    System::Com::CoTaskMemFree,
    UI::Shell::{Common::ITEMIDLIST, SHGetNameFromIDList, SIGDN_NORMALDISPLAY},
};

use crate::{
    state::iconcache::BrickUIconCacheState, 
    winapi::icons::{
        cache::get_icon_from_file, 
        resolver::parse_icon_location
    }
};
use crate::winapi::resolve_lnk;

// Questo non so se ha senso, forse basta risolvere l'lnk
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Recent {
    Directory(RecentFile),
    Url(RecentFile),
    File(RecentFile),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecentFile {
    name: String,
    target: Option<String>,
    icon: String,
    last_accessed: DateTime<Local>,
    created_at: DateTime<Local>,
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
fn resolve_pidl_name(id_list: &lnk::LinkTargetIdList) -> Option<String> {
    let mut raw_bytes = Vec::new();

    for item in id_list.id_list().iter() {
        let size = item.size(); // u16 -> lunghezza totale (compresi i due byte di size)
        let size_le = size.to_le_bytes();
        raw_bytes.extend_from_slice(&size_le);
        raw_bytes.extend_from_slice(item.data());
    }

    // Terminatore ITEMID vuoto (size = 0)
    raw_bytes.extend_from_slice(&[0u8, 0u8]);

    if raw_bytes.len() < 4 {
        return None;
    }

    unsafe {
        let pidl_ptr = raw_bytes.as_ptr() as *const ITEMIDLIST;
        match SHGetNameFromIDList(pidl_ptr, SIGDN_NORMALDISPLAY) {
            Ok(pwstr) if !pwstr.is_null() => {
                let name = pwstr.to_string().ok();
                CoTaskMemFree(Some(pwstr.as_ptr() as *const _));
                name
            }
            _ => None,
        }
    }
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn get_explorer_recents(
    app_data_dir: &PathBuf,
    icon_cache: Arc<RwLock<BrickUIconCacheState>>
) -> Result<Vec<Recent>, String> {
    let recents_dir = app_data_dir.join("Microsoft\\Windows\\Recent");

    let mut recents: Vec<Recent> = Vec::new();

    for entry in fs::read_dir(&recents_dir).map_err(|e| e.to_string())? {
        let icon_cache_clone = icon_cache.clone();
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.extension().map(|e| e == "lnk").unwrap_or(false) {
            // qui servirebbe risolvere il .lnk → percorso reale
            let lnk = match resolve_lnk(&path) {
                Ok(lnk) => lnk,
                Err(e) => {
                    eprintln!("Error while resolving lnk: {e}");
                    continue;
                }
            };

            let target = if let Some(real_target) = lnk.link_target() {
                // target fisico (file o exe)
                Some(real_target)
            } else if let Some(link_info) = lnk.link_info() {
                // target fisico dalla link_info
                link_info.local_base_path().map(|s| s.to_string())
            } else if let Some(id_list) = lnk.linktarget_id_list() {
                // fallback: risolvi PIDL come target
                Some(resolve_pidl_name(id_list).unwrap_or_else(|| {
                    path.file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                }))
            } else {
                // fallback finale: nome file
                Some(
                    path.file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                )
            };

            // 2. Risolvi friendly name per PIDL
            let friendly_name = if let Some(id_list) = lnk.linktarget_id_list() {
                resolve_pidl_name(id_list).unwrap_or_else(|| {
                    path.file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                })
            } else {
                path.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            };

            let accessed = DateTime::<Local>::from_naive_utc_and_offset(
                lnk.header().access_time().datetime(),
                *Local::now().offset(),
            );

            let created = DateTime::<Local>::from_naive_utc_and_offset(
                lnk.header().creation_time().datetime(),
                *Local::now().offset(),
            );

            let icon_path = lnk
                .string_data()
                .icon_location()
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| {
                    if let Some(path) = &target {
                        path.clone()
                    } else {
                        "%SystemRoot%\\System32\\shell32.dll,3".to_string() // icona di file generico
                    }
                });

            let (icon_pathbuf, icon_index) = parse_icon_location(&icon_path);

            let icon = match get_icon_from_file(icon_cache_clone, &icon_pathbuf, icon_index).await? {
                Some(cached_icon_path) => cached_icon_path,
                None => icon_path,
            };

            let recent: Recent;

            if let Some(target_path) = &target {
                let pathbuf = PathBuf::from(target_path);

                if pathbuf.is_dir() {
                    recent = Recent::Directory(RecentFile {
                        name: friendly_name,
                        target: target,
                        icon: icon,
                        last_accessed: accessed,
                        created_at: created,
                    });
                } else {
                    recent = Recent::File(RecentFile {
                        name: friendly_name,
                        target: target,
                        icon: icon,
                        last_accessed: accessed,
                        created_at: created,
                    });
                }
            } else {
                recent = Recent::File(RecentFile {
                    name: friendly_name,
                    target: target,
                    icon: icon,
                    last_accessed: accessed,
                    created_at: created,
                });
            }

            recents.push(recent);
        }
    }

    Ok(recents)
}
