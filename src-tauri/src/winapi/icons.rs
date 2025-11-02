use chrono::Local;
use image::{ImageBuffer, Rgba};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    os::windows::ffi::OsStrExt,
    path::{Path, PathBuf}, sync::Arc
};
use tokio::sync::Mutex;
use windows::Win32::UI::Shell::{
    SHGSI_ICON, SHGetStockIconInfo, SHSTOCKICONINFO, SIID_DOCNOASSOC, SIID_FOLDER,
};
use windows::Win32::{Storage::FileSystem::GetDriveTypeW, UI::Shell::SHGetFileInfoW};
use windows::{
    Win32::{
        Graphics::Gdi::{
            BI_RGB, BITMAP, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, DeleteObject, GetDC,
            GetDIBits, GetObjectW, HGDIOBJ, RGBQUAD, ReleaseDC,
        },
        System::WindowsProgramming::DRIVE_CDROM,
        UI::{
            Shell::{
                ExtractIconExW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON, SHSTOCKICONID,
                SIID_APPLICATION, SIID_AUDIOFILES, SIID_DOCASSOC, SIID_DRIVECD, SIID_DRIVEFIXED,
                SIID_DRIVENET, SIID_DRIVERAM, SIID_DRIVEREMOVE, SIID_IMAGEFILES, SIID_LINK,
                SIID_RECYCLER, SIID_VIDEOFILES, SIID_WORLD, SIID_ZIPFILE,
            },
            WindowsAndMessaging::{DestroyIcon, GetIconInfo, HICON, ICONINFO},
        },
    },
    core::{PCWSTR, PWSTR},
};

use crate::config::save_yaml;

// In questo modo viene calcolato solo una volta l'hash del contenuto di un icona
// In base al percorso del file si puo' ottenere l'hash del contenuto (che e' anche il nome del file)
// In questo modo abbiamo un singolo file per ogni icona diversa
#[derive(Serialize, Deserialize, Default, Clone, Debug, JsonSchema)]
pub struct IconsMap {
    #[serde(default = "default_schema", rename = "$schema", skip)]
    #[schemars(description = "The JSON Schema version or URI for this Settings definition.")]
    pub schema: String,

    pub entries: HashMap<String, IconEntry>,
}

fn default_schema() -> String {
    "../../.schemas/icons.map.schema.json".to_string()
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, JsonSchema)]
pub struct IconEntry {
    hash: String,
    created_at: String,
}

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
                _DRIVE_REMOVABLE => SIID_DRIVEREMOVE,
                _DRIVE_REMOTE => SIID_DRIVENET,
                _DRIVE_RAMDISK => SIID_DRIVERAM,
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

/// Ritorna il percorso di un icona nella cartella cache ottenuta da un file
pub fn get_icon(
    file_path: &PathBuf,
    icon_cache_dir: &PathBuf,
    icons_map: &mut IconsMap,
    max_files: usize
) -> Result<Option<String>, String> {
    // Controllo cache esistente
    if let Some(entry) = icons_map.entries.get_mut(&file_path.to_string_lossy().to_string()) {
        let cached_icon_path = icon_cache_dir.join(format!("{}.png", entry.hash));
        if cached_icon_path.exists() {
            return Ok(Some(cached_icon_path.to_string_lossy().to_string()));
        }
    }

    // Estrai l’icona in memoria
    let png_bytes = match extract_icon_png_bytes(file_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed to extract icon for {:?}: {}", file_path, e);
            return Ok(None);
        }
    };

    // Calcola hash per caching
    let mut hasher = Sha256::new();
    hasher.update(&png_bytes);
    let hash = format!("{:x}", hasher.finalize());
    let icon_path = icon_cache_dir.join(format!("{hash}.png"));

    std::fs::create_dir_all(icon_cache_dir)
        .map_err(|e| format!("Error creating cache dir: {e}"))?;
    std::fs::write(&icon_path, &png_bytes)
        .map_err(|e| format!("Error writing PNG file: {e}"))?;

    let now = chrono::Local::now();
    icons_map.entries.insert(
        file_path.to_string_lossy().to_string(),
        IconEntry {
            hash: hash.clone(),
            created_at: now.to_rfc3339(),
        },
    );

    save_yaml(&icon_cache_dir.join("icons.map.yml"), icons_map)?;

    Ok(Some(icon_path.to_string_lossy().to_string()))
}

