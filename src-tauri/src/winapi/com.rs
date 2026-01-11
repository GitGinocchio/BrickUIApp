use std::cell::Cell;
use windows::Win32::Foundation::RPC_E_CHANGED_MODE;
use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize};

// Thread-local flag per sapere se abbiamo inizializzato COM
thread_local! {
    static COM_INITIALIZED: Cell<bool> = Cell::new(false);
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn initialize_com() -> Result<bool, String> {
    unsafe {
        match CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok() {
            Ok(_) => {
                Ok(true)
            }
            Err(e) => {
                if e.code() == RPC_E_CHANGED_MODE {
                    // RPC_E_CHANGED_MODE → COM già inizializzato con modalità diversa
                    Ok(false)
                } else {
                    Err(format!("Error initializing com: {e}"))
                }
            }
        }
    }
    /*
    COM_INITIALIZED.with(|flag| {
        if flag.get() {
            Ok(false)
        } else {
            unsafe {
                match CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok() {
                    Ok(_) => {
                        flag.set(true);
                        Ok(true)
                    }
                    Err(e) => {
                        if e.code() == RPC_E_CHANGED_MODE {
                            // RPC_E_CHANGED_MODE → COM già inizializzato con modalità diversa
                            Ok(false)
                        } else {
                            Err(format!("Error initializing com: {e}"))
                        }
                    }
                }
            }
        }
    })
    */
}

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn uninitialize_com() {
    unsafe { CoUninitialize() };
    /*
    COM_INITIALIZED.with(|flag| {
        if flag.get() {
            unsafe { CoUninitialize() };
            flag.set(false);
        }
    });
    */
}
