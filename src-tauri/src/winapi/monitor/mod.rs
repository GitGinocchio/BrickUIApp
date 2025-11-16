use std::{ffi::OsString, os::windows::ffi::OsStringExt};

use serde::{Deserialize, Serialize};
use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, POINT, RECT},
        Graphics::Gdi::{
            DISPLAY_DEVICEW, EnumDisplayDevicesW, EnumDisplayMonitors, GetMonitorInfoW, HDC,
            HMONITOR, MONITOR_FROM_FLAGS, MONITORINFO, MONITORINFOEXW, MonitorFromPoint,
            MonitorFromWindow,
        },
        UI::WindowsAndMessaging::MONITORINFOF_PRIMARY,
    },
    core::{BOOL, PCWSTR},
};

use crate::winapi::rect::{OptionalRect, Rect};

pub mod workarea;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Monitor {
    pub is_primary: bool,
    pub rect: Rect,
    pub workarea: Rect,
    pub device_name: String,
    pub friendly_name: Option<String>,
    pub hmonitor: isize,
}

impl TryFrom<HMONITOR> for Monitor {
    type Error = String;

    fn try_from(hmonitor: HMONITOR) -> Result<Self, String> {
        unsafe {
            let mut info = MONITORINFOEXW::default();
            info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;

            GetMonitorInfoW(hmonitor, &mut info as *mut _ as *mut _)
                .ok()
                .map_err(|e| format!("Error obtaining monitor info: {e}"))?;

            let len = info
                .szDevice
                .iter()
                .position(|&c| c == 0)
                .unwrap_or(info.szDevice.len());
            let device_name = OsString::from_wide(&info.szDevice[..len])
                .to_string_lossy()
                .to_string();

            let friendly_name = get_monitor_friendly_name(&device_name);

            Ok(Monitor {
                is_primary: info.monitorInfo.dwFlags & MONITORINFOF_PRIMARY != 0,
                rect: Rect {
                    left: info.monitorInfo.rcMonitor.left,
                    top: info.monitorInfo.rcMonitor.top,
                    right: info.monitorInfo.rcMonitor.right,
                    bottom: info.monitorInfo.rcMonitor.bottom,
                },
                workarea: Rect {
                    left: info.monitorInfo.rcWork.left,
                    top: info.monitorInfo.rcWork.top,
                    right: info.monitorInfo.rcWork.right,
                    bottom: info.monitorInfo.rcWork.bottom,
                },
                device_name,
                friendly_name,
                hmonitor: hmonitor.0 as isize,
            })
        }
    }
}

enum MonitorTarget {
    All(Vec<Monitor>), // raccoglie tutti i monitor
    Single {
        // cerca un monitor specifico
        index: usize, // o criterio qualsiasi
        found: Option<Monitor>,
        current_index: usize,
    },
    Primary {
        found: Option<Monitor>,
    },
}

unsafe extern "system" fn enum_monitors_proc(
    hmonitor: HMONITOR,
    _hdc: HDC,
    _lprc_clip: *mut RECT,
    lparam: LPARAM,
) -> BOOL {
    let target_ptr = lparam.0 as *mut MonitorTarget;
    if target_ptr.is_null() {
        return BOOL(0);
    }
    let target = unsafe { &mut *target_ptr };

    let mut miex = MONITORINFOEXW {
        monitorInfo: MONITORINFO {
            cbSize: size_of::<MONITORINFOEXW>() as u32,
            ..Default::default()
        },
        szDevice: [0; 32],
    };

    if unsafe { !GetMonitorInfoW(hmonitor, &mut miex.monitorInfo as *mut _ as *mut _).as_bool() } {
        return BOOL(1);
    }

    let mi = miex.monitorInfo;
    let device_name = utf16_cstr_to_string(&miex.szDevice);
    let friendly_name = get_monitor_friendly_name(&device_name);
    let is_primary = (mi.dwFlags & 1) != 0;

    let monitor = Monitor {
        is_primary,
        rect: mi.rcMonitor.into(),
        workarea: mi.rcWork.into(),
        device_name,
        friendly_name,
        hmonitor: hmonitor.0 as isize,
    };

    match target {
        MonitorTarget::All(vec) => {
            vec.push(monitor);
            BOOL(1)
        }
        MonitorTarget::Single {
            index,
            current_index,
            found,
        } => {
            if *current_index == *index {
                *found = Some(monitor);
                return BOOL(-1);
            }
            *current_index += 1;
            BOOL(1)
        }
        MonitorTarget::Primary { found } => {
            if monitor.is_primary {
                *found = Some(monitor);
                return BOOL(-1); // stop enumerazione
            }
            BOOL(1)
        }
    }
}

/// Converte WCHAR (null terminated) in String
fn utf16_cstr_to_string(buf: &[u16]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..len])
}

pub fn get_monitor_friendly_name(device_name: &str) -> Option<String> {
    let mut display_device = DISPLAY_DEVICEW {
        cb: std::mem::size_of::<DISPLAY_DEVICEW>() as u32,
        ..Default::default()
    };

    // device_name: es. "\\\\.\\DISPLAY1"
    let device_w: Vec<u16> = device_name
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    if unsafe {
        EnumDisplayDevicesW(PCWSTR(device_w.as_ptr()), 0, &mut display_device, 0).as_bool()
    } {
        // DeviceString è un [u16; 128] → convertilo in Rust String
        let friendly_name = String::from_utf16_lossy(
            &display_device.DeviceString[..display_device
                .DeviceString
                .iter()
                .position(|&c| c == 0)
                .unwrap_or(128)],
        );
        return Some(friendly_name);
    }
    None
}

