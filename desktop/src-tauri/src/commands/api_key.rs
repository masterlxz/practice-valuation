use crate::domain::chat_provider::Provider;
use crate::error::AppError;

pub(crate) const KEYRING_SERVICE: &str = "practice-valuation";

// Every provider `Provider::parse` accepts (Gemini/Claude/OpenAI, all with
// real HTTP clients as of Fase 7.6/7.7) has its own keyring entry (same
// service, username = provider id), so storing one never touches another.
#[tauri::command]
pub fn store_api_key(provider: String, key: String) -> Result<(), AppError> {
    let provider = Provider::parse(&provider)?;
    let entry = keyring::Entry::new(KEYRING_SERVICE, provider.as_str())?;
    entry.set_password(&key)?;
    Ok(())
}

// Never returns the key itself to the frontend — only whether one is stored.
#[tauri::command]
pub fn get_api_key_status(provider: String) -> Result<bool, AppError> {
    let provider = Provider::parse(&provider)?;
    let entry = keyring::Entry::new(KEYRING_SERVICE, provider.as_str())?;
    match entry.get_password() {
        Ok(_) => Ok(true),
        Err(keyring::Error::NoEntry) => Ok(false),
        Err(err) => Err(err.into()),
    }
}

#[tauri::command]
pub fn delete_api_key(provider: String) -> Result<(), AppError> {
    let provider = Provider::parse(&provider)?;
    let entry = keyring::Entry::new(KEYRING_SERVICE, provider.as_str())?;
    match entry.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
