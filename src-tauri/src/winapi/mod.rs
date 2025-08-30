pub mod events;
pub mod taskbar;
pub mod window;

use std::ffi::CString;
use windows::Win32::{Foundation::*, System::Registry::*, UI::WindowsAndMessaging::*};
use windows::core::{PCSTR, Result};

pub fn set_snap_flyout(enabled: bool) -> Result<()> {
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
