use std::{fs, os::windows::ffi::OsStrExt, path::{Path, PathBuf}, time::SystemTime};
use image::{ImageBuffer, Rgba};
use sha2::{Digest, Sha256};
use windows::{
    core::PCWSTR, Win32::{
        Graphics::Gdi::{DeleteObject, GetDC, GetDIBits, GetObjectW, ReleaseDC, BITMAP, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HGDIOBJ, RGBQUAD},
        UI::{
            Shell::ExtractIconExW,
            WindowsAndMessaging::{DestroyIcon, GetIconInfo, HICON, ICONINFO},
        },
    }
};

/// Mantiene solo `max_files` file più recenti nella cartella
fn clean_cache(icon_cache_dir: &PathBuf, max_files: usize) -> Result<(), String> {
    let mut entries: Vec<_> = fs::read_dir(icon_cache_dir)
        .map_err(|e| format!("Cannot read cache dir: {e}"))?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let meta = e.metadata().ok()?;
            let modified = meta.modified().ok()?;
            Some((e.path(), modified))
        })
        .collect();

    // Ordina per data (dal più recente al più vecchio)
    entries.sort_by_key(|(_, modified)| *modified);
    entries.reverse();

    if entries.len() > max_files {
        for (path, _) in entries.iter().skip(max_files) {
            let _ = fs::remove_file(path); // ignora errori, cache best-effort
        }
    }

    Ok(())
}

/// Aggiorna la data di ultima modifica del file
fn touch(path: &Path) -> Result<(), String> {
    let now = filetime::FileTime::from_system_time(SystemTime::now());
    filetime::set_file_times(path, now, now)
        .map_err(|e| format!("Cannot update timestamp: {e}"))
}

/// Ritorna il percorso dell'icona di un eseguibile
pub fn get_exe_icon(exe_path: &PathBuf, icon_cache_dir: &PathBuf, max_files: usize) -> Result<Option<String>, String> {
    let exe_name = exe_path
        .file_name()
        .ok_or("Exe path has no file name")?
        .to_string_lossy();

    let mut hasher = Sha256::new();
    hasher.update(exe_name.as_bytes());
    let hash = format!("{:x}", hasher.finalize());

    // Nome file finale: hash.png
    let icon_path = icon_cache_dir.join(format!("{hash}.png"));

    // Se l'immagine e' gia' presente nella cache su disco
    // Non facciamo altri calcoli e ritorniamola subito
    if icon_path.exists() {
        touch(&icon_path)?;
        return Ok(Some(icon_path.to_string_lossy().to_string()));
    }
    
    // Converte in UTF-16
    let path_utf16: Vec<u16> = exe_path.as_os_str().encode_wide().chain(Some(0)).collect();

    // Usa ExtractIconExW per estrarre la prima icona
    let mut large_icon: HICON = HICON(std::ptr::null_mut());
    let icons_loaded = unsafe {
        ExtractIconExW(
            PCWSTR(path_utf16.as_ptr()),
            0,
            Some(&mut large_icon),
            None,
            1,
        ) 
    };

    if icons_loaded == 0 || large_icon.0.is_null() {
        return Ok(None);
    }

    let mut icon_info = ICONINFO::default();
    if unsafe { GetIconInfo(large_icon, &mut icon_info).is_err() } {
        unsafe { DestroyIcon(large_icon).map_err(|e| format!("Error while destroying icon: {e}"))? };
    }

    let mut bmp = BITMAP::default();
    if unsafe { GetObjectW(
        HGDIOBJ(icon_info.hbmColor.0),
        std::mem::size_of::<BITMAP>() as i32,
        Some(&mut bmp as *mut _ as *mut _),
    ) } == 0
    {
        unsafe {
            DeleteObject(icon_info.hbmColor.into())
                .ok()
                .map_err(|e| format!("Error while destroying icon_info.hbmColor: {e}"))?;
            DeleteObject(icon_info.hbmMask.into())
                .ok()
                .map_err(|e| format!("Error while destroying icon_info.hbmColor: {e}"))?;
            DestroyIcon(large_icon)
                .map_err(|e| format!("Error while destroying large_icon: {e}"))?;     
        }
        return Ok(None);
    }

    let width = bmp.bmWidth as u32;
    let height = bmp.bmHeight as u32;
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
    if unsafe { GetDIBits(
        hdc,
        icon_info.hbmColor,
        0,
        height as u32,
        Some(buffer.as_mut_ptr() as _),
        &mut bmi,
        DIB_RGB_COLORS,
    ) } == 0
    {
        unsafe {
            ReleaseDC(None, hdc);
            DeleteObject(icon_info.hbmColor.into())
                .ok()
                .map_err(|e| format!("Error while destroying icon_info.hbmColor: {e}"))?;
            DeleteObject(icon_info.hbmMask.into())
                .ok()
                .map_err(|e| format!("Error while destroying icon_info.hbmColor: {e}"))?;
            DestroyIcon(large_icon)
                .map_err(|e| format!("Error while destroying large_icon: {e}"))?;
        }
        return Ok(None);
    }
    unsafe { ReleaseDC(None, hdc) };

    // BGRA → RGBA
    for px in buffer.chunks_exact_mut(4) {
        let b = px[0];
        px[0] = px[2];
        px[2] = b;
    }

    let img_buf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, buffer)
        .ok_or("Error while creating the image buffer")?;

    let mut png_bytes = Vec::new();
    image::DynamicImage::ImageRgba8(img_buf)
        .write_to(
            &mut std::io::Cursor::new(&mut png_bytes),
            image::ImageFormat::Png,
        )
        .map_err(|e| format!("Error while writing image bytes: {e}"))?;

    //let base64_icon = general_purpose::STANDARD.encode(&png_bytes);

    fs::create_dir_all(icon_cache_dir)
        .map_err(|e| format!("Error creating cache dir: {e}"))?;
    fs::write(&icon_path, png_bytes)
        .map_err(|e| format!("Error writing PNG file: {e}"))?;

    unsafe {
        DeleteObject(icon_info.hbmColor.into())
            .ok()
            .map_err(|e| format!("Error while destroying icon_info.hbmColor: {e}"))?;
        DeleteObject(icon_info.hbmMask.into())
            .ok()
            .map_err(|e| format!("Error while destroying icon_info.hbmColor: {e}"))?;
        DestroyIcon(large_icon)
            .map_err(|e| format!("Error while destroying large_icon: {e}"))?;
    }

    // Pulisci cache se superi il limite di 50 icone
    clean_cache(icon_cache_dir, max_files)?;

    Ok(Some(icon_path.to_string_lossy().to_string()))
}