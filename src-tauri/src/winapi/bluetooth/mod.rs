use std::{collections::HashMap, fmt::{self, Display, Formatter}};
use serde::Serialize;
use windows::{Devices::{Bluetooth::BluetoothConnectionStatus, Enumeration::{DeviceInformation, DeviceWatcher}}, Foundation::TypedEventHandler, core::HSTRING};

use crate::{utils::spawn_blocking, winapi::bluetooth::win32::scan_win32};

pub mod ble;
pub mod win32;
pub mod winrt;

/*
pub trait BTDevice {
    fn register(&mut self) -> Result<(), String>;
    fn remove(&mut self) -> Result<(), String>;

    fn connect(&mut self) -> Result<(), String>;
    fn disconnect(&mut self) -> Result<(), String>;

    fn read(&mut self) -> Result<Vec<u8>, String>;
    fn write(&mut self, data: &[u8]) -> Result<(), String>;
}
*/

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Device {
    Win32(win32::Win32Device),
    WinRT(winrt::WinRTDevice),
}

impl Display for Device {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Device::Win32(device) => write!(
                f,
                "Device::Win32(address: {}, name: {}, auth: {}, connected: {}, remembered: {})",
                device.address,
                device.name,
                device.authenticated,
                device.connected,
                device.remembered
            ),

            Device::WinRT(_device) => write!(
                f,
                "Device::WinRT()"
            ),
        }
    }
}


impl Device {
    pub fn merge_from(&mut self, mut new: Device) {
        match (self, &mut new) {
            // ---- MERGE Win32 ----
            (Device::Win32(existing), Device::Win32(new_dev)) => {
                // preserva il socket
                new_dev.socket = existing.socket.take();

                // aggiorna il resto
                *existing = new_dev.clone();
            }

            // ---- MERGE WinRT ----
            (Device::WinRT(existing), Device::WinRT(new_dev)) => {
                *existing = new_dev.clone();
            }

            // ---- Varianti diverse ----
            (existing, new_dev) => {
                *existing = new_dev.clone();
            }
        }
    }

    pub fn is_win32() -> bool { todo!() }
    pub fn is_winrt() -> bool { todo!() }
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn scan(duration: Option<u8>) -> Result<HashMap<String, Device>, String> {
    let classic_devices = spawn_blocking(move || scan_win32(duration))
        .await?
        .into_iter()
        .map(|device| {
            let address = device.address.clone();
            (address, Device::Win32(device))
        })
        .collect::<HashMap<String, Device>>();

    Ok(classic_devices)
}