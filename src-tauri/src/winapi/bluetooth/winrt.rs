use std::{fmt, sync::Mutex};

use derivative::Derivative;
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::json;
use tauri::{AppHandle, Emitter as _};
use windows::{
    Devices::{
        Bluetooth::BluetoothDevice,
        Enumeration::{
            DeviceInformation, DeviceInformationCustomPairing, DeviceInformationPairing,
            DevicePairingKinds, DevicePairingRequestedEventArgs, DevicePairingResultStatus,
            DeviceUnpairingResultStatus,
        },
    },
    Foundation::{Deferral, TypedEventHandler},
    core::{HRESULT, HSTRING},
};

use crate::winapi::com::initialize_com;

#[derive(Debug)]
struct PendingPairing {
    args: DevicePairingRequestedEventArgs,
    pin: Option<String>,
    deferral: Deferral,
}

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
    pub raw_info: DeviceInformation,

    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub app_handle: AppHandle,
}

impl fmt::Display for WinRTDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WinRTDevice(name={}, address={}, is_default={}, is_enabled={}, is_paired={}, kind={})",
            self.name, self.address, self.is_default, self.is_enabled, self.is_paired, self.kind
        )
    }
}

static PENDING_PAIRING: Lazy<Mutex<Option<PendingPairing>>> = Lazy::new(|| Mutex::new(None));

fn setup_pairing_handler()
-> TypedEventHandler<DeviceInformationCustomPairing, DevicePairingRequestedEventArgs> {
    TypedEventHandler::<DeviceInformationCustomPairing, DevicePairingRequestedEventArgs>::new(
        move |_, args| {
            initialize_com().map_err(|e| windows::core::Error::new(HRESULT(-1), e))?;

            let mut guard = PENDING_PAIRING.lock().map_err(|e| {
                windows::core::Error::new(HRESULT(-1), format!("Error: lock poisoned: {e}"))
            })?;

            if let Some(old_pending) = guard.take() {
                old_pending.deferral.Complete()?;
            }

            let args = args
                .as_ref()
                .ok_or::<windows::core::Error>(windows::core::Error::new(
                    HRESULT(-1),
                    "No args provided",
                ))?;

            let pairing_kind = args.PairingKind()?;

            println!("Received pairing request with kind: {pairing_kind:?}");

            match pairing_kind {
                DevicePairingKinds::ConfirmOnly => {
                    let deferral = args.GetDeferral()?;

                    deferral.Complete()?;

                    let pending = PendingPairing {
                        args: args.clone(),
                        pin: None,
                        deferral,
                    };

                    *guard = Some(pending);

                    //args.Accept()?;
                }
                DevicePairingKinds::ProvidePin => {
                    // Ottieni subito il deferral
                    let deferral = args.GetDeferral()?;

                    // Salva args + deferral nel pending globale
                    let pending = PendingPairing {
                        args: args.clone(),
                        pin: None,
                        deferral,
                    };

                    *guard = Some(pending);
                }
                DevicePairingKinds::DisplayPin => {
                    let deferral = args.GetDeferral()?;

                    let pin = args.Pin()?.to_string_lossy().to_string();

                    let pending = PendingPairing {
                        args: args.clone(),
                        pin: Some(pin.clone()),
                        deferral,
                    };

                    *guard = Some(pending);

                    /*
                    app_handle.emit_to(
                        "overlay",
                        "bluetooth_pairing_pin",
                        json!({ "pin": pin })
                    ).map_err(|e| windows::core::Error::new(HRESULT(-1), e.to_string()))?;
                    */

                    args.Accept()?;
                }
                kind => {
                    println!("Not handled: {kind:?}")
                }
            }
            Ok(())
        },
    )
}

fn extract_winrt_device_address(id: &str) -> String {
    id.split('#')
        .nth(1)
        .and_then(|s| s.strip_prefix("Bluetooth"))
        .and_then(|s| s.split_once('-'))
        .map(|(_, device_addr)| device_addr.to_string())
        .unwrap_or_else(|| id.to_string())
}

pub fn provide_pin(pin: String) -> Result<(), String> {
    let pending = {
        let mut guard = PENDING_PAIRING
            .lock()
            .map_err(|e| format!("Pending pairing lock poisoned: {e}"))?;
        guard.take().ok_or("No pending pairing")?
    };

    pending
        .args
        .AcceptWithPin(&HSTRING::from(pin))
        .map_err(|e| format!("Error accepting pin: {e}"))?;

    pending
        .deferral
        .Complete()
        .map_err(|e| format!("Error completing pending pairing: {e}"))?;

    Ok(())
}

