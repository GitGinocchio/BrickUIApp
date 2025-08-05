use std::process::Command;
use windows::Win32::Foundation::HWND;
use winreg::RegKey;
use winreg::enums::*;
use std::io;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn enable_auto_hide_taskbar() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\StuckRects3",
        KEY_READ | KEY_WRITE,
    )?;

    let mut raw_val = key.get_raw_value("Settings")?;
    let mut settings = raw_val.bytes;

    println!("Prima: {:?}", settings);

    if settings.len() > 8 {
        settings[8] = (settings[8] & !0x03) | 0x02;
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "Settings value troppo corto"));
    }

    println!("Dopo: {:?}", settings);

    raw_val.bytes = settings;
    key.set_raw_value("Settings", &raw_val)?;

    // Riavvia explorer per applicare la modifica
    Command::new("taskkill").args(&["/f", "/im", "explorer.exe"]).status()?;
    Command::new("explorer.exe").spawn()?;

    Ok(())
}

fn set_taskbar_auto_hide_flag() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
        KEY_WRITE,
    )?;

    key.set_value("TaskbarAutoHide", &1u32)?;
    Ok(())
}

fn hide_taskbar() {
    use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, ShowWindow, SW_HIDE};
    use windows::core::PCSTR;

    unsafe {
        let class_name = b"Shell_TrayWnd\0".as_ptr();
        let taskbar = FindWindowA(PCSTR(class_name), PCSTR::null()).unwrap_or_else(|e| {
            if e.code().0 == 0 {
                // S_OK trattato erroneamente come errore
                // gestisci come se fosse Ok(HWND(0))
                HWND(std::ptr::null_mut())
            } else {
                panic!("Errore: {:?}", e);
            }
        });

        if !taskbar.0.is_null() {
            #[warn(unused_must_use)]
            ShowWindow(taskbar, SW_HIDE);
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|_app| {
            set_taskbar_auto_hide_flag()?;
            enable_auto_hide_taskbar()?;
            hide_taskbar();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
