use std::{collections::HashMap, sync::Arc};

use tauri::{AppHandle, Manager as _, State};
use tokio::sync::RwLock;

use crate::{
    state::bluetooth::BrickUIBluetoothState,
    utils::spawn_blocking,
    winapi::bluetooth::{
        Device, scan,
        win32::{Win32Device, scan_win32},
        winrt::{WinRTDevice, scan_winrt},
    },
};

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn bluetooth_scan(
    state: State<'_, Arc<RwLock<BrickUIBluetoothState>>>,
    app_handle: AppHandle,
    duration: Option<u8>,
) -> Result<HashMap<String, Device>, String> {
    let mut state_guard = state.write().await;
    state_guard.scanning = true;

    let new_devices = scan(app_handle, duration).await?;

    for (addr, new_dev) in new_devices {
        state_guard
            .devices
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
) -> Result<HashMap<String, Win32Device>, String> {
    let mut state_guard = state.write().await;
    state_guard.scanning = true;

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

    state_guard.scanning = false;

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
    state: State<'_, Arc<RwLock<BrickUIBluetoothState>>>,
    app_handle: AppHandle,
) -> Result<HashMap<String, WinRTDevice>, String> {
    let mut state_guard = state.write().await;
    state_guard.scanning = true;

    let new_devices = scan_winrt(app_handle)
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

    state_guard.scanning = false;

    let winrt_devices = state_guard
        .devices
        .iter()
        .filter_map(|(addr, device)| device.winrt.clone().map(|winrt| (addr.clone(), winrt)))
        .collect::<HashMap<String, WinRTDevice>>();

    Ok(winrt_devices)
}

pub async fn bluetooth_devices() {}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn bluetooth_register(
    state: State<'_, Arc<RwLock<BrickUIBluetoothState>>>,
    address: String,
) -> Result<(), String> {
    let mut state_guard = state.write().await;

    let device = state_guard
        .devices
        .get_mut(&address)
        .ok_or_else(|| format!("Device {address} not found!"))?;

    device.register().await
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn bluetooth_unregister(
    state: State<'_, Arc<RwLock<BrickUIBluetoothState>>>,
    address: String,
) -> Result<(), String> {
    let mut state_guard = state.write().await;

    let device = state_guard
        .devices
        .get_mut(&address)
        .ok_or_else(|| format!("Device {address} not found!"))?;

    device.unregister().await
}

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
