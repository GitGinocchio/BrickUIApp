use std::collections::HashMap;
use winreg::enums::*;
use winreg::RegKey;
use windows::Win32::UI::WindowsAndMessaging::{SystemParametersInfoW, SPI_SETCURSORS};

/// Applica le modifiche scritte nel registro
fn apply_cursor_changes() -> Result<(), String> {
    unsafe {
        SystemParametersInfoW(
            SPI_SETCURSORS, 
            0, 
            None, 
            windows::Win32::UI::WindowsAndMessaging::SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0)
        ).map_err(|e| format!("Error while applying cursor changes: {e}"))?;
    }

    Ok(())
}

/// Salva i cursori correnti
fn backup_cursors() -> HashMap<String, String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey("Control Panel\\Cursors").unwrap();

    let mut cursors = HashMap::new();
    for name in [
        "Arrow", "Hand", "AppStarting", "Wait", "IBeam",
        "Crosshair", "Help", "No", "NWPen", "SizeAll",
        "SizeNESW", "SizeNS", "SizeNWSE", "SizeWE", "UpArrow",
    ] {
        if let Ok(value) = key.get_value::<String, _>(name) {
            cursors.insert(name.to_string(), value);
        }
    }
    cursors
}

/// Imposta tutti i cursori a un file trasparente
pub fn hide_cursors(transparent_cur: &str) -> Result<(), String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
        .open_subkey_with_flags("Control Panel\\Cursors", KEY_SET_VALUE)
        .unwrap();

    for name in [
        "Arrow", "Hand", "AppStarting", "Wait", "IBeam",
        "Crosshair", "Help", "No", "NWPen", "SizeAll",
        "SizeNESW", "SizeNS", "SizeNWSE", "SizeWE", "UpArrow",
    ] {
        key.set_value(name, &transparent_cur).unwrap();
    }

    apply_cursor_changes()?;

    Ok(())
}

// Sarebbe da fare un metodo di questo tipo e permettere all'utente di impostare un cursore custom
// all'interno di un brick
// pub fn set_cursor(...) -> Result<(), String> {}

/// Ripristina i cursori originali
pub fn restore_cursors(backup: &HashMap<String, String>) -> Result<(), String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
        .open_subkey_with_flags("Control Panel\\Cursors", KEY_SET_VALUE)
        .unwrap();

    for (name, value) in backup {
        key.set_value(name, value).unwrap();
    }

    apply_cursor_changes()?;

    Ok(())
}