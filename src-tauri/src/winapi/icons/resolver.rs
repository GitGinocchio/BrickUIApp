use std::env;
use std::os::windows::ffi::OsStrExt as _;
use std::path::{Path, PathBuf};

use windows::Win32::UI::Shell::{
    SHGSI_ICON, SHGetStockIconInfo, SHSTOCKICONINFO, SIID_DOCNOASSOC, SIID_FOLDER,
};
use windows::Win32::{Storage::FileSystem::GetDriveTypeW, UI::Shell::SHGetFileInfoW};
use windows::{
    Win32::{
        System::WindowsProgramming::DRIVE_CDROM,
        UI::{
            Shell::{
                ExtractIconExW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON, SHSTOCKICONID,
                SIID_APPLICATION, SIID_AUDIOFILES, SIID_DOCASSOC, SIID_DRIVECD, SIID_DRIVEFIXED,
                SIID_IMAGEFILES, SIID_LINK,
                SIID_RECYCLER, SIID_VIDEOFILES, SIID_WORLD, SIID_ZIPFILE,
            },
            WindowsAndMessaging::HICON,
        },
    },
    core::{PCWSTR, PWSTR},
};

use crate::winapi::icons::extractor::hicon_to_png_bytes;

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn get_default_icon(path: &str) -> Option<HICON> {
    let mut sii = SHSTOCKICONINFO {
        cbSize: std::mem::size_of::<SHSTOCKICONINFO>() as u32,
        ..Default::default()
    };

    let p = PathBuf::from(path);

    let id: SHSTOCKICONID = if p.is_dir() {
        if path.ends_with(':') || path.ends_with(":\\") {
            // Drive root → rileva tipo
            let wide: Vec<u16> = path.encode_utf16().chain(Some(0)).collect();
            let drive_type = unsafe { GetDriveTypeW(PWSTR(wide.as_ptr() as *mut _)) };
            match drive_type {
                DRIVE_CDROM => SIID_DRIVECD,
                _ => SIID_DRIVEFIXED,
            }
        } else if path.to_lowercase().contains("$recycle.bin") {
            SIID_RECYCLER // puoi anche usare SIID_RECYCLERFULL se lo sai pieno
        } else {
            SIID_FOLDER
        }
    } else if path.starts_with("http://") || path.starts_with("https://") {
        SIID_WORLD
    } else if let Some(ext) = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
    {
        match ext.as_str() {
            // 📷 immagini
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg" => SIID_IMAGEFILES,
            // 📄 documenti
            "txt" | "log" | "md" | "csv" => SIID_DOCASSOC,
            // 📦 archivi
            "zip" | "rar" | "7z" | "tar" | "gz" => SIID_ZIPFILE,
            // 🎵 audio / 🎥 video
            "mp3" | "wav" | "ogg" | "flac" => SIID_AUDIOFILES,
            "mp4" | "mkv" | "avi" | "mov" => SIID_VIDEOFILES,
            // ⚙️ eseguibili / script
            "exe" | "bat" | "cmd" | "msi" => SIID_APPLICATION,
            // ⚠️ scorciatoie / link
            "lnk" => SIID_LINK,
            _ => SIID_DOCNOASSOC,
        }
    } else {
        SIID_DOCNOASSOC
    };

    let hr = unsafe { SHGetStockIconInfo(id, SHGSI_ICON, &mut sii) };
    if hr.is_ok() { Some(sii.hIcon) } else { None }
}

/// Parse an icon location string like "%SystemRoot%\\System32\\shell32.dll,3"
/// Returns (expanded_path, optional_index)
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn parse_icon_location(s: &str) -> (PathBuf, Option<i32>) {
    // Trim quotes
    let s = s.trim().trim_matches('"').to_string();

    // Expand %VAR% style environment variables (basic support)
    let mut out = String::new();
    let mut i = 0usize;
    let chars: Vec<char> = s.chars().collect();
    while i < chars.len() {
        if chars[i] == '%' {
            if let Some(end) = (i + 1..chars.len()).find(|&j| chars[j] == '%') {
                let var_name: String = chars[i + 1..end].iter().collect();
                if let Some(val) = env::var_os(&var_name) {
                    out.push_str(&val.to_string_lossy());
                } else {
                    out.push('%');
                    out.push_str(&var_name);
                    out.push('%');
                }
                i = end + 1;
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }

    // Now try to parse trailing ",<index>" where <index> is an integer
    if let Some(pos) = out.rfind(',') {
        let (left, right) = out.split_at(pos);
        let maybe_idx = right.trim_start_matches(',').trim();
        if !maybe_idx.is_empty() && maybe_idx.chars().all(|c| c.is_ascii_digit() || c == '-') {
            if let Ok(idx) = maybe_idx.parse::<i32>() {
                return (PathBuf::from(left.to_string()), Some(idx));
            }
        }
    }

    (PathBuf::from(out), None)
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn extract_icon_png_bytes(file_path: &Path, icon_index: Option<i32>) -> Result<Vec<u8>, String> {
    let path_utf16: Vec<u16> = file_path.as_os_str().encode_wide().chain(Some(0)).collect();
    let mut large_icon: HICON = HICON(std::ptr::null_mut());

    if file_path.is_dir() {
        if let Some(h) = get_default_icon(file_path.to_string_lossy().as_ref()) {
            large_icon = h;
        } else {
            return Err("Failed to extract default icon".into());
        }
    } else {
        // If an explicit icon index was supplied (e.g. shell32.dll,3) prefer ExtractIconExW
        if let Some(idx) = icon_index {
            unsafe {
                ExtractIconExW(
                    PCWSTR(path_utf16.as_ptr()),
                    idx,
                    Some(&mut large_icon),
                    None,
                    1,
                );
            }
        } else {
            let mut sfi = SHFILEINFOW::default();
            let file_response = unsafe {
                SHGetFileInfoW(
                    PWSTR(path_utf16.as_ptr() as *mut _),
                    windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES(0),
                    Some(&mut sfi),
                    std::mem::size_of::<SHFILEINFOW>() as u32,
                    SHGFI_ICON | SHGFI_LARGEICON,
                )
            };
            if file_response != 0 && !sfi.hIcon.is_invalid() {
                large_icon = sfi.hIcon;
            } else {
                unsafe {
                    ExtractIconExW(
                        PCWSTR(path_utf16.as_ptr()),
                        0,
                        Some(&mut large_icon),
                        None,
                        1,
                    );
                }
            }
        }
    }

    if large_icon.0.is_null() {
        if let Some(h) = get_default_icon(file_path.to_string_lossy().as_ref()) {
            large_icon = h;
        } else {
            return Err("Failed to extract icon".into());
        }
    }

    // Delegate conversion of HICON -> PNG bytes to helper
    let png_bytes = hicon_to_png_bytes(large_icon)?;

    Ok(png_bytes)
}