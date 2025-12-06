use std::{collections::{HashMap, hash_map::Entry}, sync::Arc};

use tauri::{AppHandle, Manager as _, State};
use tokio::sync::{Mutex, RwLock};
use windows::{Devices::Enumeration::DeviceInformation, Win32};

use crate::{
    state::{bluetooth::BrickUIBluetoothState, generic::BrickUIGenericState}, 
    utils::spawn_blocking, 
    winapi::bluetooth::{
        Device, 
        scan, 
        win32::{
            Win32Device, 
            //connect_win32,
            //disconnect_win32, 
            scan_win32
        }
    }
};

/*
use crate::{state::BrickUIGenericState, winapi::old::{AdapterInfo, DeviceInfo}};

#[tauri::command(async)]
pub async fn get_default_adapter(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
) -> Result<Option<AdapterInfo>, String> {
    let state_guard = state.lock().await;
    let ble_state = state_guard.get_ble_state();

    ble_state.get_default_adapter().await
}

#[tauri::command(async)]
pub async fn get_adapters(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
) -> Result<HashMap<usize, AdapterInfo>, String> {
    let state_guard = state.lock().await;
    let ble_state = state_guard.get_ble_state();

    ble_state.get_adapters().await
}

#[tauri::command(async)]
pub async fn get_devices(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
) -> Result<Vec<DeviceInfo>, String> {
    let mut state_guard = state.lock().await;
    let ble_state = state_guard.get_mut_ble_state();

    ble_state.get_devices().await
}
*/

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn bluetooth_scan(
    state: State<'_, Arc<RwLock<BrickUIBluetoothState>>>,
    duration: Option<u8>,
) -> Result<HashMap<String, Device>, String> {
    let mut state_guard = state.write().await;
    state_guard.scanning = true;

    let new_devices = scan(duration).await?;

    for (addr, new_dev) in new_devices {
        state_guard.devices
            .entry(addr)
            .and_modify(|existing| existing.merge_from(new_dev.clone()))
            .or_insert(new_dev);
    }

    state_guard.scanning = false;

    for (_address, device) in &state_guard.devices {
        println!("{}", device);
    }

    Ok(state_guard.devices.clone())
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn bluetooth_win32_scan(
    state: State<'_, Arc<RwLock<BrickUIBluetoothState>>>,
    duration: Option<u8>,
) -> Result<HashMap<String, Device>, String> {
    let mut state_guard = state.write().await;
    state_guard.scanning = true;

    let new_devices = spawn_blocking(move || scan_win32(duration))
        .await?
        .into_iter()
        .map(|device| {
            let address = device.address.clone();
            (address, Device::Win32(device))
        })
        .collect::<HashMap<String, Device>>();
    
    for (addr, new_dev) in new_devices {
        state_guard.devices
            .entry(addr)
            .and_modify(|existing| existing.merge_from(new_dev.clone()))
            .or_insert(new_dev);
    }

    state_guard.scanning = false;

    let win32_devices = state_guard
        .devices
        .iter()
        .filter_map(|(k, v)| match v {
            Device::Win32(_) => Some((k.clone(), v.clone())),
            _ => None,
        })
        .collect::<HashMap<String, Device>>();

    Ok(win32_devices)
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn bluetooth_connect(
    app_handle: AppHandle,
    state: State<'_, Arc<RwLock<BrickUIBluetoothState>>>,
    address: String,
) -> Result<(), String> {
    let mut state_guard = state.write().await;

    let device = state_guard.devices
        .get_mut(&address)
        .ok_or_else(|| format!("Device {address} not found!"))?;

    match device {
        Device::Win32(win32_device) => {
            let parent_window = app_handle
                .get_window("main")
                .and_then(|window| window.hwnd().ok());

            println!("{parent_window:?}");

            win32_device.connect(parent_window)?;
            Ok(())
        },
        Device::WinRT(winrt_device) => {
            winrt_device.connect()?;
            Ok(())
        }
    }
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn bluetooth_disconnect(
    state: State<'_, Arc<RwLock<BrickUIBluetoothState>>>,
    address: String,
) -> Result<(), String> {
    let mut state_guard = state.write().await;

    let device = state_guard.devices
        .get_mut(&address)
        .ok_or_else(|| format!("Device {address} not found!"))?;

    match device {
        Device::Win32(win32_device) => {
            win32_device.disconnect()?;
            Ok(())
        },
        Device::WinRT(winrt_device) => {
            winrt_device.disconnect()?;
            Ok(())
        }
    }
}