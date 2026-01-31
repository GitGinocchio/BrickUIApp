use keyring::Entry;

const SERVICE: &str = "brickui";
const REFRESH_TOKEN_KEY: &str = "refresh_token";

/// Salva il refresh token come stringa nel keyring
pub fn save_refresh_token(refresh_token: &str) -> Result<(), String> {
    let entry = Entry::new(SERVICE, REFRESH_TOKEN_KEY)
        .map_err(|e| format!("Error creating Entry: {e}"))?;
    
    entry.set_password(refresh_token)
        .map_err(|e| format!("Error setting Secret into Entry: {e}"))?;

    println!("Saving refresh token: {refresh_token:?}");

    Ok(())
}

/// Carica il refresh token dal keyring come stringa
pub fn load_refresh_token() -> Result<Option<String>, String> {
    let entry = Entry::new(SERVICE, REFRESH_TOKEN_KEY)
        .map_err(|e| format!("Error creating Entry: {e}"))?;
    
    match entry.get_password() {
        Ok(secret) => {
            println!("Loading refresh token: {secret:?}");

            Ok(Some(secret))
        },
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(format!("Error obtaining Secret: {e}")),
    }
}

/// Cancella il refresh token dal keyring
pub fn clear_refresh_token() -> Result<(), String> {
    let entry = Entry::new(SERVICE, REFRESH_TOKEN_KEY)
        .map_err(|e| format!("Error creating Entry: {e}"))?;

    println!("Clearing saved refresh token");
    
    match entry.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Error clearing Secret: {e}")),
    }
}
