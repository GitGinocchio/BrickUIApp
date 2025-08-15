use std::{ffi::OsString, fs::File, io::BufWriter, os::windows::ffi::{OsStrExt, OsStringExt}, path::PathBuf};
use uuid::Uuid;
use windows::{
    core::{BOOL, PCWSTR, PWSTR},
    Win32::{
        Foundation::*, Graphics::Gdi::*, System::{
            Threading::*,
        }, UI::{Shell::ExtractIconExW, WindowsAndMessaging::*}
    },
};
use base64::{engine::general_purpose, Engine as _};
use image::{ImageBuffer, Rgba};

fn get_window_text(hwnd: HWND) -> Option<String> {
    let len = unsafe { GetWindowTextLengthW(hwnd) };
    if len == 0 {
        return None;
    }

    let mut buffer = vec![0u16; (len + 1) as usize];
    let buffer = buffer.as_mut_slice();

    let copied_len = unsafe {
        GetWindowTextW(hwnd, buffer)
    };

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
        if QueryFullProcessImageNameW(handle, PROCESS_NAME_WIN32, PWSTR(buffer.as_mut_ptr()), &mut size).is_ok() {
            CloseHandle(handle);
            Some(PathBuf::from(OsString::from_wide(&buffer[..size as usize])))
        } else {
            CloseHandle(handle);
            None
        }
    }
}

fn extract_icon(exe_path: &PathBuf, output_path: &PathBuf) -> Option<String> {
    use std::ptr::null_mut;

    unsafe {
        // Converte il percorso in UTF-16
        let path_utf16: Vec<u16> = exe_path.as_os_str().encode_wide().chain(Some(0)).collect();

        // Estrae l'icona
        let mut hicon: HICON = HICON(null_mut());
        let icons_loaded = ExtractIconExW(PCWSTR(path_utf16.as_ptr()), 0, Some(&mut hicon), None, 1);
        if icons_loaded == 0 || hicon.0.is_null() {
            return None;
        }

        // Ottiene le informazioni sull'icona
        let mut icon_info = ICONINFO::default();
        if GetIconInfo(hicon, &mut icon_info).is_err() {
            DestroyIcon(hicon);
            return None;
        }

        // Ottiene dimensioni del bitmap
        let mut bmp = BITMAP::default();
        if GetObjectW(HGDIOBJ(icon_info.hbmColor.0), std::mem::size_of::<BITMAP>() as i32, Some(&mut bmp as *mut _ as *mut _)) == 0 {
            DeleteObject(icon_info.hbmColor.into());
            DeleteObject(icon_info.hbmMask.into());
            DestroyIcon(hicon);
            return None;
        }

        let width = bmp.bmWidth as u32;
        let height = bmp.bmHeight as u32;

        // Prepara il buffer dei pixel (RGBA)
        let mut buffer = vec![0u8; (width * height * 4) as usize];

        // Imposta BITMAPINFO per GetDIBits
        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width as i32,
                biHeight: -(height as i32), // negativo per top-down
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
        if GetDIBits(hdc, icon_info.hbmColor, 0, height as u32, Some(buffer.as_mut_ptr() as _), &mut bmi, DIB_RGB_COLORS) == 0 {
            ReleaseDC(None, hdc);
            DeleteObject(icon_info.hbmColor.into());
            DeleteObject(icon_info.hbmMask.into());
            DestroyIcon(hicon);
            return None;
        }
        ReleaseDC(None, hdc);

        let img_buf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, buffer)?;

        // Salva come PNG
        /*
        let filename = format!("icon_{}.png", Uuid::new_v4());
        let path = output_path.join("cache").join("icons").join(filename);

        let file = File::create(&path).ok()?;
        let mut writer = BufWriter::new(file);
        image::DynamicImage::ImageRgba8(img_buf).write_to(&mut writer, image::ImageFormat::Png).ok()?;
        */

        let mut buffer = Vec::new();
        image::DynamicImage::ImageRgba8(img_buf)
            .write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)
            .ok()?;

        // Converte in base64
        let base64_icon = general_purpose::STANDARD.encode(&buffer);

        // A questo punto puoi restituire:
        let data_url = format!("data:image/png;base64,{}", base64_icon);

        // Pulizia risorse GDI
        DeleteObject(icon_info.hbmColor.into());
        DeleteObject(icon_info.hbmMask.into());
        DestroyIcon(hicon);

        Some(data_url)
    }
}

#[derive(serde::Serialize)]
pub struct WindowIcon {
    title: String,
    path: String,
}

pub fn get_taskbar_icons(path: &PathBuf) -> Vec<WindowIcon> {
    let mut results = Vec::new();

    // Struct di contesto da passare alla callback
    struct EnumContext<'a> {
        results: &'a mut Vec<WindowIcon>,
        icon_path: &'a PathBuf,
    }

    let mut context = EnumContext {
        results: &mut results,
        icon_path: path,
    };

    unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let ctx = &mut *(lparam.0 as *mut EnumContext);
        let results = &mut ctx.results;
        let path = ctx.icon_path;

        if !IsWindowVisible(hwnd).as_bool() {
            return BOOL(1);
        }

        let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        if (ex_style as u32) & WS_EX_TOOLWINDOW.0 != 0 {
            return BOOL(1);
        }

        let class_name = {
            let mut buf = [0u16; 256];
            let len = GetClassNameW(hwnd, &mut buf);
            String::from_utf16_lossy(&buf[..len as usize])
        };

        // Gestione UWP/ApplicationFrameWindow
        if class_name == "ApplicationFrameWindow" || class_name == "CabinetWClass" {
            unsafe extern "system" fn enum_child(hwnd: HWND, lparam: LPARAM) -> BOOL {
                let ctx = &mut *(lparam.0 as *mut EnumContext);
                let results = &mut ctx.results;
                if let Some(title) = get_window_text(hwnd) {
                    if !title.is_empty() {
                        /*
                        results.push(WindowIcon {
                            title,
                            path: String::from("<uwp_placeholder>"),
                        });
                        */
                    }
                }
                BOOL(1)
            }
            EnumChildWindows(Some(hwnd), Some(enum_child), LPARAM(ctx as *mut _ as isize));
            return BOOL(1);
        }

        // Titolo finestra
        let title = match get_window_text(hwnd) {
            Some(t) if !t.is_empty() && t != "Program Manager" => t,
            _ => return BOOL(1),
        };

        // PID
        let mut pid = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        // Percorso eseguibile e icona
        if let Some(exe_path) = get_exe_path(pid) {
            if let Some(icon_path) = extract_icon(&exe_path, path) {
                results.push(WindowIcon {
                    title,
                    path: icon_path //.to_string_lossy().to_string(),
                });
            } else {
                /*
                results.push(WindowIcon {
                    title,
                    path: String::from("<no_icon>"),
                });
                */
            }
        } else {
            /*
            results.push(WindowIcon {
                title,
                path: String::from("<no_exe>"),
            });
            */
        }

        BOOL(1)
    }

    unsafe {
        let _ = EnumWindows(Some(enum_windows_proc), LPARAM(&mut context as *mut _ as isize));
    }

    results
}
