use std::ffi::{CStr, CString};
use windows::Win32::Foundation::{FALSE, TRUE};
use windows::Win32::System::Threading::GetCurrentProcessId;
use windows::Win32::UI::WindowsAndMessaging::{GetWindowThreadProcessId, IsWindowVisible};
use windows::core::PCSTR;
use windows::{
    Win32::{
        Foundation::{CloseHandle, HANDLE, HWND, LPARAM},
        System::Threading::{
            OpenProcess, PROCESS_NAME_FORMAT, PROCESS_QUERY_LIMITED_INFORMATION,
            QueryFullProcessImageNameW,
        },
        UI::WindowsAndMessaging::{
            EnumChildWindows, EnumWindows, FindWindowA, FindWindowExA, GetClassNameA, GetClassNameW,
        },
    },
    core::{BOOL, PWSTR},
};

pub fn dump_children(hwnd: HWND) -> Result<(), String> {
    fn recurse(hwnd: HWND, depth: usize) -> Result<(), String> {
        unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
            let depth = lparam.0 as usize;
            let indent = "  ".repeat(depth);

            let mut buf = [0u8; 256];
            let len = unsafe { GetClassNameA(hwnd, &mut buf) };
            if len > 0 {
                if let Ok(name) = std::str::from_utf8(&buf[..len as usize]) {
                    println!("{indent}Child HWND={:?} Class={}", hwnd, name);
                }
            }

            // Richiama ricorsivamente per i figli di questa finestra
            if let Err(e) = recurse(hwnd, depth + 1) {
                eprintln!("{indent}  [Error] {e}");
            }

            BOOL(1) // continua enumerazione
        }

        unsafe {
            EnumChildWindows(Some(hwnd), Some(enum_proc), LPARAM(depth as isize))
                .ok()
                .map_err(|e| format!("Error enumerating child windows: {e}"))?;
        }
        Ok(())
    }

    recurse(hwnd, 1)
}

pub fn get_process_exe_path(pid: u32) -> Result<String, String> {
    let handle: HANDLE = unsafe {
        OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid)
            .map_err(|e| format!("Error obtaining process handle: {e}"))?
    };

    if handle.is_invalid() {
        return Err("Error invalid process handle".into());
    }

    let mut buf = vec![0u16; 260];
    let mut size = buf.len() as u32;

    unsafe {
        QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_FORMAT(0),
            PWSTR(buf.as_mut_ptr()),
            &mut size,
        )
        .map_err(|e| format!("Error obtaining FullProcessImageNameW: {e}"))?
    };

    unsafe {
        CloseHandle(handle).map_err(|e| format!("Error obtaining FullProcessImageNameW: {e}"))?
    };

    String::from_utf16(&buf[..size as usize]).map_err(|e| format!("Utf16 Error: {e}"))
}

pub fn is_explorer_process(pid: u32) -> Result<bool, String> {
    let exe_path =
        get_process_exe_path(pid).map_err(|e| format!("Error obtaining process exe path: {e}"))?;

    let exe_name = exe_path.rsplit('\\').next().unwrap_or("").to_lowercase();

    Ok(exe_name == "explorer.exe")
}

/// Ricerca di una classe figlia
pub fn find_childw(parent: HWND, class_name: &str) -> Result<Option<HWND>, String> {
    let target_c = CString::new(class_name).map_err(|e| format!("Invalid class name: {e}"))?;

    #[repr(C)]
    struct CallbackData {
        target_ptr: *const i8,
        found: Option<HWND>,
    }

    let mut data = CallbackData {
        target_ptr: target_c.as_ptr(),
        found: None,
    };

    unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let data: &mut CallbackData = unsafe { &mut *(lparam.0 as *mut CallbackData) };

        // Buffer per il nome della classe
        let mut buf: [u8; 256] = [0; 256];
        let len = unsafe { GetClassNameA(hwnd, &mut buf) };

        if len > 0 {
            let found_c = unsafe { CStr::from_ptr(buf.as_ptr() as *const i8) };
            let target_c = unsafe { CStr::from_ptr(data.target_ptr) };

            // Stampa il nome della finestra trovata
            println!("Child HWND={:?} class={:?}", hwnd, found_c);

            if found_c == target_c {
                data.found = Some(hwnd);
                return BOOL(0); // stop enumerating
            }
        }

        BOOL(1) // continue
    }

    unsafe {
        EnumChildWindows(
            Some(parent),
            Some(enum_proc),
            LPARAM(&mut data as *mut CallbackData as isize),
        )
        .ok()
        .map_err(|e| format!("Error enumerating child windows: {e}"))?;
    }

    Ok(data.found)
}

/// Trova tutte le Shell_TrayWnd attive
pub fn find_all_shell_traywnds() -> Result<Vec<HWND>, String> {
    let mut results = Vec::new();

    extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        unsafe {
            let mut buf = [0u8; 256];
            let len = GetClassNameA(hwnd, &mut buf);
            let class_name = std::str::from_utf8(&buf[..len as usize]).unwrap_or("");

            println!("{class_name:?}");

            if len > 0 && class_name == "Shell_TrayWnd" {
                (*(lparam.0 as *mut Vec<HWND>)).push(hwnd);
            }
            BOOL(1)
        }
    }

    unsafe {
        EnumWindows(Some(enum_proc), LPARAM(&mut results as *mut _ as isize))
            .map_err(|e| format!("Error enumerating windows: {e}"))?
    }

    Ok(results)
}

