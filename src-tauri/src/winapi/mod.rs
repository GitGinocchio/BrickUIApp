pub mod cursor;
pub mod events;
pub mod explorer;
pub mod icons;
pub mod startmenu;
pub mod taskbar;
pub mod window;
pub mod monitor;
pub mod workarea;

use lnk::ShellLink;
use lnk::encoding::WINDOWS_1252;
use serde::{Deserialize, Serialize};
use std::ffi::CString;
use std::path::PathBuf;
use std::ptr::null_mut;
use windows::Win32::{Foundation::*, System::Registry::*, UI::WindowsAndMessaging::*};
use windows::core::PCSTR;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rect {
    pub left: i32, 
    pub right: i32,
    pub top: i32,
    pub bottom: i32
}

impl From<RECT> for Rect {
    fn from(r: RECT) -> Self {
        Self {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}

impl From<Rect> for RECT {
    fn from(r: Rect) -> Self {
        RECT {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}

impl From<&RECT> for Rect {
    fn from(r: &RECT) -> Self {
        Self {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}

impl From<&Rect> for RECT {
    fn from(r: &Rect) -> Self {
        RECT {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}

pub fn resolve_lnk(lnk: &PathBuf) -> Result<ShellLink, String> {
    let shortcut = ShellLink::open(lnk, WINDOWS_1252).unwrap();
    Ok(shortcut)
}

pub fn set_snap_flyout(enabled: bool) -> windows::core::Result<()> {
    unsafe {
        let mut hkey = HKEY(null_mut());

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
