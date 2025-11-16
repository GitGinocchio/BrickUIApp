use std::{collections::HashMap, sync::Arc};

use tauri::{AppHandle, State};
use tokio::sync::Mutex;

use crate::{
    state::BrickUIState,
    winapi::bluetooth::{Device, classic::{ClassicDevice, async_scan_classic}, scan},
};

/*
use crate::{state::BrickUIState, winapi::old::{AdapterInfo, DeviceInfo}};

#[tauri::command(async)]
pub async fn get_default_adapter(
    state: State<'_, Arc<Mutex<BrickUIState>>>,
) -> Result<Option<AdapterInfo>, String> {
    let state_guard = state.lock().await;
    let ble_state = state_guard.get_ble_state();

    ble_state.get_default_adapter().await
}

#[tauri::command(async)]
pub async fn get_adapters(
    state: State<'_, Arc<Mutex<BrickUIState>>>,
) -> Result<HashMap<usize, AdapterInfo>, String> {
    let state_guard = state.lock().await;
    let ble_state = state_guard.get_ble_state();

    ble_state.get_adapters().await
}

#[tauri::command(async)]
pub async fn get_devices(
    state: State<'_, Arc<Mutex<BrickUIState>>>,
) -> Result<Vec<DeviceInfo>, String> {
    let mut state_guard = state.lock().await;
    let ble_state = state_guard.get_mut_ble_state();

    ble_state.get_devices().await
}

#[tauri::command(async)]
pub async fn scan(
    state: State<'_, Arc<Mutex<BrickUIState>>>,
    duration: u64,
    adapter: Option<usize>
) -> Result<(), String> {
    let mut state_guard = state.lock().await;
    let ble_state = state_guard.get_mut_ble_state();

    ble_state.scan(duration, adapter).await
}
*/

#[tauri::command(async)]
pub async fn bluetooth_scan(
    _state: State<'_, Arc<Mutex<BrickUIState>>>,
    duration: Option<u8>,
) -> Result<Vec<Device>, String> {
    //let state_guard = state.lock().await;
    //let bt_state = state_guard.get_bluetooth_state();

    scan(duration).await
}

#[tauri::command(async)]
pub async fn bluetooth_classic_scan(
    _state: State<'_, Arc<Mutex<BrickUIState>>>,
    duration: Option<u8>,
) -> Result<Vec<ClassicDevice>, String> {
    //let state_guard = state.lock().await;
    //let bt_state = state_guard.get_bluetooth_state();

    async_scan_classic(duration).await
}
