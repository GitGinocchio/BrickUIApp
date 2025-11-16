use btleplug::api::{Central, CentralState, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use windows::Devices::Enumeration::DeviceWatcher;
use windows::Foundation::TypedEventHandler;
use windows::Win32::Devices::Bluetooth::{BLUETOOTH_DEVICE_INFO, BLUETOOTH_DEVICE_SEARCH_PARAMS, BLUETOOTH_FIND_RADIO_PARAMS, BluetoothFindDeviceClose, BluetoothFindFirstDevice, BluetoothFindFirstRadio, BluetoothFindNextDevice, BluetoothFindNextRadio, BluetoothFindRadioClose};
use windows::Win32::Foundation::HANDLE;
use windows::core::{HSTRING, Ref};
use std::collections::HashMap;
use std::sync::Arc;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct AdapterInfo {
    pub name: String,
    pub powered: String,
    pub address: Option<String>, // Alcuni adapter potrebbero non avere un address
    pub adapter_type: String,    // es. "WinRT", "BlueZ", ecc.
}

impl AdapterInfo {
    pub async fn from_adapter(adapter: &Adapter) -> Result<Self, String> {
        let info = adapter.adapter_info()
            .await
            .map_err(|e| format!("Error obtaining adapter info: {e}"))?;
        let state = adapter.adapter_state()
            .await
            .map_err(|e| format!("Error obtaining adapter state: {e}"))?;

        let powered = match state {
            CentralState::PoweredOff => "Powered Off",
            CentralState::PoweredOn => "PoweredOn",
            CentralState::Unknown => "Unknown"
        };

        Ok(AdapterInfo {
            name: info.clone(),
            powered: powered.into(),
            address: None, // WinRT e altri backend spesso non espongono indirizzo
            adapter_type: info, // puoi personalizzare se vuoi includere tipo
        })
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct DeviceInfo {
    pub name: String,
    pub address: String,
    pub address_type: Option<String>,
    pub class: Option<u32>,
    pub rssi: Option<i16>,
    pub manufacturer_data: HashMap<u16, Vec<u8>>,
    pub service_data: HashMap<Uuid, Vec<u8>>,
    pub services: Vec<Uuid>,
    pub tx_power_level: Option<i16>
}

impl DeviceInfo {
    pub async fn from_peripheral(peripheral: &Peripheral) -> Result<Self, String> {
        let properties = peripheral
            .properties()
            .await
            .map_err(|e| format!("Error obtaining device properties: {e}"))?
            .ok_or_else(|| format!("No properties was found for this device"))?;
        
        let name = properties.local_name.unwrap_or_else(|| "Unknown".to_string());
        let address_type = properties.address_type.map(|t| format!("{t:?}"));
        let address = properties.address.to_string();
        let rssi = properties.rssi;
        let class = properties.class;
        let manufacturer_data = properties.manufacturer_data;
        let service_data = properties.service_data;
        let services = properties.services;
        let tx_power_level = properties.tx_power_level;

        Ok(DeviceInfo {
            name,
            address_type,
            address,
            class,
            rssi,
            manufacturer_data,
            service_data,
            services,
            tx_power_level
        })
    }
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            name: "".into(),
            address: "".into(),
            address_type: None,
            class: None,
            rssi: None,
            manufacturer_data: HashMap::new(),
            service_data: HashMap::new(),
            services: vec![],
            tx_power_level: None
        }
    }
}

#[derive(Clone, Debug)]
pub struct BTState {
    default_adapter: Option<Adapter>,
    adapters: HashMap<usize, Adapter>,
    discovered: Vec<Peripheral>
}

impl BTState {
    pub async fn new() -> Result<Self, String> {
        let manager = Manager::new()
            .await
            .map_err(|e| format!("Could not instantiate Manager: {e}"))?;

        let adapters: HashMap<usize, Adapter> = manager.adapters()
            .await
            .map_err(|e| format!("Error obtaining adapters: {e}"))?
            .iter()
            .enumerate()
            .map(|(id, adapter)| (id, adapter.clone()))
            .collect();

        let default_adapter = adapters
            .clone()
            .into_iter()
            .next()
            .map(|(_, adapter)| adapter);

        if let Some(adapter) = &default_adapter {
            let info = adapter.adapter_info()
                .await
                .map_err(|e| format!("Error obtaining adapter info: {e}"))?;
        }

        Ok(Self {
            adapters,
            default_adapter,
            discovered: vec![],
        })
    }

    pub fn scan_classic(&self) -> Result<Vec<(String, String)>, String> {
        let mut devices = Vec::new();

        unsafe {
            // Enumerazione dei radio
            let mut radio_params = BLUETOOTH_FIND_RADIO_PARAMS { dwSize: std::mem::size_of::<BLUETOOTH_FIND_RADIO_PARAMS>() as u32 };
            let mut radio_handle: HANDLE = HANDLE::default();
            let radio_enum = BluetoothFindFirstRadio(&mut radio_params, &mut radio_handle)
                .map_err(|e| format!("BluetoothFindFirstRadioError: {e}"))?;
            if radio_enum.is_invalid() {
                return Err("No Bluetooth radios found.".into());
            }

            loop {
                // Preparazione dei parametri di ricerca dispositivi
                let search_params = BLUETOOTH_DEVICE_SEARCH_PARAMS {
                    dwSize: std::mem::size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32,
                    fReturnAuthenticated: true.into(),
                    fReturnRemembered: true.into(),
                    fReturnUnknown: true.into(),
                    fReturnConnected: true.into(),
                    fIssueInquiry: true.into(),
                    cTimeoutMultiplier: 4,
                    hRadio: radio_handle,
                };

                let mut device_info = BLUETOOTH_DEVICE_INFO { dwSize: std::mem::size_of::<BLUETOOTH_DEVICE_INFO>() as u32, ..Default::default() };

                // Scansione dei dispositivi
                let h_find = BluetoothFindFirstDevice(&search_params, &mut device_info)
                    .map_err(|e| format!("BluetoothFindFirstDeviceError: {e}"))?;
                if !h_find.is_invalid() {
                    loop {
                        let name = String::from_utf16_lossy(
                            &device_info.szName.iter().take_while(|&&c| c != 0).cloned().collect::<Vec<u16>>()
                        );
                        let address = format!(
                            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                            device_info.Address.Anonymous.rgBytes[5],
                            device_info.Address.Anonymous.rgBytes[4],
                            device_info.Address.Anonymous.rgBytes[3],
                            device_info.Address.Anonymous.rgBytes[2],
                            device_info.Address.Anonymous.rgBytes[1],
                            device_info.Address.Anonymous.rgBytes[0],
                        );

                        devices.push((address, name));

                        if !BluetoothFindNextDevice(h_find, &mut device_info).is_ok() {
                            break;
                        }
                    }
                    BluetoothFindDeviceClose(h_find).map_err(|e| format!("BluetoothFindDeviceCloseError: {e}"))?;
                }

                // Passa al prossimo radio
                let mut next_radio: HANDLE = HANDLE::default();
                if !BluetoothFindNextRadio(radio_enum, &mut next_radio).is_ok() {
                    break;
                }
                radio_handle = next_radio;
            }

            BluetoothFindRadioClose(radio_enum).map_err(|e| format!("BluetoothFindRadioCloseError: {e}"))?;
        }

        Ok(devices)
    }

    pub async fn scan_ble(&mut self, duration: u64, adapter_id: Option<usize>) -> Result<(), String> {
        let adapter = match adapter_id {
            Some(id) => {
                if let Some(adapter) = self.adapters.get(&id) {
                    adapter
                }
                else {
                    self.default_adapter
                        .as_ref()
                        .ok_or_else(|| "Could not find an available adapter")?
                }
            },
            None => self.default_adapter
                .as_ref()
                .ok_or_else(|| "Could not find an available adapter")?
        };

        adapter
            .start_scan(ScanFilter::default())
            .await
            .map_err(|e| format!("Error starting scan: {e}"))?;

        sleep(Duration::from_secs(duration)).await;

        adapter
            .stop_scan()
            .await
            .map_err(|e| format!("Error stopping scan: {e}"))?;

        let peripherals = adapter
            .peripherals()
            .await
            .map_err(|e| format!("Error while retrieving peripherals: {e}"))?;

        for p in &peripherals {
            if let Some(props) = p.properties().await.unwrap() {
                println!("Props: {:?}", props);
            } else {
                println!("No properties yet for {:?}", p.id());
            }
        }

        self.discovered = peripherals;

        Ok(())
    }

    pub async fn get_default_adapter(&self) -> Result<Option<AdapterInfo>, String> {
        match &self.default_adapter {
            Some(adapter) => {
                Ok(Some(
                    AdapterInfo::from_adapter(&adapter)
                    .await?
                ))
            },
            None => Ok(None)
        }
    }

    pub async fn get_adapters(&self) -> Result<HashMap<usize,AdapterInfo>, String> {
        let mut adapters_info = HashMap::new();

        for (id, adapter) in &self.adapters {
            let info = AdapterInfo::from_adapter(adapter).await?;
            adapters_info.insert(*id, info);
        }

        Ok(adapters_info)
    }

    pub async fn get_devices(&mut self) -> Result<Vec<DeviceInfo>, String> {
        let mut devices_info = Vec::new();

        self.scan_classic_win32()?.iter().for_each(|device| {
            let device_name = device.1.clone();
            let device_id = device.0.clone();

            devices_info.push(DeviceInfo { 
                name: format!("Win32: {device_name} ({device_id})"),
                ..Default::default() 
            });
        });

        for device in &self.discovered {
            let info = DeviceInfo::from_peripheral(device).await?;
            devices_info.push(info);
        }

        Ok(devices_info)
    }

    // Da sistemare:

    pub async fn connect_device(&self, device: &Peripheral) -> Result<(), String> {
        if device.is_connected().await.map_err(|e| e.to_string())? {
            return Ok(());
        }
        device.connect().await.map_err(|e| format!("Failed to connect: {e}"))?;
        device.discover_services().await.map_err(|e| format!("Service discovery failed: {e}"))?;
        Ok(())
    }

    pub async fn disconnect_device(&self, device: &Peripheral) -> Result<(), String> {
        if device.is_connected().await.map_err(|e| e.to_string())? {
            device.disconnect().await.map_err(|e| format!("Failed to disconnect: {e}"))?;
        }
        Ok(())
    }

    pub async fn read_characteristic(&self, device: &Peripheral, uuid: uuid::Uuid) -> Result<Vec<u8>, String> {
        let chars = device.characteristics();
        if let Some(characteristic) = chars.iter().find(|c| c.uuid == uuid) {
            let value = device.read(characteristic).await.map_err(|e| e.to_string())?;
            Ok(value)
        } else {
            Err("Characteristic not found".into())
        }
    }

    pub async fn write_characteristic(&self, device: &Peripheral, uuid: uuid::Uuid, data: &[u8]) -> Result<(), String> {
        let chars = device.characteristics();
        if let Some(characteristic) = chars.iter().find(|c| c.uuid == uuid) {
            device.write(characteristic, data, btleplug::api::WriteType::WithResponse)
                .await
                .map_err(|e| e.to_string())
        } else {
            Err("Characteristic not found".into())
        }
    }
}
