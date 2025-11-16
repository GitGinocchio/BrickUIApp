use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::Serialize;
use tauri::async_runtime::spawn_blocking;
use windows::Win32::{
    Devices::Bluetooth::{
        BLUETOOTH_DEVICE_INFO, BLUETOOTH_DEVICE_SEARCH_PARAMS, BLUETOOTH_FIND_RADIO_PARAMS,
        BluetoothAuthenticateDevice, BluetoothFindDeviceClose, BluetoothFindFirstDevice,
        BluetoothFindFirstRadio, BluetoothFindNextDevice, BluetoothFindNextRadio,
        BluetoothFindRadioClose, BluetoothRemoveDevice,
    },
    Foundation::{ERROR_SUCCESS, HANDLE, SYSTEMTIME},
};

use crate::winapi::bluetooth::BTDevice;

fn systemtime_to_naive(s: &SYSTEMTIME) -> Option<NaiveDateTime> {
    NaiveDate::from_ymd_opt(s.wYear as i32, s.wMonth.into(), s.wDay.into()).and_then(|date| {
        NaiveTime::from_hms_milli_opt(
            s.wHour.into(),
            s.wMinute.into(),
            s.wSecond.into(),
            s.wMilliseconds.into(),
        )
        .map(|time| NaiveDateTime::new(date, time))
    })
}

fn systemtime_to_timestamp(s: &SYSTEMTIME) -> Option<i64> {
    let naive = systemtime_to_naive(s)?;
    Some(naive.and_utc().timestamp())
}

fn parse_device_class(cod: u32) -> (u8, u8, u16) {
    let major = ((cod >> 8) & 0x1F) as u8;
    let minor = ((cod >> 2) & 0x3F) as u8;
    let services = ((cod >> 13) & 0x7FF) as u16;

    (major, minor, services)
}