// Ricerca ricorsiva di una classe figlia
pub fn find_childw_recursive(parent: HWND, target: &str) -> Result<Option<HWND>, String> {
    let result: Option<HWND> = None;

    extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        unsafe {
            let (target, result_ptr) = &mut *(lparam.0 as *mut (&str, Option<HWND>));
            let mut buf = [0u8; 256];
            let len = GetClassNameA(hwnd, &mut buf);
            if len > 0 {
                let name = std::str::from_utf8(&buf[..len as usize]).unwrap_or("");
                if name == *target {
                    (*result_ptr) = Some(hwnd);
                    return BOOL(0);
                }
            }

            EnumChildWindows(Some(hwnd), Some(enum_proc), LPARAM(lparam.0))
                .ok()
                .map_err(|e| format!("Error enumerating child windows: {e}"));

            BOOL(1)
        }
    }
    let mut data = (target, result);

    unsafe {
        EnumChildWindows(
            Some(parent),
            Some(enum_proc),
            LPARAM(&mut data as *mut _ as isize),
        )
        .ok()
        .map_err(|e| format!("Error enumerating child windows: {e}"))?
    }

    Ok(data.1)
}

pub fn find_tray_toolbar_window() -> Result<HWND, String> {
    unsafe {
        // 🔹 1️⃣ Windows 11 path
        /*
        let win11_hwnd = FindWindowA(PCSTR(b"SystemTray_Main\0".as_ptr()), None);
        if let Ok(hwnd) = win11_hwnd {
            if !hwnd.is_invalid() {
                println!("[+] Found Windows 11 SystemTray_Main");
                dump_children(hwnd)?;
                return Ok(hwnd);
            }
        }
        */

        // 🔹 2️⃣ Windows 10 (classic) path
        let shell_tray = FindWindowA(PCSTR(b"Shell_TrayWnd\0".as_ptr()), None)
            .map_err(|e| format!("Error finding Shell_TrayWnd: {e}"))?;
        if shell_tray.is_invalid() {
            return Err("Shell_TrayWnd not found".into());
        }

        dump_children(shell_tray)?;

        let tray_notify = FindWindowExA(
            Some(shell_tray),
            None,
            PCSTR(b"TrayNotifyWnd\0".as_ptr()),
            None,
        )
        .map_err(|e| format!("Error finding TrayNotifyWnd: {e}"))?;
        if tray_notify.is_invalid() {
            return Err("TrayNotifyWnd not found".into());
        }

        dump_children(tray_notify)?;

        let sys_pager = FindWindowExA(Some(tray_notify), None, PCSTR(b"SysPager\0".as_ptr()), None)
            .map_err(|e| format!("Error finding SysPager: {e}"))?;
        if sys_pager.is_invalid() {
            return Err("SysPager not found".into());
        }

        dump_children(sys_pager)?;

        let toolbar = FindWindowExA(
            Some(sys_pager),
            None,
            PCSTR(b"ToolbarWindow32\0".as_ptr()),
            None,
        )
        .map_err(|e| format!("Error finding ToolbarWindow32: {e}"))?;
        if toolbar.is_invalid() {
            return Err("ToolbarWindow32 not found".into());
        }

        dump_children(toolbar)?;

        println!("[+] Found Windows 10 Tray Toolbar");
        Ok(toolbar)
    }
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn get_window_class(hwnd: HWND) -> Option<String> {
    unsafe {
        // buffer per la classe (massimo 256 caratteri)
        let mut class_name = [0u16; 256];
        let len = GetClassNameW(hwnd, &mut class_name);

        if len == 0 {
            // fallita
            None
        } else {
            // converte da UTF-16 a Rust String
            Some(String::from_utf16_lossy(&class_name[..len as usize]))
        }
    }
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn is_tauri_window(hwnd: HWND) -> Result<bool, String> {
    get_window_class(hwnd).map_or(Ok(false), |class_name| Ok(class_name.starts_with("Tauri")))
}

pub fn find_chrome_widget(hwnd: HWND) -> Option<HWND> {
    unsafe extern "system" fn enum_proc(
        hwnd: HWND,
        lparam: LPARAM,
    ) -> BOOL {
        let target = unsafe { &mut *(lparam.0 as *mut Option<HWND>) };

        let mut class_name = [0u16; 256];
        let len = unsafe { GetClassNameW(hwnd, &mut class_name) };

        if len > 0 {
            let name = String::from_utf16_lossy(&class_name[..len as usize]);
            if name == "Chrome_RenderWidgetHostHWND" {
                *target = Some(hwnd);
                return FALSE; // fermati
            }
        }

        unsafe { EnumChildWindows(
            Some(hwnd), 
            Some(enum_proc), 
            lparam
        ) };
        TRUE
    }

    let mut result = None;
    unsafe {
        EnumChildWindows(
            Some(hwnd),
            Some(enum_proc),
            LPARAM(&mut result as *mut _ as isize),
        );
    }

    result
}