pub async fn get_icon_async(
    file_path: &PathBuf,
    icon_cache_dir: &PathBuf,
    icons_map: &mut IconsMap,
    max_files: usize
) -> Result<Option<String>, String> {
    let cached_path = {
        if let Some(entry) = icons_map
            .entries
            .get_mut(&file_path.to_string_lossy().to_string())
        {
            let cached_icon_path = icon_cache_dir.join(format!("{}.png", entry.hash));
            if tokio::fs::try_exists(&cached_icon_path).await.map_err(|e| e.to_string())? {
                Some(cached_icon_path)
            } else {
                None
            }
        } else {
            None
        }
    };

    if let Some(path) = cached_path {
        return Ok(Some(path.to_string_lossy().to_string()));
    }

    /*
    let png_bytes = tokio::task::spawn_blocking({
        let file_path = file_path.clone();
        extract_icon_png_bytes(&file_path)?;
    })
    .await
    .map_err(|e| format!("Thread join error: {e}"))??;
    */

    let png_bytes = match extract_icon_png_bytes(&file_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed to extract icon for {:?}: {}", file_path, e);
            return Ok(None);
        }
    };

    let mut hasher = Sha256::new();
    hasher.update(&png_bytes);
    let hash = format!("{:x}", hasher.finalize());
    let icon_path = icon_cache_dir.join(format!("{hash}.png"));

    tokio::fs::create_dir_all(&icon_cache_dir)
        .await
        .map_err(|e| format!("Error creating cache dir: {e}"))?;
    tokio::fs::write(&icon_path, &png_bytes)
        .await
        .map_err(|e| format!("Error writing PNG file: {e}"))?;

    let now = Local::now();
    icons_map.entries.insert(
        file_path.to_string_lossy().to_string(),
        IconEntry {
            hash,
            created_at: now.to_rfc3339(),
        },
    );

    save_yaml(&icon_cache_dir.join("icons.map.yml"), icons_map)?;

    Ok(Some(icon_path.to_string_lossy().to_string()))
}

fn extract_icon_png_bytes(file_path: &Path) -> Result<Vec<u8>, String> {
    let path_utf16: Vec<u16> = file_path.as_os_str().encode_wide().chain(Some(0)).collect();
    let mut large_icon: HICON = HICON(std::ptr::null_mut());

    if file_path.is_dir() {
        if let Some(h) = get_default_icon(file_path.to_string_lossy().as_ref()) {
            large_icon = h;
        } else {
            return Ok(vec![]);
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
                ExtractIconExW(PCWSTR(path_utf16.as_ptr()), 0, Some(&mut large_icon), None, 1);
            }
        }
    }

    if large_icon.0.is_null() {
        return Err("Failed to extract icon".into());
    }

    let mut icon_info = ICONINFO::default();
    unsafe { GetIconInfo(large_icon, &mut icon_info) }.map_err(|_| "GetIconInfo failed".to_string())?;
    let mut bmp = BITMAP::default();

    if unsafe {
        GetObjectW(
            HGDIOBJ(icon_info.hbmColor.0),
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut bmp as *mut _ as *mut _),
        )
    } == 0
    {
        return Err("GetObjectW failed".into());
    }

    let (width, height) = (bmp.bmWidth as u32, bmp.bmHeight as u32);
    let mut buffer = vec![0u8; (width * height * 4) as usize];

    let mut bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width as i32,
            biHeight: -(height as i32),
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0 as u32,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [RGBQUAD::default(); 1],
    };

    let hdc = unsafe { GetDC(None) };
    let res = unsafe {
        GetDIBits(
            hdc,
            icon_info.hbmColor,
            0,
            height,
            Some(buffer.as_mut_ptr() as _),
            &mut bmi,
            DIB_RGB_COLORS,
        )
    };
    unsafe { ReleaseDC(None, hdc) };
    if res == 0 {
        return Err("GetDIBits failed".into());
    }

    for px in buffer.chunks_exact_mut(4) {
        px.swap(0, 2);
    }

    let img_buf: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, buffer).ok_or("Image buffer creation failed")?;
    let mut png_bytes = Vec::new();
    image::DynamicImage::ImageRgba8(img_buf)
        .write_to(
            &mut std::io::Cursor::new(&mut png_bytes),
            image::ImageFormat::Png,
        )
        .map_err(|e| format!("Error writing PNG: {e}"))?;

    unsafe {
        DeleteObject(icon_info.hbmColor.into()).ok().map_err(|e| format!("{e}"))?;
        DeleteObject(icon_info.hbmMask.into()).ok().map_err(|e| format!("{e}"))?;
        DestroyIcon(large_icon).map_err(|e| format!("{e}"))?;
    }

    Ok(png_bytes)
}