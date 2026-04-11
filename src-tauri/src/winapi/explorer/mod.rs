pub mod recents;

use windows::Win32::System::Com::COINIT_APARTMENTTHREADED;
use windows::Win32::System::Com::COINIT_DISABLE_OLE1DDE;
use windows::Win32::System::Com::CoInitializeEx;
use windows::Win32::System::Com::CoTaskMemFree;
use windows::Win32::System::Com::CoUninitialize;
use windows::core::PCWSTR;
use windows::Win32::UI::Shell::SHOpenFolderAndSelectItems;
use windows::Win32::UI::Shell::SHParseDisplayName;
use std::ffi::c_void;
use std::path::Path;

#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn open_file_folder(path: &str) -> Result<(), String> {
    // 1. Validazione preventiva: il file esiste davvero?
    let os_path = Path::new(&path);
    if !os_path.exists() {
        return Err(format!("Il percorso '{}' non esiste sul disco.", path));
    }

    // 2. Conversione sicura in Wide String (UTF-16) per Windows
    // Usiamo un vettore per assicurarci che la memoria sia valida durante la chiamata
    let wide_path: Vec<u16> = path
        .encode_utf16()
        .chain(std::iter::once(0)) // Terminatore nullo fondamentale!
        .collect();

    unsafe {
        // 3. Inizializzazione COM forzata per questo thread
        // Usiamo un Result per gestire se COM è già attivo (S_FALSE) o fallisce
        let com_res = CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE);
        
        let mut pidl_item = std::ptr::null_mut();
        
        // 4. Trasformiamo il path in un PIDL
        let parse_res = SHParseDisplayName(
            PCWSTR(wide_path.as_ptr()),
            None,
            &mut pidl_item,
            0,
            None,
        );

        if parse_res.is_err() {
            if com_res.is_ok() { CoUninitialize(); }
            return Err(format!("Errore Shell (Parse): {:?}", parse_res));
        }

        let select_res = SHOpenFolderAndSelectItems(pidl_item, None, 0);

        if !pidl_item.is_null() {
            CoTaskMemFree(Some(pidl_item as *const c_void));
        }
        
        if com_res.is_ok() {
            CoUninitialize();
        }

        if select_res.is_err() {
            return Err(format!("Errore Shell (Open): {:?}", select_res));
        }
    }

    Ok(())
}