use serde::Serialize;
use windows::Devices::{Bluetooth::BluetoothDevice, Enumeration::DeviceInformation};

//use crate::winapi::bluetooth::BTDevice;

#[derive(Serialize, Clone, Debug)]
pub struct WinRTDevice {
}

impl WinRTDevice {
    pub fn register(&mut self) -> Result<(), std::string::String> { todo!() }
    pub fn remove(&mut self) -> Result<(), std::string::String> { todo!() }

    pub fn connect(&mut self) -> Result<(), String> { todo!() }
    pub fn disconnect(&mut self) -> Result<(), String> { todo!() }

    pub fn read(&mut self) -> Result<Vec<u8>, String> { todo!() }
    pub fn write(&mut self, data: &[u8]) -> Result<(), std::string::String> { todo!() }
}

pub async fn scan_winrt() -> Result<(), String> {
    let selector = BluetoothDevice
        ::GetDeviceSelectorFromPairingState(false)
        .map_err(|e| format!("{e}"))?;

    let devices = DeviceInformation
        ::FindAllAsyncAqsFilter(&selector)
        .map_err(|e| format!("{e}"))?
        .await
        .map_err(|e| format!("{e}"))?;

    let size = devices.Size().map_err(|e| format!("{e}"))?;

    println!("{size}");

    for i in 0..size {
        let device = devices.GetAt(i).map_err(|e| format!("{e}"))?;
        println!("Name: {}", device.Name().map_err(|e| format!("{e}"))?);
        println!("Id:   {}", device.Id().map_err(|e| format!("{e}"))?);
        println!("Kind: {:?}", device.Kind().map_err(|e| format!("{e}"))?);
    }

    Ok(())
}