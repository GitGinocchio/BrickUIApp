use std::collections::HashMap;

use windows::Devices::Enumeration::DeviceWatcher;

use crate::winapi::bluetooth::Device;

#[derive(Debug)]
pub struct BluetoothState {
    pub watcher: Option<DeviceWatcher>,
    pub devices: HashMap<String, Device>
}

impl BluetoothState {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            watcher: None,
            devices: HashMap::new()
        })
    }
}
