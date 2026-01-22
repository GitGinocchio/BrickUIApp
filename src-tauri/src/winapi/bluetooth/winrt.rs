use std::{fmt, sync::{Mutex, mpsc::RecvTimeoutError}, time::Duration};

use derivative::Derivative;
use once_cell::sync::Lazy;
use tauri::{AppHandle, Emitter as _};
use std::sync::mpsc;

use serde::Serialize;
use windows::{
    Devices::{
        Enumeration::{
            DeviceInformation, 
            DeviceInformationCustomPairing, 
            DeviceInformationPairing, 
            DevicePairingKinds,
            DevicePairingRequestedEventArgs, 
            DevicePairingResultStatus, 
            DeviceUnpairingResultStatus
        },
    },
    Foundation::{Deferral, TypedEventHandler},
    core::HSTRING
};

use crate::{winapi::{bluetooth::{AcceptPairingMessage, PairingMessage}, com::initialize_com}};

#[derive(Debug)]
struct PendingPairing {
    message: PairingMessage,
    args: DevicePairingRequestedEventArgs,
    deferral: Deferral,
}

static PENDING_PAIRING: Lazy<Mutex<Option<PendingPairing>>> = Lazy::new(|| Mutex::new(None));
static PAIRING_SENDER: Lazy<Mutex<Option<mpsc::Sender<AcceptPairingMessage>>>> = Lazy::new(|| Mutex::new(None));

#[derive(Serialize, Clone, Derivative)]
#[derivative(Debug)]
pub struct WinRTDevice {
    pub id: String,
    pub address: String,
    pub is_default: bool,
    pub is_enabled: bool,
    pub is_paired: bool,
    pub can_pair: bool,
    pub kind: String,
    pub name: String,

    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub pairing: DeviceInformationPairing,

    pub protection_level: Option<String>,

    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub raw_info: DeviceInformation
}

impl fmt::Display for WinRTDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WinRTDevice(name={}, address={}, is_default={}, is_enabled={}, is_paired={}, can_pair={}, kind={})",
            self.name, self.address, self.is_default, self.is_enabled, self.is_paired, self.can_pair, self.kind
        )
    }
}

fn setup_pairing_handler(app_handle: AppHandle) -> TypedEventHandler<DeviceInformationCustomPairing, DevicePairingRequestedEventArgs> {
    TypedEventHandler::<DeviceInformationCustomPairing, DevicePairingRequestedEventArgs>::new(
        move |_, args| {
            let args = args.as_ref().expect("Error obtaining args");
            let deferral = args.GetDeferral().expect("Error obtaining Deferral");

            let mut pending_pairing_guard = PENDING_PAIRING.lock().expect("Pending pairing lock poisoned");

            let message = match args.PairingKind().expect("Error obtaining pairing kind") {
                DevicePairingKinds::ProvidePin => PairingMessage::ProvidePin,
                DevicePairingKinds::ConfirmOnly => PairingMessage::ConfirmOnly,
                DevicePairingKinds::ProvideAddress => PairingMessage::ProvideAddress,
                DevicePairingKinds::None => {
                    println!("[winrt_handler] PairingKinds::None → auto accept");
                    args.Accept().expect("Error accepting");
                    deferral.Complete().expect("Error completing deferral");
                    return Ok(());
                },
                DevicePairingKinds::DisplayPin => {
                    let pin = args.Pin().unwrap().to_string_lossy().to_string();
                    PairingMessage::DisplayPin { pin }
                },
                DevicePairingKinds::ConfirmPinMatch => {
                    let pin = args.Pin().unwrap().to_string_lossy().to_string();
                    PairingMessage::ConfirmPinMatch { pin }
                },
                _ => PairingMessage::Failed { message: "Unsupported kind".into() },
            };

            println!("[winrt_handler] sending message");
            app_handle
                .emit("bluetooth_pairing_request", message.clone())
                .expect("Error sending bluetooth_pairing_request event");

            *pending_pairing_guard = Some(PendingPairing {
                message: message,
                args: args.clone(),
                deferral
            });
            
            Ok(())
        },
    )
}

pub(super) fn extract_winrt_device_address(id: &str) -> String {
    id.split('#')
        .nth(1)
        .and_then(|s| s.strip_prefix("Bluetooth"))
        .and_then(|s| s.split_once('-'))
        .map(|(_, device_addr)| device_addr.to_string())
        .unwrap_or_else(|| id.to_string())
}

