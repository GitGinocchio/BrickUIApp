use windows::Win32::Networking::WinSock::{WSAStartup, WSADATA};

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn inititalize_sockets() -> Result<(), String> {
    unsafe {
        let mut data = WSADATA::default();
        let version: u16 = (2u16 << 8) | 2u16;

        let r = WSAStartup(version, &mut data);
        if r != 0 {
            return Err(format!("WSAStartup failed: {r}"));
        }
    }

    Ok(())
}
