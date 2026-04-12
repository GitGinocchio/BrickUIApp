use std::{collections::HashMap, sync::Arc};

use tauri::{AppHandle, State};
use tokio::sync::RwLock;

use crate::{
    state::bluetooth::BrickUIBluetoothState,
    winapi::bluetooth::{
        Device, PairingMessage, winrt::{pair_provide_pin, pair_confirm, pair_provide_address}
    },
};

/*
#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn bluetooth_scan(
    state: State<'_, Arc<RwLock<BrickUIBluetoothState>>>,
    _app_handle: AppHandle,
    duration: Option<u8>,
) -> Result<HashMap<String, Device>, String> {
    {
        let mut state_guard = state.write().await;

        println!("scanning");
        let new_devices = scan(duration).await?;

        for (addr, new_dev) in new_devices {
            state_guard
                .devices
                .entry(addr)
                .and_modify(|existing| existing.merge_from(new_dev.clone()))
                .or_insert(new_dev);
        }

        for (_address, device) in &state_guard.devices {
            println!("{}", device);
        }

        Ok(state_guard.devices.clone())
    }
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn bluetooth_win32_scan(
    state: State<'_, Arc<RwLock<BrickUIBluetoothState>>>,
    duration: Option<u8>,
) -> Result<HashMap<String, Win32Device>, String> {
    let mut state_guard = state.write().await;

    let new_devices = spawn_blocking(move || scan_win32(duration))
        .await?
        .into_iter()
        .map(|device| {
            let address = device.address.clone();
            (address, Device::new(Some(device), None))
        })
        .collect::<HashMap<String, Device>>();

    for (addr, new_dev) in new_devices {
        state_guard
            .devices
            .entry(addr)
            .and_modify(|existing| existing.merge_from(new_dev.clone()))
            .or_insert(new_dev);
    }

    let win32_devices = state_guard
        .devices
        .iter()
        .filter_map(|(addr, device)| device.win32.clone().map(|win32| (addr.clone(), win32)))
        .collect::<HashMap<String, Win32Device>>();

    Ok(win32_devices)
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn bluetooth_winrt_scan(
    state: State<'_, Arc<RwLock<BrickUIBluetoothState>>>
) -> Result<HashMap<String, WinRTDevice>, String> {
    let mut state_guard = state.write().await;

    let new_devices = scan_winrt()
        .await?
        .into_iter()
        .map(|device| {
            let address = device.address.clone();
            (address, Device::new(None, Some(device)))
        })
        .collect::<HashMap<String, Device>>();

    for (addr, new_dev) in new_devices {
        state_guard
            .devices
            .entry(addr)
            .and_modify(|existing| existing.merge_from(new_dev.clone()))
            .or_insert(new_dev);
    }

    let winrt_devices = state_guard
        .devices
        .iter()
        .filter_map(|(addr, device)| device.winrt.clone().map(|winrt| (addr.clone(), winrt)))
        .collect::<HashMap<String, WinRTDevice>>();

    Ok(winrt_devices)
}
*/

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn bluetooth_get_devices(
    state: State<'_, Arc<RwLock<BrickUIBluetoothState>>>
) -> Result<HashMap<String, Device>, String> {
    let guard = state.read().await;
    let devices = guard.devices.clone();
    for device in &devices {
        println!("{:?}, {:?}", device.0, device.1);
    }
    Ok(devices)
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn bluetooth_pair(
    state: State<'_, Arc<RwLock<BrickUIBluetoothState>>>,
    app_handle: AppHandle,
    address: String,
) -> Result<PairingMessage, String> {
    let mut state_guard = state.write().await;

    let device = state_guard
        .devices
        .get_mut(&address)
        .ok_or_else(|| format!("Device {address} not found!"))?;

    device.pair(app_handle).await
}

#[tauri::command]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn bluetooth_pair_provide_pin(
    pin: String
) -> Result<(), String> {
    pair_provide_pin(pin)
}

#[tauri::command]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn bluetooth_pair_provide_address(
    address: String
) -> Result<(), String> {
    pair_provide_address(address)
}


#[tauri::command]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn bluetooth_pair_confirm() -> Result<(), String> {
    pair_confirm()
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn bluetooth_unpair(
    state: State<'_, Arc<RwLock<BrickUIBluetoothState>>>,
    address: String,
) -> Result<(), String> {
    let mut state_guard = state.write().await;

    let device = state_guard
        .devices
        .get_mut(&address)
        .ok_or_else(|| format!("Device {address} not found!"))?;

    device.unpair()
}

/*
#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn bluetooth_connect(
    state: State<'_, Arc<RwLock<BrickUIBluetoothState>>>,
    address: String,
) -> Result<(), String> {
    let mut state_guard = state.write().await;

    let device = state_guard
        .devices
        .get_mut(&address)
        .ok_or_else(|| format!("Device {address} not found!"))?;

    device.connect().await
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn bluetooth_disconnect(
    state: State<'_, Arc<RwLock<BrickUIBluetoothState>>>,
    address: String,
) -> Result<(), String> {
    let mut state_guard = state.write().await;

    let device = state_guard
        .devices
        .get_mut(&address)
        .ok_or_else(|| format!("Device {address} not found!"))?;

    device.disconnect().await
}
*/
