use crate::error::AppError;

const KEYRING_SERVICE: &str = "practice-valuation";

// Only Gemini is wired end-to-end in this slice (Fase 7.1/7.2); Claude and
// OpenAI join this list in Fase 7.6/7.7. Each provider gets its own keyring
// entry (same service, username = provider id) so adding one later never
// touches an already-stored key.
const SUPPORTED_PROVIDERS: [&str; 1] = ["gemini"];

fn require_supported_provider(provider: &str) -> Result<(), AppError> {
    if SUPPORTED_PROVIDERS.contains(&provider) {
        Ok(())
    } else {
        Err(AppError::UnknownProvider(provider.to_string()))
    }
}

#[tauri::command]
pub fn store_api_key(provider: String, key: String) -> Result<(), AppError> {
    require_supported_provider(&provider)?;
    let entry = keyring::Entry::new(KEYRING_SERVICE, &provider)?;
    entry.set_password(&key)?;
    Ok(())
}

// Never returns the key itself to the frontend — only whether one is stored.
#[tauri::command]
pub fn get_api_key_status(provider: String) -> Result<bool, AppError> {
    require_supported_provider(&provider)?;
    let entry = keyring::Entry::new(KEYRING_SERVICE, &provider)?;
    match entry.get_password() {
        Ok(_) => Ok(true),
        Err(keyring::Error::NoEntry) => Ok(false),
        Err(err) => Err(err.into()),
    }
}

#[tauri::command]
pub fn delete_api_key(provider: String) -> Result<(), AppError> {
    require_supported_provider(&provider)?;
    let entry = keyring::Entry::new(KEYRING_SERVICE, &provider)?;
    match entry.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
