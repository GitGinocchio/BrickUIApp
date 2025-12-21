use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};
use tauri::AppHandle;
use windows::{
    Devices::{
        Bluetooth::BluetoothConnectionStatus,
        Enumeration::{DeviceInformation, DeviceWatcher},
    },
    Foundation::TypedEventHandler,
    core::HSTRING,
};

use crate::{
    utils::spawn_blocking,
    winapi::bluetooth::{win32::scan_win32, winrt::scan_winrt},
};

pub mod ble;
pub mod win32;
pub mod winrt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConnectionType {
    Win32,
    WinRT,
}

#[derive(Clone, Debug, Serialize)]
pub struct Device {
    pub win32: Option<win32::Win32Device>,
    pub winrt: Option<winrt::WinRTDevice>,
    pub connection_type: Option<ConnectionType>,
}

impl Device {
    pub fn new(win32: Option<win32::Win32Device>, winrt: Option<winrt::WinRTDevice>) -> Self {
        Self {
            win32: win32,
            winrt: winrt,
            connection_type: None,
        }
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match (&self.win32, &self.winrt) {
            (Some(w32), Some(wrt)) => write!(f, "Device(win32: {}, winrt: {})", w32, wrt),
            (Some(w32), None) => write!(f, "Device(win32: {})", w32),
            (None, Some(wrt)) => write!(f, "Device(winrt: {})", wrt),
            (None, None) => write!(f, "Device(empty)"),
        }
    }
}

impl Device {
    pub fn merge_win32(&mut self, mut new: win32::Win32Device) {
        if let Some(existing) = &mut self.win32 {
            // preserva il socket
            new.socket = existing.socket.take();
        }

        self.win32 = Some(new);
    }

    pub fn merge_winrt(&mut self, new: winrt::WinRTDevice) {
        self.winrt = Some(new);
    }

    pub fn merge_from(&mut self, other: Device) {
        if let Some(new_win32) = other.win32 {
            self.merge_win32(new_win32);
        }
        if let Some(new_winrt) = other.winrt {
            self.merge_winrt(new_winrt);
        }
    }

    pub async fn register(&mut self) -> Result<(), String> {
        // TODO: Aggiungere una preferenza di registazione
        // L'utente puo' mostrare una preferenza su quale backend usare
        // Win32 o WinRT

        if self.connection_type.is_some() {
            return Ok(()); // Already connected
        }

        if let Some(winrt) = &mut self.winrt {
            winrt
                .register()
                .await
                .map_err(|e| format!("WinRT registration failed: {e}"))?;
            self.connection_type = Some(ConnectionType::WinRT);
        } else if let Some(win32) = &mut self.win32 {
            win32
                .register()
                .map_err(|e| format!("Win32 registration failed: {e}"))?;
            self.connection_type = Some(ConnectionType::Win32);
        } else {
            return Err(format!("Device has no backend available!"));
        }

        Ok(())
    }

    pub async fn unregister(&mut self) -> Result<(), String> {
        if let Some(winrt) = &mut self.winrt {
            winrt
                .unregister()
                .await
                .map_err(|e| format!("WinRT unregister failed: {e}"))?;
        } else if let Some(win32) = &mut self.win32 {
            win32
                .unregister()
                .map_err(|e| format!("Win32 unregister failed: {e}"))?;
        } else {
            return Err(format!("Device {self:?} has no backend available!"));
        }

        self.connection_type = None;
        Ok(())
    }

    pub async fn connect(&mut self) -> Result<(), String> {
        self.register().await
    }

    pub async fn disconnect(&mut self) -> Result<(), String> {
        self.unregister().await
    }
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn scan(
    app_handle: AppHandle,
    duration: Option<u8>,
) -> Result<HashMap<String, Device>, String> {
    let mut devices = spawn_blocking(move || scan_win32(duration))
        .await?
        .into_iter()
        .map(|device| {
            let address = device.address.clone();
            (address, Device::new(Some(device), None))
        })
        .collect::<HashMap<String, Device>>();

    let winrt_devices = scan_winrt(app_handle)
        .await?
        .into_iter()
        .map(|device| {
            let address = device.address.clone();
            (address, Device::new(None, Some(device)))
        })
        .collect::<HashMap<String, Device>>();

    for (address, winrt_device) in winrt_devices {
        devices
            .entry(address)
            .and_modify(|device| {
                device.merge_from(winrt_device.clone());
            })
            .or_insert(winrt_device);
    }

    Ok(devices)
}
