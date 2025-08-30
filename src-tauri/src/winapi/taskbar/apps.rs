use base64::{Engine as _, engine::general_purpose};
use image::{ImageBuffer, Rgba};
use std::{
    collections::HashMap,
    ffi::OsString,
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::PathBuf,
};
use windows::{
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        System::Threading::*,
        UI::{Shell::ExtractIconExW, WindowsAndMessaging::*},
    },
    core::{BOOL, PCWSTR, PWSTR},
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
            CloseHandle(handle);
            Some(PathBuf::from(OsString::from_wide(&buffer[..size as usize])))
        } else {
            CloseHandle(handle);
            None
        }
    }
}

pub fn extract_icon(exe_path: &PathBuf) -> Option<String> {
    unsafe {
        // Converte in UTF-16
        let path_utf16: Vec<u16> = exe_path.as_os_str().encode_wide().chain(Some(0)).collect();

        // Usa ExtractIconExW per estrarre la prima icona
        let mut large_icon: HICON = HICON(std::ptr::null_mut());
        let icons_loaded = ExtractIconExW(
            PCWSTR(path_utf16.as_ptr()),
            0,
            Some(&mut large_icon),
            None,
            1,
        );

        if icons_loaded == 0 || large_icon.0.is_null() {
            return None;
        }

        let mut icon_info = ICONINFO::default();
        if GetIconInfo(large_icon, &mut icon_info).is_err() {
            DestroyIcon(large_icon);
            return None;
        }

        let mut bmp = BITMAP::default();
        if GetObjectW(
            HGDIOBJ(icon_info.hbmColor.0),
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut bmp as *mut _ as *mut _),
        ) == 0
        {
            DeleteObject(icon_info.hbmColor.into());
            DeleteObject(icon_info.hbmMask.into());
            DestroyIcon(large_icon);
            return None;
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

        let hdc = GetDC(None);
        if GetDIBits(
            hdc,
            icon_info.hbmColor,
            0,
            height as u32,
            Some(buffer.as_mut_ptr() as _),
            &mut bmi,
            DIB_RGB_COLORS,
        ) == 0
        {
            ReleaseDC(None, hdc);
            DeleteObject(icon_info.hbmColor.into());
            DeleteObject(icon_info.hbmMask.into());
            DestroyIcon(large_icon);
            return None;
        }
        ReleaseDC(None, hdc);

        // BGRA → RGBA
        for px in buffer.chunks_exact_mut(4) {
            let b = px[0];
            px[0] = px[2];
            px[2] = b;
        }

        let img_buf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, buffer)?;

        let mut png_bytes = Vec::new();
        image::DynamicImage::ImageRgba8(img_buf)
            .write_to(
                &mut std::io::Cursor::new(&mut png_bytes),
                image::ImageFormat::Png,
            )
            .ok()?;

        let base64_icon = general_purpose::STANDARD.encode(&png_bytes);

        DeleteObject(icon_info.hbmColor.into());
        DeleteObject(icon_info.hbmMask.into());
        DestroyIcon(large_icon);

        Some(format!("data:image/png;base64,{}", base64_icon))
    }
}

#[derive(serde::Serialize)]
pub struct GroupedIcons {
    exe_path: String,
    icon: String,
    titles: Vec<String>,
}

pub fn get_taskbar_icons(path: &PathBuf) -> Vec<GroupedIcons> {
    let mut results: HashMap<String, GroupedIcons> = HashMap::new();

    struct EnumContext<'a> {
        results: &'a mut HashMap<String, GroupedIcons>,
    }

    let mut context = EnumContext {
        results: &mut results,
    };

    unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let ctx = &mut *(lparam.0 as *mut EnumContext);
        let results = &mut ctx.results;

        if !IsWindowVisible(hwnd).as_bool() {
            return BOOL(1);
        }
        let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        if (ex_style as u32) & WS_EX_TOOLWINDOW.0 != 0 {
            return BOOL(1);
        }

        let title = match get_window_text(hwnd) {
            Some(t) if !t.is_empty() && t != "Program Manager" => t,
            _ => return BOOL(1),
        };

        let mut pid = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        if let Some(exe_path) = get_exe_path(pid) {
            let exe_str = exe_path.to_string_lossy().to_string();

            if let Some(icon_data) = extract_icon(&exe_path) {
                results
                    .entry(exe_str.clone())
                    .and_modify(|g| g.titles.push(title.clone()))
                    .or_insert(GroupedIcons {
                        exe_path: exe_str.clone(),
                        icon: icon_data,
                        titles: vec![title.clone()],
                    });
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

    results.into_values().collect()
}