impl WinRTDevice {
    #[cfg_attr(feature = "profiling", tracing::instrument)]
    pub async fn from_info_and_app_handle(
        info: &DeviceInformation,
        app_handle: AppHandle,
    ) -> Result<Self, String> {
        let glyph_thumb = info
            .GetGlyphThumbnailAsync()
            .map_err(|e| format!("GetGlyphThumbnailAsync Error: {e}"))?
            .await
            .map_err(|e| format!("GetGlyphThumbnailAsync Error: {e}"))?;

        let thumb = info
            .GetThumbnailAsync()
            .map_err(|e| format!("GetGlyphThumbnailAsync Error: {e}"))?
            .await
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
            protection_level: None,
            app_handle: app_handle,
        })
    }

    pub async fn register(&mut self) -> Result<(), std::string::String> {
        if self
            .pairing
            .IsPaired()
            .map_err(|e| format!("Error checking device IsPaired: {e}"))?
        {
            self.is_paired = true;
            self.can_pair = false;
            return Ok(());
        }

        if self
            .pairing
            .CanPair()
            .map_err(|e| format!("Error checking device CanPair: {e}"))?
        {
            if let Ok(op) = self.pairing.PairAsync() {
                if let Ok(result) = op.await {
                    if result
                        .Status()
                        .map_err(|e| format!("Error checking Status: {e}"))?
                        == DevicePairingResultStatus::Paired
                    {
                        self.is_paired = true;
                        self.can_pair = false;
                        return Ok(());
                    }
                }
            }
        }

        eprintln!("Device can't pair!");

        let custom = self
            .pairing
            .Custom()
            .map_err(|e| format!("Error instantiating Custom pairing: {e}"))?;

        // Siamo in un contesto async (Thread che possono essere avviati e interrotti al bisogno)
        // tutto quello che viene creato in questo contesto deve avere il trait Send (quindi passabile tra thread)
        // Se qualcosa non e' Send bisogna avviarlo come "task" sync e ritornare il risultato quando finisce
        // custom e' necessario anche dopo quindi la task deve ritornare custom, che e' stato preso (borrowed)
        // dalla task (grazie alla keyword move prima della closure)
        let custom =
            tokio::task::spawn_blocking::<_, Result<DeviceInformationCustomPairing, String>>(
                move || {
                    // handler COM no Send (non inviabile tra thread, quindi impossibile da usare in contesto async)
                    let handler = setup_pairing_handler();

                    // Metodo sync per registrare l'handler
                    custom
                        .PairingRequested(&handler)
                        .map_err(|e| format!("Failed to register handler: {:?}", e))?;

                    Ok(custom)
                },
            )
            .await
            .map_err(|_| "Thread panicked".to_string())??;

        println!("Pre Pair Async");

        let result = custom
            .PairAsync(
                DevicePairingKinds::ConfirmOnly
                    | DevicePairingKinds::DisplayPin
                    | DevicePairingKinds::ProvidePin,
            )
            .map_err(|e| format!("Error in PairAsync: {e}"))?
            .await
            .map_err(|e| format!("Error awaiting PairAsync: {e}"))?;

        println!("Post Pair Async");

        match result
            .Status()
            .map_err(|e| format!("Error checking Status: {e}"))?
        {
            DevicePairingResultStatus::Paired => {
                self.is_paired = true;
                self.can_pair = false;
                Ok(())
            }
            status => Err(format!("Pairing failed: {:?}", status)),
        }
    }

    pub async fn unregister(&mut self) -> Result<(), std::string::String> {
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

    pub fn connect(&mut self) -> Result<(), String> {
        todo!()
    }
    pub fn disconnect(&mut self) -> Result<(), String> {
        todo!()
    }

    pub fn read(&mut self) -> Result<Vec<u8>, String> {
        todo!()
    }
    pub fn write(&mut self, data: &[u8]) -> Result<(), std::string::String> {
        todo!()
    }
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn scan_winrt(app_handle: AppHandle) -> Result<Vec<WinRTDevice>, String> {
    let selector =
        BluetoothDevice::GetDeviceSelectorFromPairingState(false).map_err(|e| format!("{e}"))?;

    let devices_info = DeviceInformation::FindAllAsyncAqsFilter(&selector)
        .map_err(|e| format!("{e}"))?
        .await
        .map_err(|e| format!("{e}"))?;

    let size = devices_info.Size().map_err(|e| format!("{e}"))?;

    let mut devices = vec![];
    for i in 0..size {
        let device_info = devices_info.GetAt(i).map_err(|e| format!("{e}"))?;
        let device = WinRTDevice::from_info_and_app_handle(&device_info, app_handle.clone())
            .await
            .map_err(|e| format!("from_winrt Error: {e}"))?;
        devices.push(device);
    }

    Ok(devices)
}
