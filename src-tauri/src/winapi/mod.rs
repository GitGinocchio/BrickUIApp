pub mod bluetooth;
pub mod com;
pub mod desktop;
pub mod events;
pub mod explorer;
pub mod icons;
pub mod monitor;
pub mod rect;
pub mod sock;
pub mod startmenu;
pub mod taskbar;
pub mod window;
pub mod cursors;

use lnk::ShellLink;
use lnk::encoding::WINDOWS_1252;
use std::path::PathBuf;

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn resolve_lnk(lnk: &PathBuf) -> Result<ShellLink, String> {
    let shortcut =
        ShellLink::open(lnk, WINDOWS_1252).map_err(|e| format!("Could not parse lnk file: {e}"))?;
    Ok(shortcut)
}

/*
// TODO: Vedere se questo metodo funziona e fa qualcosa
pub fn set_snap_flyout(enabled: bool) -> windows::core::Result<()> {
    unsafe {
        let mut hkey = HKEY::default();

        let subkey =
            CString::new("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced")
                .unwrap();
        RegOpenKeyExA(
            HKEY_CURRENT_USER,
            PCSTR(subkey.as_ptr() as *const u8),
            None,
            KEY_SET_VALUE,
            &mut hkey,
        )
        .ok()?;

        let value: u32 = if enabled { 1 } else { 0 };
        let value_bytes = value.to_ne_bytes();
        let value_name = CString::new("EnableSnapAssistFlyout").unwrap();

        RegSetValueExA(
            hkey,
            PCSTR(value_name.as_ptr() as *const u8),
            None,
            REG_DWORD,
            Some(&value_bytes),
        )
        .ok()?;

        RegCloseKey(hkey).ok()?;

        // Notifica Explorer
        let param_utf16: Vec<u16> = "ImmersiveShell\0".encode_utf16().collect();
        SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            WPARAM(0),
            LPARAM(param_utf16.as_ptr() as isize),
            SMTO_ABORTIFHUNG,
            5000,
            None,
        );

        Ok(())
    }
}
*/