use std::{ffi::OsString, fs::File, io::BufWriter, os::windows::ffi::{OsStrExt, OsStringExt}, path::PathBuf};
use uuid::Uuid;
use windows::{
    core::{BOOL, PCWSTR},
    Win32::{
        Foundation::*, Graphics::Gdi::*, System::{
            Diagnostics::ToolHelp::*,
            Threading::*,
        }, UI::{Shell::ExtractIconExW, WindowsAndMessaging::*}
    },
};
use image::{ImageBuffer, Rgba};
use tauri::command;

fn get_window_text(hwnd: HWND) -> Option<String> {
    let len = unsafe { GetWindowTextLengthW(hwnd) };
    if len == 0 {
        return None;
    }

    let mut buffer = vec![0u16; (len + 1) as usize];
    unsafe {
        GetWindowTextW(hwnd, PWSTR(buffer.as_mut_ptr()), len + 1);
    }

    Some(String::from_utf16_lossy(&buffer[..len as usize]))
}

fn get_exe_path(pid: u32) -> Option<PathBuf> {
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);
        if handle.is_invalid() {
            return None;
        }

        let mut buffer = vec![0u16; 260];
        let mut size = buffer.len() as u32;
        if QueryFullProcessImageNameW(handle, 0, PWSTR(buffer.as_mut_ptr()), &mut size).as_bool() {
            CloseHandle(handle);
            Some(PathBuf::from(OsString::from_wide(&buffer[..size as usize])))
        } else {
            CloseHandle(handle);
            None
        }
    }
}

fn extract_icon(path: &PathBuf) -> Option<PathBuf> {
    unsafe {
        let path_utf16: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
        let mut hicon: HICON = HICON(std::ptr::null_mut());
        let icons_loaded = ExtractIconExW(PCWSTR(path_utf16.as_ptr()), 0, Some(&mut hicon), None, 1);

        if icons_loaded == 0 || hicon.0 == std::ptr::null_mut() {
            return None;
        }

        // Estrae l’icona in bitmap
        let hdc = GetDC(Some(HWND(std::ptr::null_mut())));
        let mut icon_info = ICONINFO::default();
        GetIconInfo(hicon, &mut icon_info);

        let mut bmp = BITMAP::default();
        GetObjectW(HGDIOBJ(icon_info.hbmColor.0), std::mem::size_of::<BITMAP>() as i32, Some(&mut bmp as *mut _ as *mut _));

        let width = bmp.bmWidth as u32;
        let height = bmp.bmHeight as u32;

        let mut buffer = vec![0u8; (width * height * 4) as usize];
        let success = GetBitmapBits(icon_info.hbmColor, buffer.len() as i32, buffer.as_mut_ptr() as _);

        let img_buf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, buffer)?;

        let temp_dir = std::env::temp_dir();
        let filename = format!("icon_{}.png", Uuid::new_v4());
        let path = temp_dir.join(filename);

        let file = File::create(&path).ok()?;
        let writer = BufWriter::new(file);
        image::DynamicImage::ImageRgba8(img_buf).flipv().write_to(writer, image::ImageOutputFormat::Png).ok()?;

        Some(path)
    }
}

#[derive(serde::Serialize)]
struct WindowIcon {
    title: String,
    icon_path: String,
}

#[command]
pub fn get_taskbar_icons() -> Vec<WindowIcon> {
    let mut results = Vec::new();

    unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let results = &mut *(lparam.0 as *mut Vec<WindowIcon>);

        if !IsWindowVisible(hwnd).as_bool() || GetParent(hwnd).unwrap().0 != HWND(std::ptr::null_mut()).0 {
            return BOOL(1);
        }

        let mut pid = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        if let Some(title) = get_window_text(hwnd) {
            if title.is_empty() || title == "Program Manager" {
                return BOOL(1);
            }

            if let Some(exe_path) = get_exe_path(pid) {
                if let Some(icon_path) = extract_icon(&exe_path) {
                    results.push(WindowIcon {
                        title,
                        icon_path: icon_path.to_string_lossy().to_string(),
                    });
                }
            }
        }

        BOOL(1)
    }

    unsafe {
        EnumWindows(Some(enum_windows_proc), LPARAM(&mut results as *mut _ as isize));
    }

    results
}
