use std::fs;
use std::path::PathBuf;

use super::types::{Account, AuthError};

const APP_DIR_NAME: &str = "Redstonium";
const ACCOUNTS_FILE: &str = "accounts.json";

/// Return the path to `<data_dir>/Redstonium/accounts.json`.
///
/// | OS      | Path                                                  |
/// |---------|-------------------------------------------------------|
/// | Windows | `C:\Users\<user>\AppData\Roaming\Redstonium\`         |
/// | macOS   | `~/Library/Application Support/Redstonium/`           |
/// | Linux   | `~/.local/share/Redstonium/`                          |
fn accounts_path() -> Result<PathBuf, AuthError> {
    let data_dir = dirs::data_dir()
        .ok_or_else(|| AuthError::Storage("Could not determine data directory".into()))?;
    Ok(data_dir.join(APP_DIR_NAME).join(ACCOUNTS_FILE))
}

/// Load a previously saved account from disk, if one exists.
pub fn load() -> Result<Option<Account>, AuthError> {
    let path = accounts_path()?;

    if !path.exists() {
        return Ok(None);
    }

    let contents =
        fs::read_to_string(&path).map_err(|e| AuthError::Storage(e.to_string()))?;

    let account: Account =
        serde_json::from_str(&contents).map_err(|e| AuthError::Storage(e.to_string()))?;

    Ok(Some(account))
}

/// Save an account to disk, creating the directory if needed.
pub fn save(account: &Account) -> Result<(), AuthError> {
    let path = accounts_path()?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| AuthError::Storage(e.to_string()))?;
    }

    let json = serde_json::to_string_pretty(account)
        .map_err(|e| AuthError::Storage(e.to_string()))?;

    fs::write(&path, json).map_err(|e| AuthError::Storage(e.to_string()))?;

    Ok(())
}

/// Delete the saved account file (used on logout).
pub fn clear() -> Result<(), AuthError> {
    let path = accounts_path()?;

    if path.exists() {
        fs::remove_file(&path).map_err(|e| AuthError::Storage(e.to_string()))?;
    }

    Ok(())
}
