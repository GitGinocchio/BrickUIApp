use std::{collections::HashMap, ffi::OsString, os::windows::ffi::OsStringExt, path::PathBuf};
use windows::{
    Win32::{Foundation::*, System::Threading::*, UI::WindowsAndMessaging::*},
    core::{BOOL, PWSTR},
};

use crate::winapi::{
    icons::{IconsMap, get_icon},
    resolve_lnk,
};

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

#[derive(serde::Serialize)]
pub struct App {
    exe: String,
    icon: Option<String>,
    titles: Vec<String>,
    pinned: bool,
}

fn collect_active_taskbar_apps(
    icon_cache_dir: &PathBuf,
    max_files: usize,
    pinned: Option<HashMap<String, App>>,
    icons_map: &mut IconsMap,
) -> HashMap<String, App> {
    let mut results: HashMap<String, App> = pinned.unwrap_or_else(|| HashMap::new());

    struct EnumContext<'a> {
        max_files: usize,
        icon_cache_dir: &'a PathBuf,
        results: &'a mut HashMap<String, App>,
        icons_map: &'a mut IconsMap,
    }

    let mut context = EnumContext {
        max_files: max_files,
        icon_cache_dir: icon_cache_dir,
        results: &mut results,
        icons_map: icons_map,
    };

    unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let ctx = unsafe { &mut *(lparam.0 as *mut EnumContext) };
        let icon_cache_dir = ctx.icon_cache_dir;
        let results = &mut ctx.results;
        let icons_map = &mut ctx.icons_map;
        let max_files = ctx.max_files;

        if unsafe { !IsWindowVisible(hwnd).as_bool() } {
            return BOOL(1);
        }
        let ex_style = unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) };
        if (ex_style as u32) & WS_EX_TOOLWINDOW.0 != 0 {
            return BOOL(1);
        }

        let title = match get_window_text(hwnd) {
            Some(t) if !t.is_empty() && t != "Program Manager" => t,
            _ => return BOOL(1),
        };

        let mut pid = 0;
        unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };

        if let Some(exe_path) = get_exe_path(pid) {
            let exe_str = exe_path.to_string_lossy().to_string();

            // Da aggiungere altri casi... (Anche Program Manager, vedi sopra)
            if exe_str.ends_with("TextInputHost.exe") {
                return BOOL(1);
            } else if exe_str.ends_with("C:\\Windows\\System32\\ApplicationFrameHost.exe") {
                return BOOL(1);
            }

            match get_icon(&exe_path, icon_cache_dir, icons_map, max_files) {
                Ok(some_icon) => {
                    results
                        .entry(exe_str.clone())
                        .and_modify(|g| g.titles.push(title.clone()))
                        .or_insert(App {
                            exe: exe_str.clone(),
                            icon: some_icon,
                            titles: vec![title.clone()],
                            pinned: false,
                        });
                }
                Err(e) => {
                    eprintln!("Error while obtaining the exe icon: {e}");
                }
            }
        }

        BOOL(1)
    }

    unsafe {
        let _ = EnumWindows(
            Some(enum_windows_proc),
            LPARAM(&mut context as *mut _ as isize),
        );
    }

    results
}

fn collect_pinned_taskbar_apps(
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

                    let icon = get_icon(
                        &PathBuf::from(icon_location),
                        icon_cache_dir,
                        icons_map,
                        max_files,
                    )
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

pub fn get_taskbar_apps(
    icon_cache_dir: &PathBuf,
    config_dir: &PathBuf,
    max_files: usize,
    icons_map: &mut IconsMap,
) -> Vec<App> {
    let results: HashMap<String, App> =
        collect_pinned_taskbar_apps(icon_cache_dir, config_dir, max_files, icons_map);
    collect_active_taskbar_apps(icon_cache_dir, max_files, Some(results), icons_map)
        .into_values()
        .collect()
}

pub fn get_active_taskbar_apps(
    icon_cache_dir: &PathBuf,
    max_files: usize,
    icons_map: &mut IconsMap,
) -> Vec<App> {
    collect_active_taskbar_apps(icon_cache_dir, max_files, None, icons_map)
        .into_values()
        .collect()
}

pub fn get_pinned_taskbar_apps(
    icon_cache_dir: &PathBuf,
    config_dir: &PathBuf,
    max_files: usize,
    icons_map: &mut IconsMap,
) -> Vec<App> {
    collect_pinned_taskbar_apps(icon_cache_dir, config_dir, max_files, icons_map)
        .into_values()
        .collect()
}
