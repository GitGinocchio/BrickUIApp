use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use windows::Win32::UI::WindowsAndMessaging::{SPI_SETCURSORS, SystemParametersInfoW};
use winreg::RegKey;
use winreg::enums::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, JsonSchema)]
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

impl CursorType {
    pub const ALL: [CursorType; 15] = [
        CursorType::Arrow,
        CursorType::Hand,
        CursorType::AppStarting,
        CursorType::Wait,
        CursorType::IBeam,
        CursorType::Crosshair,
        CursorType::Help,
        CursorType::No,
        CursorType::NWPen,
        CursorType::SizeAll,
        CursorType::SizeNESW,
        CursorType::SizeNS,
        CursorType::SizeNWSE,
        CursorType::SizeWE,
        CursorType::UpArrow,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            CursorType::Arrow => "Arrow",
            CursorType::Hand => "Hand",
            CursorType::AppStarting => "AppStarting",
            CursorType::Wait => "Wait",
            CursorType::IBeam => "IBeam",
            CursorType::Crosshair => "Crosshair",
            CursorType::Help => "Help",
            CursorType::No => "No",
            CursorType::NWPen => "NWPen",
            CursorType::SizeAll => "SizeAll",
            CursorType::SizeNESW => "SizeNESW",
            CursorType::SizeNS => "SizeNS",
            CursorType::SizeNWSE => "SizeNWSE",
            CursorType::SizeWE => "SizeWE",
            CursorType::UpArrow => "UpArrow",
        }
    }
}

/// Applica le modifiche scritte nel registro
#[cfg_attr(feature = "profiling", tracing::instrument)]
fn apply_cursor_changes() -> Result<(), String> {
    unsafe {
        SystemParametersInfoW(
            SPI_SETCURSORS,
            0,
            None,
            windows::Win32::UI::WindowsAndMessaging::SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
        .map_err(|e| format!("Error while applying cursor changes: {e}"))?;
    }

    Ok(())
}

/// Salva i cursori correnti
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn backup_cursors() -> HashMap<CursorType, String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey("Control Panel\\Cursors").unwrap();

    let mut cursors = HashMap::new();
    for name in CursorType::ALL {
        if let Ok(value) = key.get_value::<String, _>(name.as_str()) {
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

    for name in CursorType::ALL {
        key.set_value(name.as_str(), &transparent_cur).unwrap();
    }

    apply_cursor_changes()?;

    Ok(())
}

// Sarebbe da fare un metodo di questo tipo e permettere all'utente di impostare un cursore custom
// all'interno di un brick
// pub fn set_cursor(...) -> Result<(), String> {}

/// Ripristina i cursori originali
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn restore_cursors(backup: &HashMap<CursorType, String>) -> Result<(), String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
        .open_subkey_with_flags("Control Panel\\Cursors", KEY_SET_VALUE)
        .unwrap();

    for (name, value) in backup {
        key.set_value(name.as_str(), value).unwrap();
    }

    apply_cursor_changes()?;

    Ok(())
}
