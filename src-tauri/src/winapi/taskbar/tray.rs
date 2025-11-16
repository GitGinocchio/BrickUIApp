use std::path::PathBuf;
use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance};
use windows::Win32::System::Variant::VARIANT;
use windows::Win32::UI::Accessibility::{
    CUIAutomation, IUIAutomation, IUIAutomationElement, TreeScope_Descendants,
    UIA_ButtonControlTypeId, UIA_ControlTypePropertyId, UIA_CustomControlTypeId,
    UIA_ToolTipControlTypeId,
};
use windows::Win32::UI::WindowsAndMessaging::FindWindowA;
use windows::core::{BSTR, PCSTR};

use crate::winapi::icons::IconsMap;
use crate::winapi::rect::Rect;
//use crate::config::{save_yaml, save_yaml_async};

#[derive(serde::Serialize, Debug)]
pub struct TrayIcon {
    pub pid: i32,
    pub name: String,
    pub tooltip: String,
    pub rect: Rect,
}

pub fn get_tray_icons(
    _icon_cache_dir: &PathBuf,
    _icons_map: &mut IconsMap,
    _max_files: usize,
) -> Result<Vec<TrayIcon>, String> {
    unsafe {
        let mut results: Vec<TrayIcon> = Vec::new();

        let shell = FindWindowA(PCSTR(b"Shell_TrayWnd\0".as_ptr()), None)
            .map_err(|e| format!("Error obtaining Shell_TrayWnd window: {e}"))?;
        if shell.is_invalid() {
            return Err("Could not get Shell_TrayWnd".into());
        }

        /*
        let mut abd = APPBARDATA {
            cbSize: std::mem::size_of::<APPBARDATA>() as u32,
            hWnd: shell,
            uEdge: 0,
            rc: Default::default(),
            lParam: LPARAM(ABS_AUTOHIDE as isize),
            uCallbackMessage: 0,
        };
        SHAppBarMessage(ABM_SETSTATE, &mut abd);
        */

        let automation: IUIAutomation =
            CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER)
                .map_err(|e| format!("Error creating UIAutomation instance: {e}"))?;

        let root: IUIAutomationElement = automation
            .ElementFromHandle(shell)
            .map_err(|e| format!("Error obtaining Shell_TrayWnd element: {e}"))?;

        // 1️⃣ Raccogli tutte le icone (Button o Custom)
        let vt_button: VARIANT = VARIANT::from(UIA_ButtonControlTypeId.0);
        let vt_custom: VARIANT = VARIANT::from(UIA_CustomControlTypeId.0);
        let cond_button = automation
            .CreatePropertyCondition(UIA_ControlTypePropertyId, &vt_button)
            .map_err(|e| format!("{e}"))?;
        let cond_custom = automation
            .CreatePropertyCondition(UIA_ControlTypePropertyId, &vt_custom)
            .map_err(|e| format!("{e}"))?;
        let cond_icons = automation
            .CreateOrCondition(&cond_button, &cond_custom)
            .map_err(|e| format!("{e}"))?;

        let icon_elements = root
            .FindAll(TreeScope_Descendants, &cond_icons)
            .map_err(|e| format!("{e}"))?;
        let icon_count = icon_elements.Length().map_err(|e| format!("{e}"))?;

        for i in 0..icon_count {
            let element = icon_elements.GetElement(i).map_err(|e| format!("{e}"))?;

            let pid = element.CurrentProcessId().map_err(|e| format!("{e}"))?;
            let name: BSTR = element.CurrentName().unwrap_or_default();
            let rect = element.CurrentBoundingRectangle().unwrap_or_default();
            //let offscreen = element.CurrentIsOffscreen().unwrap_or_default().as_bool();

            /*
            if offscreen || name.to_string().trim().is_empty() {
                continue;
            }
            */

            results.push(TrayIcon {
                pid,
                name: name.to_string(),
                tooltip: String::new(),
                rect: rect.into(),
            });
        }

        // 2️⃣ Raccogli i tooltip visibili
        let vt_tt: VARIANT = VARIANT::from(UIA_ToolTipControlTypeId.0);
        let cond_tt = automation
            .CreatePropertyCondition(UIA_ControlTypePropertyId, &vt_tt)
            .map_err(|e| format!("{e}"))?;
        let tt_elements = root
            .FindAll(TreeScope_Descendants, &cond_tt)
            .map_err(|e| format!("{e}"))?;
        let tt_count = tt_elements.Length().map_err(|e| format!("{e}"))?;

        // 3️⃣ Abbina tooltip all’icona più vicina (geometria)
        for j in 0..tt_count {
            let tt_elem = tt_elements.GetElement(j).map_err(|e| format!("{e}"))?;
            let tt_name: BSTR = tt_elem.CurrentName().unwrap_or_default();
            let tt_text = tt_name.to_string();
            if tt_text.is_empty() {
                continue;
            }
            let tt_rect = tt_elem.CurrentBoundingRectangle().unwrap_or_default();

            // Trova icona più vicina
            let mut min_dist = f64::MAX;
            let mut best_idx = None;
            for (idx, icon) in results.iter().enumerate() {
                let dx = (icon.rect.left - tt_rect.left).abs() as f64;
                let dy = (icon.rect.top - tt_rect.top).abs() as f64;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < min_dist {
                    min_dist = dist;
                    best_idx = Some(idx);
                }
            }

            if let Some(idx) = best_idx {
                results[idx].tooltip = tt_text;
            }
        }

        /*
        let mut abd = APPBARDATA {
            cbSize: std::mem::size_of::<APPBARDATA>() as u32,
            hWnd: shell,
            uEdge: 0,
            rc: Default::default(),
            lParam: LPARAM(ABS_ALWAYSONTOP as isize),
            uCallbackMessage: 0,
        };
        SHAppBarMessage(ABM_SETSTATE, &mut abd);
        */

        Ok(results)
    }
}