fn parse_minor_class(major: MajorDeviceClass, cod: u32) -> MinorDeviceClass {
    let minor = ((cod >> 2) & 0x3F) as u8;

    match major {
        MajorDeviceClass::Computer => match minor {
            0x00 => MinorDeviceClass::ComputerUncategorized,
            0x01 => MinorDeviceClass::DesktopWorkstation,
            0x02 => MinorDeviceClass::Server,
            0x03 => MinorDeviceClass::Laptop,
            0x04 => MinorDeviceClass::HandheldPcPda,
            0x05 => MinorDeviceClass::PalmSizedPcPda,
            0x06 => MinorDeviceClass::WearableComputer,
            _ => MinorDeviceClass::Uncategorized,
        },

        MajorDeviceClass::Phone => match minor {
            0x00 => MinorDeviceClass::PhoneUncategorized,
            0x01 => MinorDeviceClass::Cellular,
            0x02 => MinorDeviceClass::Cordless,
            0x03 => MinorDeviceClass::Smartphone,
            0x04 => MinorDeviceClass::Modem,
            0x05 => MinorDeviceClass::IsdnAccess,
            _ => MinorDeviceClass::Uncategorized,
        },

        MajorDeviceClass::AudioVideo => match minor {
            0x00 => MinorDeviceClass::AudioVideoUncategorized,
            0x01 => MinorDeviceClass::WearableHeadsetDevice,
            0x02 => MinorDeviceClass::HandsFreeDevice,
            0x04 => MinorDeviceClass::Microphone,
            0x05 => MinorDeviceClass::Loudspeaker,
            0x06 => MinorDeviceClass::Headphones,
            0x07 => MinorDeviceClass::PortableAudio,
            0x08 => MinorDeviceClass::CarAudio,
            0x09 => MinorDeviceClass::SetTopBox,
            0x0A => MinorDeviceClass::HiFiAudioDevice,
            0x0B => MinorDeviceClass::Vcr,
            0x0C => MinorDeviceClass::VideoCamera,
            0x0D => MinorDeviceClass::Camcorder,
            0x0E => MinorDeviceClass::VideoMonitor,
            0x0F => MinorDeviceClass::VideoDisplayAndLoudspeaker,
            0x10 => MinorDeviceClass::VideoConferencing,
            0x12 => MinorDeviceClass::GamingToy,
            _ => MinorDeviceClass::AudioVideoUncategorized,
        },

        MajorDeviceClass::Peripheral => {
            let is_keyboard = (minor & 0x10) != 0;
            let is_pointing = (minor & 0x20) != 0;

            match (is_keyboard, is_pointing) {
                (true, false) => MinorDeviceClass::Keyboard,
                (false, true) => MinorDeviceClass::PointingDevice,
                (true, true) => MinorDeviceClass::ComboKeyboardPointingDevice,
                _ => MinorDeviceClass::PeripheralUncategorized,
            }
        }

        MajorDeviceClass::Imaging => match minor {
            0x04 => MinorDeviceClass::ImagingDisplay,
            0x08 => MinorDeviceClass::ImagingCamera,
            0x10 => MinorDeviceClass::ImagingScanner,
            0x20 => MinorDeviceClass::ImagingPrinter,
            _ => MinorDeviceClass::Uncategorized,
        },

        MajorDeviceClass::Wearable => match minor {
            0x01 => MinorDeviceClass::WearableWristWatch,
            0x02 => MinorDeviceClass::WearablePager,
            0x03 => MinorDeviceClass::WearableJacket,
            0x04 => MinorDeviceClass::WearableHelmet,
            0x05 => MinorDeviceClass::WearableGlasses,
            _ => MinorDeviceClass::Uncategorized,
        },

        MajorDeviceClass::Toy => match minor {
            0x01 => MinorDeviceClass::ToyRobot,
            0x02 => MinorDeviceClass::ToyVehicle,
            0x03 => MinorDeviceClass::ToyDoll,
            0x04 => MinorDeviceClass::ToyController,
            0x05 => MinorDeviceClass::ToyGame,
            _ => MinorDeviceClass::Uncategorized,
        },

        _ => MinorDeviceClass::Uncategorized,
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum MajorDeviceClass {
    Generic,
    Computer,
    Phone,
    Network,
    AudioVideo,
    Peripheral,
    Imaging,
    Wearable,
    Toy,
    Uncategorized,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum MinorDeviceClass {
    // For Major: Computer (0x01)
    ComputerUncategorized,
    DesktopWorkstation,
    Server,
    Laptop,
    HandheldPcPda,
    PalmSizedPcPda,
    WearableComputer,

    // For Major: Phone (0x02)
    PhoneUncategorized,
    Cellular,
    Cordless,
    Smartphone,
    Modem,
    IsdnAccess,

    // For Major: Audio/Video (0x04)
    AudioVideoUncategorized,
    WearableHeadsetDevice,
    HandsFreeDevice,
    Microphone,
    Loudspeaker,
    Headphones,
    PortableAudio,
    CarAudio,
    SetTopBox,
    HiFiAudioDevice,
    Vcr,
    VideoCamera,
    Camcorder,
    VideoMonitor,
    VideoDisplayAndLoudspeaker,
    VideoConferencing,
    GamingToy,

    // For Major: Peripheral (0x05)
    PeripheralUncategorized,
    Keyboard,
    PointingDevice,
    ComboKeyboardPointingDevice,

    // For Major: Imaging (0x06)
    ImagingDisplay,
    ImagingCamera,
    ImagingScanner,
    ImagingPrinter,

    // For Major: Wearable (0x07)
    WearableWristWatch,
    WearablePager,
    WearableJacket,
    WearableHelmet,
    WearableGlasses,

    // For Major: Toy (0x08)
    ToyRobot,
    ToyVehicle,
    ToyDoll,
    ToyController,
    ToyGame,

    // Uncategorized (Major = 0x1F)
    Uncategorized,
}

impl From<u8> for MajorDeviceClass {
    fn from(v: u8) -> Self {
        match v {
            0x01 => Self::Computer,
            0x02 => Self::Phone,
            0x03 => Self::Network,
            0x04 => Self::AudioVideo,
            0x05 => Self::Peripheral,
            0x06 => Self::Imaging,
            0x07 => Self::Wearable,
            0x08 => Self::Toy,
            _ => Self::Uncategorized,
        }
    }
}

impl From<(MajorDeviceClass, u32)> for MinorDeviceClass {
    fn from((major, cod): (MajorDeviceClass, u32)) -> Self {
        parse_minor_class(major, cod)
    }
}

#[derive(Serialize, Clone)]
pub struct ClassicDevice {
    pub name: String,
    pub address: String,
    pub authenticated: bool,
    pub remembered: bool,
    pub connected: bool,
    pub last_seen: Option<i64>,
    pub last_used: Option<i64>,
    pub device_class: u32,
    pub device_type: (MajorDeviceClass, MinorDeviceClass),
    pub services: u16,

    #[serde(skip)]
    raw_device_info: BLUETOOTH_DEVICE_INFO,
}

impl ClassicDevice {
    pub fn from_win32(info: &BLUETOOTH_DEVICE_INFO) -> Self {
        let name = String::from_utf16_lossy(
            &info
                .szName
                .iter()
                .take_while(|&&c| c != 0)
                .cloned()
                .collect::<Vec<_>>(),
        );

        let address = unsafe {
            format!(
                "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                info.Address.Anonymous.rgBytes[5],
                info.Address.Anonymous.rgBytes[4],
                info.Address.Anonymous.rgBytes[3],
                info.Address.Anonymous.rgBytes[2],
                info.Address.Anonymous.rgBytes[1],
                info.Address.Anonymous.rgBytes[0],
            )
        };

        let cod = info.ulClassofDevice;

        let (major, minor, services) = parse_device_class(cod);

        let major_device_class = MajorDeviceClass::from(major);

        ClassicDevice {
            name,
            address,
            authenticated: info.fAuthenticated.as_bool(),
            remembered: info.fRemembered.as_bool(),
            connected: info.fConnected.as_bool(),
            last_seen: systemtime_to_timestamp(&info.stLastSeen),
            last_used: systemtime_to_timestamp(&info.stLastUsed),
            device_class: cod,
            services: services,
            device_type: (
                major_device_class.clone(),
                MinorDeviceClass::from((major_device_class, minor as u32)),
            ),
            raw_device_info: *info,
        }
    }

    pub fn from_mac(address: String) -> Self {
        todo![]
    }
}

impl BTDevice for ClassicDevice {
    fn connect(&mut self) -> Result<(), std::string::String> {
        unsafe {
            let mut adapter_params = BLUETOOTH_FIND_RADIO_PARAMS {
                dwSize: std::mem::size_of::<BLUETOOTH_FIND_RADIO_PARAMS>() as u32,
            };
            let mut adapter_handle: HANDLE = HANDLE::default();

            let radio_enum = BluetoothFindFirstRadio(&mut adapter_params, &mut adapter_handle)
                .map_err(|e| format!("BluetoothFindFirstRadioError: {e}"))?;

            loop {
                // prova ad autenticare il dispositivo con questo adapter
                let res = BluetoothAuthenticateDevice(
                    None,
                    Some(adapter_handle),
                    &mut self.raw_device_info,
                    None,
                );

                if res == ERROR_SUCCESS.0 {
                    break; // successo
                }

                // passa al prossimo adapter
                let mut next_adapter: HANDLE = HANDLE::default();

                BluetoothFindNextRadio(radio_enum, &mut next_adapter)
                    .map_err(|e| format!("BluetoothFindNextRadioError: {e}"))?;

                adapter_handle = next_adapter;
            }

            *self = ClassicDevice::from_win32(&self.raw_device_info)
        }

        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), std::string::String> {
        unsafe {
            let res = BluetoothRemoveDevice(&self.raw_device_info.Address);

            if res == ERROR_SUCCESS.0 {
                return Err(format!("BluetoothRemoveDeviceError: {res}"));
            }
        }

        Ok(())
    }

    fn read(&mut self, uuid: &str) -> Result<Vec<u8>, String> {
        todo!()
    }

    fn write(&mut self, uuid: &str, data: &[u8]) -> Result<(), std::string::String> {
        todo!()
    }
}

pub fn scan_classic(duration: Option<u8>) -> Result<Vec<ClassicDevice>, String> {
    let mut devices = Vec::new();

    unsafe {
        // Find radios
        let mut radio_params = BLUETOOTH_FIND_RADIO_PARAMS {
            dwSize: std::mem::size_of::<BLUETOOTH_FIND_RADIO_PARAMS>() as u32,
        };
        let mut radio_handle: HANDLE = HANDLE::default();

        let radio_enum = match BluetoothFindFirstRadio(&mut radio_params, &mut radio_handle) {
            Ok(h) => h,
            Err(_) => {
                // Nessun radio → non è un errore → return lista vuota
                return Ok(devices);
            }
        };

        if radio_enum.is_invalid() {
            return Ok(devices);
        }

        loop {
            // Device search parameters
            let search_params = BLUETOOTH_DEVICE_SEARCH_PARAMS {
                dwSize: std::mem::size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32,
                fReturnAuthenticated: true.into(),
                fReturnRemembered: true.into(),
                fReturnUnknown: true.into(),
                fReturnConnected: true.into(),
                fIssueInquiry: true.into(),
                cTimeoutMultiplier: duration.unwrap_or(5),
                hRadio: radio_handle,
            };

            let mut device_info = BLUETOOTH_DEVICE_INFO {
                dwSize: std::mem::size_of::<BLUETOOTH_DEVICE_INFO>() as u32,
                ..Default::default()
            };

            // Find first device
            let h_find = match BluetoothFindFirstDevice(&search_params, &mut device_info) {
                Ok(h) => h,
                Err(_) => {
                    // Nessun device per questo radio → passa al prossimo radio
                    let mut next_radio = HANDLE::default();
                    if BluetoothFindNextRadio(radio_enum, &mut next_radio).is_ok() {
                        radio_handle = next_radio;
                        continue;
                    } else {
                        break;
                    }
                }
            };

            if !h_find.is_invalid() {
                loop {
                    devices.push(ClassicDevice::from_win32(&device_info));

                    if !BluetoothFindNextDevice(h_find, &mut device_info).is_ok() {
                        break;
                    }
                }
                // Chiudi handle
                let _ = BluetoothFindDeviceClose(h_find);
            }

            // Next radio
            let mut next_radio = HANDLE::default();
            if !BluetoothFindNextRadio(radio_enum, &mut next_radio).is_ok() {
                break;
            }
            radio_handle = next_radio;
        }

        let _ = BluetoothFindRadioClose(radio_enum);
    }

    Ok(devices)
}

pub async fn async_scan_classic(duration: Option<u8>) -> Result<Vec<ClassicDevice>, String> {
    let devices =
        tauri::async_runtime::spawn_blocking(move || scan_classic(duration))
            .await
            .map_err(|e| format!("Task join error: {e}"))??;

    Ok(devices)
}