impl WinRTDevice {
    #[cfg_attr(feature = "profiling", tracing::instrument)]
    pub fn from_info(info: &DeviceInformation) -> Result<Self, String> {
        let glyph_thumb = info
            .GetGlyphThumbnailAsync()
            .map_err(|e| format!("GetGlyphThumbnailAsync Error: {e}"))?
            .get()
            .map_err(|e| format!("GetGlyphThumbnailAsync Error: {e}"))?;

        let thumb = info
            .GetThumbnailAsync()
            .map_err(|e| format!("GetGlyphThumbnailAsync Error: {e}"))?
            .get()
            .map_err(|e| format!("GetGlyphThumbnailAsync Error: {e}"))?;

        let id = info
            .Id()
            .map_err(|e| format!("Id Error: {e}"))?
            .to_string_lossy();

        let address = extract_winrt_device_address(&id).to_ascii_lowercase();

        let is_default = info
            .IsDefault()
            .map_err(|e| format!("IsDefault Error: {e}"))?;

        let is_enabled = info
            .IsEnabled()
            .map_err(|e| format!("IsEnabled Error: {e}"))?;

        let kind = format!("{:?}", info.Kind().map_err(|e| format!("Kind Error: {e}"))?);

        let name = info
            .Name()
            .map_err(|e| format!("Name Error: {e}"))?
            .to_string_lossy();

        let pairing = info.Pairing().map_err(|e| format!("Pairing Error: {e}"))?;
        let can_pair = pairing
            .CanPair()
            .map_err(|e| format!("CanPair Error: {e}"))?;
        let is_paired = pairing
            .IsPaired()
            .map_err(|e| format!("IsPaired Error: {e}"))?;

        let properties = info
            .Properties()
            .map_err(|e| format!("Properties Error: {e}"))?;

        println!("Properties: {properties:?}");

        /*
        let serializable_props = HashMap::new();
        for key_value in properties {
            let key = key_value.Key().map_err(|e| format!("Key Error: {e}"))?.to_string();
            let value = key_value.Value().map_err(|e| format!("Value Error: {e}"))?;
            serializable_props.insert(key, v)
        }
        */

        Ok(Self {
            id,
            address,
            is_default,
            is_enabled,
            is_paired,
            can_pair,
            kind,
            name,
            pairing,
            raw_info: info.clone(),
            protection_level: None
        })
    }

    pub fn pair(&mut self, app_handle: AppHandle) -> Result<PairingMessage, String> {
        initialize_com()?;

        if self.pairing.IsPaired().map_err(|e| format!("Error checking device IsPaired: {e}"))? {
            return Ok(PairingMessage::AlreadyPaired);
        }

        if !self.pairing.CanPair().map_err(|e| format!("Error checking device CanPair: {e}"))? {
            self.can_pair = false;
            return Ok(PairingMessage::CantPair);
        }

        let (tx, rx) = mpsc::channel();

        {
            let mut pending_pairing_guard = PENDING_PAIRING
                .lock()
                .map_err(|e| format!("Error: pending_pairing lock poisoned: {e}"))?;

            if let Some(old_pending) = pending_pairing_guard.take() {
                old_pending.deferral
                    .Complete()
                    .map_err(|e| format!("Error completing deferral: {e}"))?;
            }

            let mut pairing_sender_guard = PAIRING_SENDER
                .lock()
                .map_err(|e| format!("Error: lock poisoned: {e}"))?;

            *pairing_sender_guard = Some(tx);
        }


        let custom = self.pairing
            .Custom()
            .map_err(|e| format!("Error instantiating Custom pairing: {e}"))?;

        let handler = setup_pairing_handler(app_handle);

        // Metodo sync per registrare l'handler
        let token = custom
            .PairingRequested(&handler)
            .map_err(|e| format!("Failed to register handler: {:?}", e))?;
        
        let pair_op = custom.PairAsync(
            DevicePairingKinds::ConfirmOnly
            | DevicePairingKinds::ProvidePin
            | DevicePairingKinds::DisplayPin
            | DevicePairingKinds::ProvideAddress
            | DevicePairingKinds::ConfirmPinMatch
            | DevicePairingKinds::None
        ).map_err(|e| e.to_string())?;

        println!("[register] waiting message from frontend");
        let accepted = match rx.recv_timeout(Duration::from_secs(30)) {
            Ok(message) => {
                let pairing_guard = PENDING_PAIRING.lock().map_err(|e| format!("watchdog async: lock poisoned: {e}"))?;

                if let Some(pending) = pairing_guard.as_ref() {
                    match (&pending.message, message) {
                        (PairingMessage::ProvideAddress, AcceptPairingMessage::AcceptWithAddress(address)) => {
                            println!("[register] accepting with address: {address}");
                            pending.args
                                .AcceptWithAddress(&HSTRING::from(address))
                                .map_err(|e| format!("Error accepting with pin: {e}"))?;
                        },
                        (PairingMessage::ProvidePin, AcceptPairingMessage::AcceptWithPin(pin)) => {
                            println!("[register] accepting with pin: {pin}");
                            pending.args
                                .AcceptWithPin(&HSTRING::from(pin))
                                .map_err(|e| format!("Error accepting with pin: {e}"))?;
                        },
                        (PairingMessage::ConfirmPinMatch { .. }, AcceptPairingMessage::Accept) |
                        (PairingMessage::DisplayPin { .. }, AcceptPairingMessage::Accept) |
                        (PairingMessage::ConfirmOnly, AcceptPairingMessage::Accept) => {
                            println!("[register] accepting");
                            pending.args.Accept().map_err(|e| format!("Error accepting: {e}"))?;
                        },
                        _ => {
                            return Err("Invalid pairing response for this device".into());
                        }
                    }
                }

                true
            },

            Err(RecvTimeoutError::Timeout) => {
                eprintln!("[register] Timeout waiting for response from WinRT handler");

                false
            },

            Err(RecvTimeoutError::Disconnected) => {
                eprintln!("[register] Handshake channel disconnected");

                false
            }
        };

        println!("[register] completing deferral");

        let mut pairing_guard = PENDING_PAIRING.lock().map_err(|e| format!("Pending pairing lock poisoned: {e}"))?;
        if let Some(pending) = pairing_guard.as_ref() {
            pending.deferral
                .Complete()
                .map_err(|e| format!("[register] error completing deferral: {e}"))?;
        }

        if !accepted {
            pair_op.Cancel().map_err(|e| format!("Error canceling pair operation: {e}"))?;
            pair_op.Close().map_err(|e| format!("Error closing pair operation: {e}"))?;
        }

        let result = pair_op.get().map_err(|e| format!("Error waiting pairing async to finish: {e}"))?;

        let response = match result.Status().map_err(|e| format!("Error retreiving status: {e}"))? {
            DevicePairingResultStatus::Paired => Ok(PairingMessage::Paired),
            status => {
                Ok(PairingMessage::Failed { message: format!("Error during pairing: {:#?}", status.0) })
            }
        };

        custom.RemovePairingRequested(token).map_err(|e| format!("Error removing pairing requested handler: {e}"))?;

        // cleanup
        let mut sender_guard = PAIRING_SENDER.lock().map_err(|e| format!("Pairing sender lock poisoned: {e}"))?;
        *sender_guard = None;
        *pairing_guard = None;

        response
    }

