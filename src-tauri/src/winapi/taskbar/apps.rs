use std::{collections::HashMap, ffi::OsString, os::windows::ffi::OsStringExt, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use futures::stream::{FuturesUnordered, StreamExt};
use windows::{
    Win32::{Foundation::*, System::Threading::*, UI::WindowsAndMessaging::*},
    core::{BOOL, PWSTR},
};

use crate::winapi::{
    icons::{IconsMap, get_icon, get_icon_async},
    resolve_lnk,
};

#[derive(serde::Serialize)]
pub struct App {
    exe: String,
    icon: Option<String>,
    titles: Vec<String>,
    pinned: bool,
}

fn get_window_text(hwnd: HWND) -> Option<String> {
    let len = unsafe { GetWindowTextLengthW(hwnd) };
    if len == 0 {
        return None;
    }

    let mut buffer = vec![0u16; (len + 1) as usize];
    let buffer = buffer.as_mut_slice();

    let copied_len = unsafe { GetWindowTextW(hwnd, buffer) };

    if copied_len == 0 {
        return None;
    }

    Some(String::from_utf16_lossy(&buffer[..copied_len as usize]))
}

fn get_exe_path(pid: u32) -> Option<PathBuf> {
    unsafe {
        let handle = match OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid) {
            Ok(handle) => handle,
            Err(e) => {
                eprintln!("Errore durante l'ottenimento del percorso dell'eseguibile: {e}");
                return None;
            }
        };

        if handle.is_invalid() {
            return None;
        }

        let mut buffer = vec![0u16; 260];
        let mut size = buffer.len() as u32;
        if QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            PWSTR(buffer.as_mut_ptr()),
            &mut size,
        )
        .is_ok()
        {
            CloseHandle(handle)
                .map_err(|e| format!("Error while closing handle: {e}"))
                .ok()?;
            Some(PathBuf::from(OsString::from_wide(&buffer[..size as usize])))
        } else {
            CloseHandle(handle)
                .map_err(|e| format!("Error while closing handle: {e}"))
                .ok()?;
            None
        }
    }
}

async fn collect_active_taskbar_apps(
    icon_cache_dir: &PathBuf,
    max_files: usize,
    pinned: Option<HashMap<String, App>>,
    icons_map: &mut IconsMap,
) -> HashMap<String, App> {
    let mut results = pinned.unwrap_or_default();
    let mut exe_list: Vec<(String, String)> = vec![];

    unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let list = unsafe { &mut *(lparam.0 as *mut Vec<(String, String)>) };

        if !unsafe { IsWindowVisible(hwnd).as_bool() } {
            return BOOL(1);
        }

        if let Some(title) = get_window_text(hwnd) {
            let mut pid = 0;
            unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
            if let Some(exe_path) = get_exe_path(pid) {
                list.push((exe_path.to_string_lossy().to_string(), title));
            }
        }

        BOOL(1)
    }

    unsafe {
        let _ = EnumWindows(
            Some(enum_windows_proc),
            LPARAM(&mut exe_list as *mut _ as isize),
        );
    }

    let icon_cache_dir = icon_cache_dir.clone();
    let mut tasks = FuturesUnordered::new();

    for (exe, title) in exe_list.into_iter() {
        let icon_cache_dir = icon_cache_dir.clone();
        let mut icons_map_clone = icons_map.clone();

        tasks.push(tokio::spawn(async move {
            // Calcola icona senza lock
            let icon = crate::winapi::icons::get_icon_async(
                &PathBuf::from(&exe),
                None,
                &icon_cache_dir,
                &mut icons_map_clone,
                max_files,
            )
            .await
            .ok()
            .flatten();

            (exe, title, icon)
        }));
    }

    while let Some(task) = tasks.next().await {
        if let Ok((exe, title, icon)) = task {
            results
                .entry(exe.clone())
                .and_modify(|a| a.titles.push(title.clone()))
                .or_insert(App {
                    exe,
                    icon,
                    titles: vec![title],
                    pinned: false,
                });
        }
    }

    results
}

async fn collect_pinned_taskbar_apps(
    icon_cache_dir: &PathBuf,
    config_dir: &PathBuf,
    max_files: usize,
    icons_map: &mut IconsMap,
) -> HashMap<String, App> {
    let mut results: HashMap<String, App> = HashMap::new();

    let pinned_dir =
        config_dir.join("Microsoft/Internet Explorer/Quick Launch/User Pinned/TaskBar");

    for entry in std::fs::read_dir(pinned_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map(|e| e == "lnk").unwrap_or(false) {
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

                    let (icon_pathbuf, icon_index) = crate::winapi::icons::parse_icon_location(icon_location);

                    let icon = crate::winapi::icons::get_icon_async(
                        &icon_pathbuf,
                        icon_index,
                        icon_cache_dir,
                        icons_map,
                        max_files,
                    )
                    .await
                    .ok()
                    .flatten();

                    results.entry(exe.clone()).or_insert(App {
                        exe,
                        icon,
                        titles: vec![],
                        pinned: true,
                    });
                }
                Err(error) => {
                    eprintln!("Error while trying to resolve the lnk: {error}");
                }
            }
        }
    }

    // Poi trasformi in Vec<App>
    results
}

pub async fn get_taskbar_apps(
    icon_cache_dir: &PathBuf,
    config_dir: &PathBuf,
    max_files: usize,
    icons_map: &mut IconsMap,
) -> Vec<App> {
    let results: HashMap<String, App> =
        collect_pinned_taskbar_apps(icon_cache_dir, config_dir, max_files, icons_map)
        .await;
    collect_active_taskbar_apps(icon_cache_dir, max_files, Some(results), icons_map)
        .await
        .into_values()
        .collect()
}

pub async fn get_active_taskbar_apps(
    icon_cache_dir: &PathBuf,
    max_files: usize,
    icons_map: &mut IconsMap,
) -> Vec<App> {
    collect_active_taskbar_apps(icon_cache_dir, max_files, None, icons_map)
        .await
        .into_values()
        .collect()
}

pub async fn get_pinned_taskbar_apps(
    icon_cache_dir: &PathBuf,
    config_dir: &PathBuf,
    max_files: usize,
    icons_map: &mut IconsMap,
) -> Vec<App> {
    collect_pinned_taskbar_apps(icon_cache_dir, config_dir, max_files, icons_map)
        .await
        .into_values()
        .collect()
}
