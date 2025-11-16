use serde::Serialize;

use crate::winapi::bluetooth::classic::{async_scan_classic, scan_classic};

pub mod ble;
pub mod classic;

pub trait BTDevice {
    fn connect(&mut self) -> Result<(), String>;
    fn disconnect(&mut self) -> Result<(), String>;
    fn read(&mut self, uuid: &str) -> Result<Vec<u8>, String>;
    fn write(&mut self, uuid: &str, data: &[u8]) -> Result<(), String>;
}

#[derive(Clone)]
pub struct BTState {}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Device {
    Classic(classic::ClassicDevice),
    Ble(ble::BleDevice),
}

impl BTState {
    pub async fn new() -> Result<Self, String> {
        Ok(Self {})
    }
}


pub async fn scan(duration: Option<u8>) -> Result<Vec<Device>, String> {
    let classic_devices = async_scan_classic(duration)
        .await?
        .into_iter()
        .map(Device::Classic)
        .collect();

    Ok(classic_devices)
}