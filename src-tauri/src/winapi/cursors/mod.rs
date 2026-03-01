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

use crate::state::cursors::CursorsState;

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

fn serialize_scheme(cursors: &HashMap<CursorType, String>) -> String {
    CursorType::iter()
        .map(|ct| cursors.get(&ct).cloned().unwrap_or_default())
        .collect::<Vec<_>>()
        .join(",")
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


#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, 
    Eq, Hash, EnumIter, AsRefStr, JsonSchema
)]
pub enum CursorType {
    Arrow,
    Help,
    AppStarting,
    Wait,
    Crosshair,
    IBeam,
    NWPen,
    No,
    SizeNS,
    SizeWE,
    SizeNWSE,
    SizeNESW,
    SizeAll,
    UpArrow,
    Hand
}

#[derive(Clone, Debug)]
pub struct Scheme {
    pub name: String,
    pub cursors: HashMap<CursorType, String>,
}

impl Scheme {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            cursors: HashMap::new(),
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let schemes_key = hkcu.open_subkey("Control Panel\\Cursors\\Schemes").ok()?;

        // Legge la stringa del schema
        let value: String = schemes_key.get_value(name).ok()?;

        // Parla la stringa in HashMap<CursorType,String>
        let mut cursors = HashMap::new();
        let parts: Vec<&str> = value.split(',').collect();
        for pair in parts.chunks_exact(2) {
            if let (Some(cursor_name), Some(path)) = (pair.get(0), pair.get(1)) {
                if let Some(cursor_type) = CursorType::iter()
                    .find(|ct| ct.as_ref() == *cursor_name)
                {
                    cursors.insert(cursor_type, path.to_string());
                }
            }
        }

        Some(Self {
            name: name.to_string(),
            cursors,
        })
    }

    /// Aggiunge o aggiorna un cursore nello schema (solo in memoria)
    pub fn add_cursor(&mut self, cursor_type: CursorType, path: &str) {
        self.cursors.insert(cursor_type, path.to_string());
    }

    /// Rimuove un cursore dallo schema (solo in memoria)
    pub fn remove_cursor(&mut self, cursor_type: CursorType) {
        self.cursors.remove(&cursor_type);
    }

    pub fn save(&self) -> Result<(), String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let schemes_key = hkcu
            .open_subkey_with_flags("Control Panel\\Cursors\\Schemes", KEY_SET_VALUE)
            .or_else(|_| hkcu.create_subkey("Control Panel\\Cursors\\Schemes").map(|(k,_)| k))
            .map_err(|e| format!("Error obtaining schemes key: {e}"))?;

        let value = serialize_scheme(&self.cursors);
        schemes_key
            .set_value(&self.name, &value)
            .map_err(|e| format!("Error setting scheme key: {e}"))?;

        Ok(())
    }

    pub fn delete_scheme(name: &str) -> Result<(), String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        // Controlla schema attivo
        let cursors_key = hkcu
            .open_subkey("Control Panel\\Cursors")
            .map_err(|e| e.to_string())?;

        if let Ok(active) = cursors_key.get_value::<String, _>("Scheme Source") {
            if active == name {
                // fallback
                if let Some(default) = Scheme::from_name("Windows Default") {
                    default.save()?;
                }
            }
        }

        let schemes_key = hkcu
            .open_subkey_with_flags(
                "Control Panel\\Cursors\\Schemes",
                KEY_SET_VALUE,
            )
            .map_err(|e| e.to_string())?;

        schemes_key
            .delete_value(name)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn delete(self) -> Result<(), String> {
        Scheme::delete_scheme(&self.name)?;
        Ok(())
    }
}

impl CursorsState {
    pub fn add_scheme(&mut self, scheme: Scheme) -> Result<(), String> {
        scheme.save()?;
        self.schemes.insert(scheme.name.clone(), scheme);
        Ok(())
    }
    pub fn remove_scheme(&mut self, name: &str) -> Result<(), String> {
        if let Some(scheme) = self.schemes.remove(name) {
            scheme.delete()?;
            return Ok(());
        }
        Err("scheme not found".into())
    }

    pub fn set_scheme(&mut self, name: &str) -> Result<(), String> {
        let scheme = if let Some(s) = self.schemes.get(name) {
            s.clone()
        } else {
            Scheme::from_name(name)
                .ok_or("Scheme not found")?
        };

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let key = hkcu
            .open_subkey_with_flags("Control Panel\\Cursors", KEY_SET_VALUE)
            .map_err(|e| e.to_string())?;

        for ct in CursorType::iter() {
            let value = scheme
                .cursors
                .get(&ct)
                .cloned()
                .unwrap_or_default();

            key.set_value(ct.as_ref(), &value)
                .map_err(|e| e.to_string())?;
        }

        key.set_value("Scheme Source", &scheme.name)
            .map_err(|e| e.to_string())?;

        apply_cursor_changes()?;

        Ok(())
    }
    
    pub fn unset_scheme(&mut self) -> Result<(), String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let key = hkcu
            .open_subkey_with_flags("Control Panel\\Cursors", KEY_SET_VALUE)
            .map_err(|e| e.to_string())?;

        for (ct, path) in &self.default {
            key.set_value(ct.as_ref(), path)
                .map_err(|e| e.to_string())?;
        }

        // Pulisce Scheme Source (opzionale)
        key.set_value("Scheme Source", &"")
            .map_err(|e| e.to_string())?;

        apply_cursor_changes()?;

        Ok(())
    }

    pub fn get_scheme(&self, name: &str) -> Option<&Scheme> {
        self.schemes.get(name)
    }

    pub fn get_schemes(&self) -> &HashMap<String, Scheme> {
        &self.schemes
    }
}