use std::collections::HashMap;

use crate::winapi::bluetooth::Device;

#[derive(Clone, Debug)]
pub struct BrickUIBluetoothState {
    pub devices: HashMap<String, Device>,
    pub scanning: bool,
}

impl BrickUIBluetoothState {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            devices: HashMap::new(),
            scanning: false,
        })
    }
}
