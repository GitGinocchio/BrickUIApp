use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, SMTO_ABORTIFHUNG, SendMessageTimeoutW, WM_SETTINGCHANGE,
};
use windows::core::PCWSTR;

pub fn refresh_desktop_icons() -> Result<(), String> {
    let progman = unsafe {
        FindWindowW(
            PCWSTR::from_raw("Progman\0".encode_utf16().collect::<Vec<u16>>().as_ptr()),
            None,
        )
        .ok()
    };

    if let Some(hwnd) = progman
        && !hwnd.is_invalid()
    {
        let mut result: usize = 0;
        unsafe {
            SendMessageTimeoutW(
                hwnd,
                WM_SETTINGCHANGE,
                WPARAM(0),
                LPARAM(0),
                SMTO_ABORTIFHUNG,
                1000,
                Some(&mut result as *mut _),
            );
        }
    }

    Ok(())
}
