// Implementare la gestione dei cursori
// Spostare il file cursor.rs qui dentro

// TODO: Piuttosto di modificare i cursori di default, utilizzare gli Schemes
//       presenti sempre nel regedit
//       quindi ogni volta 
// https://stackoverflow.com/questions/41713827/programmatically-change-custom-mouse-cursor-in-windows
// https://thebitguru.com/articles/programmatically-changing-windows-mouse-cursors/3

use std::collections::HashMap;

use schemars::JsonSchema;
use strum::IntoEnumIterator as _;
use strum_macros::{EnumIter, AsRefStr};
use serde::{Serialize, Deserialize};
use windows::Win32::UI::WindowsAndMessaging::{SPI_SETCURSORS, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, SystemParametersInfoW};
use winreg::enums::{HKEY_CURRENT_USER, KEY_SET_VALUE};
use winreg::RegKey;

#[derive(
    Clone, Debug, Serialize, Deserialize, 
    PartialEq, Eq, Hash, EnumIter, AsRefStr, JsonSchema
)]
pub enum CursorType {
    Arrow,
    Hand,
    AppStarting,
    Wait,
    IBeam,
    Crosshair,
    Help,
    No,
    NWPen,
    SizeAll,
    SizeNESW,
    SizeNS,
    SizeNWSE,
    SizeWE,
    UpArrow,
}

/// Applica le modifiche scritte nel registro
#[cfg_attr(feature = "profiling", tracing::instrument)]
fn apply_cursor_changes() -> Result<(), String> {
    unsafe {
        SystemParametersInfoW(
            SPI_SETCURSORS,
            0,
            None,
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
        .map_err(|e| format!("Error applying cursor changes: {e}"))?;
    }

    Ok(())
}

/// Salva i cursori correnti
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn backup_cursors() -> HashMap<CursorType, String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey("Control Panel\\Cursors").unwrap();

    let mut cursors = HashMap::new();
    for name in CursorType::iter() {
        if let Ok(value) = key.get_value::<String, _>(name.as_ref()) {
            cursors.insert(name, value);
        }
    }
    cursors
}

/// Imposta tutti i cursori a un file trasparente
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn hide_cursors(transparent_cur: &str) -> Result<(), String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
        .open_subkey_with_flags("Control Panel\\Cursors", KEY_SET_VALUE)
        .unwrap();

    for name in CursorType::iter() {
        key.set_value(name.as_ref(), &transparent_cur).unwrap();
    }

    apply_cursor_changes()?;

    Ok(())
}

/// Ripristina i cursori originali
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn restore_cursors(backup: &HashMap<CursorType, String>) -> Result<(), String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
        .open_subkey_with_flags("Control Panel\\Cursors", KEY_SET_VALUE)
        .map_err(|e| format!("Error opening cursor key: {e}"))?;

    for (name, value) in backup {
        key.set_value(name.as_ref(), value)
            .map_err(|e| format!("Error setting cursor value: {e}"))?;
    }

    apply_cursor_changes()?;

    Ok(())
}