    pub async fn unpair(&mut self) -> Result<(), std::string::String> {
        if !self.is_paired {
            return Ok(());
        }

        let result = self
            .pairing
            .UnpairAsync()
            .map_err(|e| format!("UnpairAsync error: {e}"))?
            .await
            .map_err(|e| format!("UnpairAsync await error: {e}"))?;

        match result
            .Status()
            .map_err(|e| format!("Error obtaining status: {e}"))?
        {
            DeviceUnpairingResultStatus::Unpaired => {
                self.is_paired = false;
                Ok(())
            }
            status => Err(format!("Unpairing failed: {:?}", status)),
        }
    }
}

/*
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn scan_winrt() -> Result<Vec<WinRTDevice>, String> {
    // TODO: Get Devices dovrebbe ottenere quelli gia' paired
    /*
    let selector = BluetoothDevice::GetDeviceSelectorFromConnectionStatus(BluetoothConnectionStatus::Disconnected)
        .map_err(|e| format!("{e}"))?;
    */

    //let selector = BluetoothDevice::GetDeviceSelectorFromPairingState(false)
    //    .map_err(|e| format!("{e}"))?;

    let selector = BluetoothDevice
        ::GetDeviceSelectorFromConnectionStatus(BluetoothConnectionStatus::Disconnected)
        .map_err(|e| format!("{e}"))?;

    // let selector = HSTRING::from(r#"System.Devices.Aep.ProtocolId:="{e0cbf06c-cd8b-4647-bb8a-263b43f0f974}""#);

    let devices_info = DeviceInformation::FindAllAsyncAqsFilter(&selector)
        .map_err(|e| format!("{e}"))?
        .await
        .map_err(|e| format!("{e}"))?;

    let size = devices_info.Size().map_err(|e| format!("{e}"))?;

    let mut devices = vec![];
    for i in 0..size {
        let device_info = devices_info.GetAt(i).map_err(|e| format!("{e}"))?;
        let device = WinRTDevice::from_info(&device_info)
            .map_err(|e| format!("from_winrt Error: {e}"))?;
        devices.push(device);
    }

    Ok(devices)
}
*/

pub fn pair_provide_pin(pin: String) -> Result<(), String> {
    println!("[provide_pin] Accepting");

    let guard = PAIRING_SENDER.lock().map_err(|e| format!("Pairing sender lock poisoned: {e}"))?;

    if let Some(sender) = guard.as_ref() {
        sender.send(AcceptPairingMessage::AcceptWithPin(pin)).map_err(|e| format!("Error sending AcceptPairingMessage: {e}"))?;
    }

    Ok(())
}

pub fn pair_provide_address(address: String) -> Result<(), String> {
    println!("[provide_address] Accepting");

    let guard = PAIRING_SENDER.lock().map_err(|e| format!("Pairing sender lock poisoned: {e}"))?;

    if let Some(sender) = guard.as_ref() {
        sender.send(AcceptPairingMessage::AcceptWithAddress(address)).map_err(|e| format!("Error sending AcceptPairingMessage: {e}"))?;
    }

    Ok(())
}

pub fn pair_confirm() -> Result<(), String> {
    println!("[confirm] Accepting");

    let guard = PAIRING_SENDER.lock().map_err(|e| format!("Pairing sender lock poisoned: {e}"))?;

    if let Some(sender) = guard.as_ref() {
        sender.send(AcceptPairingMessage::Accept).map_err(|e| format!("Error sending AcceptPairingMessage: {e}"))?;
    }

    Ok(())
}