pub fn get_all_monitors() -> Result<Vec<Monitor>, String> {
    let mut target = MonitorTarget::All(Vec::new());

    unsafe {
        EnumDisplayMonitors(
            Some(HDC(std::ptr::null_mut())),
            Some(std::ptr::null()),
            Some(enum_monitors_proc),
            LPARAM(&mut target as *mut _ as isize),
        )
        .ok()
        .map_err(|e| format!("EnumDisplayMonitors fallita: {e}"))?
    };

    match target {
        MonitorTarget::All(vec) => Ok(vec),
        _ => unreachable!(),
    }
}

pub fn get_monitor(index: usize) -> Result<Monitor, String> {
    let mut target = MonitorTarget::Single {
        index,
        current_index: 0,
        found: None,
    };

    unsafe {
        EnumDisplayMonitors(
            Some(HDC(std::ptr::null_mut())),
            Some(std::ptr::null()),
            Some(enum_monitors_proc),
            LPARAM(&mut target as *mut _ as isize),
        )
        .ok()
        .map_err(|e| format!("EnumDisplayMonitors fallita: {e}"))?
    };

    match target {
        MonitorTarget::Single { found, .. } => {
            found.ok_or_else(|| format!("Monitor with index {} not found", index))
        }
        _ => unreachable!(),
    }
}

pub fn get_monitor_from_point(x: i32, y: i32) -> Result<Monitor, String> {
    let pt = POINT { x, y };
    let hmon: HMONITOR = unsafe { MonitorFromPoint(pt, MONITOR_FROM_FLAGS(0)) };
    if hmon.0 == std::ptr::null_mut() {
        return Err("No monitor found at the given point".into());
    }

    let mut miex = MONITORINFOEXW {
        monitorInfo: MONITORINFO {
            cbSize: size_of::<MONITORINFOEXW>() as u32,
            ..Default::default()
        },
        szDevice: [0; 32],
    };

    if !unsafe { GetMonitorInfoW(hmon, &mut miex.monitorInfo as *mut _ as *mut _).as_bool() } {
        return Err("Failed to get monitor info".into());
    }

    let mi = miex.monitorInfo;
    let len = miex
        .szDevice
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(miex.szDevice.len());
    let device_name = String::from_utf16_lossy(&miex.szDevice[..len]);
    let friendly_name = get_monitor_friendly_name(&device_name);
    let is_primary = (mi.dwFlags & 1) != 0;

    Ok(Monitor {
        is_primary,
        rect: mi.rcMonitor.into(),
        workarea: mi.rcWork.into(),
        device_name,
        friendly_name,
        hmonitor: hmon.0 as isize,
    })
}

pub fn get_monitor_from_hwnd(hwnd: isize) -> Result<Monitor, String> {
    unsafe {
        // Ottieni handle del monitor più vicino alla finestra
        const MONITOR_DEFAULTTONEAREST: u32 = 2;
        let hmonitor: HMONITOR = MonitorFromWindow(
            HWND(hwnd as *mut _),
            MONITOR_FROM_FLAGS(MONITOR_DEFAULTTONEAREST),
        );

        if hmonitor.is_invalid() {
            return Err("Impossibile ottenere l'handle del monitor".into());
        }

        let mut miex = MONITORINFOEXW {
            monitorInfo: MONITORINFO {
                cbSize: size_of::<MONITORINFOEXW>() as u32,
                ..Default::default()
            },
            szDevice: [0; 32],
        };

        GetMonitorInfoW(hmonitor, &mut miex.monitorInfo as *mut _ as *mut _)
            .ok()
            .map_err(|e| format!("Error obtaining monitor info: {e}"))?;

        let mi = miex.monitorInfo;
        let device_name = utf16_cstr_to_string(&miex.szDevice);
        let friendly_name = get_monitor_friendly_name(&device_name);
        let is_primary = (mi.dwFlags & 1) != 0;

        // Crea oggetto Monitor personalizzato
        let monitor = Monitor {
            is_primary,
            rect: mi.rcMonitor.into(),
            workarea: mi.rcWork.into(),
            device_name,
            friendly_name,
            hmonitor: hmonitor.0 as isize,
        };

        Ok(monitor)
    }
}

pub fn get_primary_monitor() -> Result<Monitor, String> {
    let mut target = MonitorTarget::Primary { found: None };

    unsafe {
        EnumDisplayMonitors(
            Some(HDC(std::ptr::null_mut())),
            Some(std::ptr::null()),
            Some(enum_monitors_proc),
            LPARAM(&mut target as *mut _ as isize),
        )
        .ok()
        .map_err(|e| format!("EnumDisplayMonitors fallita: {e}"))?
    };

    match target {
        MonitorTarget::Primary { found } => found.ok_or_else(|| "Primary monitor not found".into()),
        _ => unreachable!(),
    }
}

pub fn get_primary_hmonitor() -> Result<isize, String> {
    let mut target = MonitorTarget::Primary { found: None };

    unsafe {
        EnumDisplayMonitors(
            Some(HDC(std::ptr::null_mut())),
            Some(std::ptr::null()),
            Some(enum_monitors_proc),
            LPARAM(&mut target as *mut _ as isize),
        )
        .ok()
        .map_err(|e| format!("EnumDisplayMonitors fallita: {e}"))?
    };

    match target {
        MonitorTarget::Primary { found } => found
            .and_then(|monitor| Some(monitor.hmonitor))
            .ok_or_else(|| "Primary monitor not found".into()),
        _ => unreachable!(),
    }
}
