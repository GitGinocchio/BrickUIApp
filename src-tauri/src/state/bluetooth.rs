use std::collections::HashMap;

use windows::Devices::Enumeration::DeviceWatcher;

use crate::winapi::bluetooth::Device;

#[derive(Clone, Debug)]
pub struct BrickUIBluetoothState {
    pub watcher: Option<DeviceWatcher>,
    pub devices: HashMap<String, Device>
}

impl BrickUIBluetoothState {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            watcher: None,
            devices: HashMap::new()
        })
    }
}
