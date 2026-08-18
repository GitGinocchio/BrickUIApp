use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::{
    fmt::{self, Display, Formatter}, sync::Arc,
};
use tauri::{AppHandle, Emitter as _};
use windows::{
    Devices::{
        Bluetooth::{BluetoothConnectionStatus, BluetoothDevice},
        Enumeration::{DeviceInformation, DeviceInformationUpdate, DeviceWatcher},
    },
    Foundation::TypedEventHandler, core::{Error, HRESULT}
};

use crate::{
    state::{bluetooth::BrickUIBluetoothState, iconcache::BrickUIconCacheState}, 
    winapi::bluetooth::{
        win32::Win32Device, 
        winrt::{
            WinRTDevice,
            extract_winrt_device_address
        }
    }
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
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PairingMessage {
    // Pairing modes
    ConfirmOnly,
    ConfirmPinMatch { pin: String },
    DisplayPin { pin: String },
    ProvideAddress,
    ProvidePin,

    // Messages
    Paired,
    CantPair,
    AlreadyPaired,

    // Errors
    Failed { message: String }
}

#[derive(Clone, Debug)]
pub enum AcceptPairingMessage {
    AcceptWithAddress(String),
    AcceptWithPin(String),
    Accept
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
            (Some(w32), Some(wrt)) => write!(f, "Device(win32: {}, winrt: {}, connection_type: {:?})", w32, wrt, self.connection_type),
            (Some(w32), None) => write!(f, "Device(win32: {}, connection_type: {:?})", w32, self.connection_type),
            (None, Some(wrt)) => write!(f, "Device(winrt: {}, connection_type: {:?})", wrt, self.connection_type),
            (None, None) => write!(f, "Device(empty)"),
        }
    }
}

impl Device {
    /*
    pub fn merge_win32(&mut self, mut new: win32::Win32Device) {
        if let Some(existing) = &mut self.win32 {
            // preserva il socket
            new.socket = existing.socket.take();
        }

        // Se viene fatta un'altra scansione e viene rilevato che il device non e' piu' paired allora resettiamo il connection_type
        if let Some(ConnectionType::Win32) = self.connection_type && !new.authenticated {
            self.connection_type = None;
        }

        self.win32 = Some(new);
    }

    pub fn merge_winrt(&mut self, new: winrt::WinRTDevice) {
        // Se viene fatta un'altra scansione e viene rilevato che il device non e' piu' paired allora resettiamo il connection_type
        if let Some(ConnectionType::WinRT) = self.connection_type && !new.is_paired {
            self.connection_type = None;
        }

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
    */

    pub async fn pair(&mut self, app_handle: AppHandle) -> Result<PairingMessage, String> {
        // TODO: Aggiungere una preferenza di registazione
        // L'utente puo' mostrare una preferenza su quale backend usare
        // Win32 o WinRT

        if let Some(ConnectionType::Win32) = self.connection_type && let Some(win32) = &self.win32 && !win32.authenticated {
            self.connection_type = None;
        }
        else if let Some(ConnectionType::WinRT) = self.connection_type && let Some(winrt) = &self.winrt && !winrt.is_paired {
            self.connection_type = None;
        }
        
        if self.connection_type.is_some() {
            return Ok(PairingMessage::AlreadyPaired);
        }

        // 1. Provo WinRT
        if let Some(winrt) = &mut self.winrt {
            match winrt.pair(app_handle) {
                Ok(PairingMessage::CantPair) => {
                    eprintln!("Device cant pair. Trying Win32...");
                    // non ritorno subito, provo il fallback    
                },
                Err(e) => {
                    eprintln!("WinRT registration failed: {e}. Trying Win32...");
                    // non ritorno subito, provo il fallback
                },
                Ok(msg) => {
                    self.connection_type = Some(ConnectionType::WinRT);

                    return Ok(msg);
                },
            }
        }

        // 2. Provo Win32 come fallback
        if let Some(win32) = &mut self.win32 {
            match win32.pair() {
                Ok(_) => {
                    self.connection_type = Some(ConnectionType::Win32);
                    return Ok(PairingMessage::Paired);
                },
                Err(e) => return Err(format!("Win32 registration failed: {e}")),
            }
        }

        Err(format!("Device has no backend available!"))
    }

    pub fn unpair(&mut self) -> Result<(), String> {
        if let Some(winrt) = &mut self.winrt {
            winrt
                .unpair()
                .map_err(|e| format!("WinRT unregister failed: {e}"))?;
        } else if let Some(win32) = &mut self.win32 {
            win32
                .unpair()
                .map_err(|e| format!("Win32 unregister failed: {e}"))?;
        } else {
            return Err(format!("Device {self:?} has no backend available!"));
        }

        self.connection_type = None;
        Ok(())
    }

    pub async fn connect(&mut self) -> Result<(), String> {
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<(), String> {
        Ok(())
    }
}

/*
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn scan(duration: Option<u8>) -> Result<HashMap<String, Device>, String> {
    let mut devices = spawn_blocking(move || scan_win32(duration))
        .await?
        .into_iter()
        .map(|device| {
            let address = device.address.clone();
            (address, Device::new(Some(device), None))
        })
        .collect::<HashMap<String, Device>>();

    let winrt_devices = scan_winrt()
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

    let devices = scan_winrt()
        .await?
        .into_iter()
        .map(|device| {
            let address = device.address.clone();
            (address, Device::new(None, Some(device)))
        })
        .collect::<HashMap<String, Device>>();

    Ok(devices)
}
*/

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn start_bluetooth_watcher(
    bluetooth_state: Arc<RwLock<BrickUIBluetoothState>>,
    icon_cache: Arc<RwLock<BrickUIconCacheState>>,
    app_handle: &AppHandle
) -> Result<DeviceWatcher, String> {
    // Selector AEP (Classic + audio/HID)
    //let selector = HSTRING::from(r#"System.Devices.Aep.ProtocolId:="{e0cbf06c-cd8b-4647-bb8a-263b43f0f974}""#);

    let selector = BluetoothDevice
        ::GetDeviceSelectorFromConnectionStatus(BluetoothConnectionStatus::Disconnected)
        .map_err(|e| format!("{e}"))?;

    let watcher: DeviceWatcher = DeviceInformation
        ::CreateWatcherAqsFilter(&selector)
        .map_err(|e| format!("Error creating device watcher: {e}"))?;

    let ic_state_arc = icon_cache.clone();
    let bt_state_arc = bluetooth_state.clone();
    let app_handle_arc = app_handle.clone();

    watcher.Added(&TypedEventHandler::<DeviceWatcher, DeviceInformation>::new(move |_watcher, info| {
        let info = info.as_ref().expect("[watcher:added]: Error obtaining args");
        let bt_state = bt_state_arc.clone();
        let ic_state = ic_state_arc.clone();
        let app_handle_clone = app_handle_arc.clone();
        let info_clone = info.clone();

        let id = info
            .Id()
            .expect("Error obtaining device id")
            .to_string_lossy()
            .to_string();

        let address = extract_winrt_device_address(&id);

        let mut guard = bt_state.blocking_write();

        let winrt = WinRTDevice::from_info(&info_clone, ic_state).ok();
        let win32 = Win32Device::from_mac(&address).ok();
        let device = Device::new(win32, winrt);

        println!("Device added: {device:?}");

        app_handle_clone
            .emit("bluetooth_device_added", device.clone())
            .map_err(|e| Error::new(HRESULT(-1), e.to_string()))?;

        guard.devices.insert(address, device);
            
        Ok(())
    })).map_err(|e| format!("Error setting Added watcher: {e}"))?;

    let bt_state_arc = bluetooth_state.clone();
    let app_handle_arc = app_handle.clone();

    watcher.Updated(&TypedEventHandler::<DeviceWatcher, DeviceInformationUpdate>::new(move |_watcher, uinfo| {
        let uinfo = uinfo.as_ref().expect("[watcher:updated]: Error obtaining args");
        let app_handle_clone = app_handle_arc.clone();
        let state_clone = bt_state_arc.clone();
        let uinfo_clone = uinfo.clone();

        println!("device updated: {:#?}", uinfo.Id()?);

        let mut guard = state_clone.blocking_write();

        let id = uinfo_clone
            .Id()
            .map_err(|e| Error::new(HRESULT(-1), format!("Error obtaining Id: {e}")))?
            .to_string_lossy()
            .to_string();

        let address = extract_winrt_device_address(&id);

        if let Some(device) = guard.devices.get_mut(&address) && let Some(winrt) = &mut device.winrt {
            let kind = uinfo_clone
                .Kind()
                .map_err(|e| Error::new(HRESULT(-1), format!("Error obtaining Kind: {e}")))?;
            
            winrt.kind = format!("{:?}", kind);

            let win32 = Win32Device::from_mac(&winrt.address)
                .map_err(|e| Error::new(HRESULT(-1), format!("Error obtaining Win32Device: {e}")))?;

            device.win32 = Some(win32);

            app_handle_clone
                .emit("bluetooth_device_updated", device.clone())
                .map_err(|e| Error::new(HRESULT(-1), e.to_string()))?;
        }

        Ok(())
    })).map_err(|e| format!("Error setting Added watcher: {e}"))?;

    let bt_state_arc = bluetooth_state.clone();
    let app_handle_arc = app_handle.clone();

    watcher.Removed(&TypedEventHandler::<DeviceWatcher, DeviceInformationUpdate>::new(move |_watcher, uinfo| {
        let uinfo = uinfo.as_ref().expect("[watcher:updated]: Error obtaining args");
        let app_handle_clone = app_handle_arc.clone();
        let state_clone = bt_state_arc.clone();
        let uinfo_clone = uinfo.clone();

        println!("device removed: {:#?}", uinfo.Id()?);

        let mut guard = state_clone.blocking_write();

        let id = uinfo_clone
            .Id()
            .map_err(|e| Error::new(HRESULT(-1), format!("Error obtaining Id: {e}")))?
            .to_string_lossy()
            .to_string();

        if let Some(removed) = guard.devices.remove(&id) {
            app_handle_clone
                .emit("bluetooth_device_removed", removed)
                .map_err(|e| Error::new(HRESULT(-1), format!("Error sending removed device: {e}")))?
        }
        
        Ok(())
    })).map_err(|e| format!("Error setting Removed watcher: {e}"))?;

    watcher.Start().map_err(|e| format!("Error starting watcher: {e}"))?;

    Ok(watcher)
}