pub mod types;

mod microsoft;
mod minecraft;
mod storage;
mod xbox;

use std::sync::Arc;

use tiny_http::Server;

use types::{Account, AuthError, is_expired};

/// Start the local HTTP server on :8080 and open the browser to Microsoft's
/// login page. Returns the server handle so the caller can cancel by calling
/// `server.unblock()`.
///
/// This function is blocking (opens browser) and should be called from a
/// background thread.
pub fn start_login_server() -> Result<Arc<Server>, AuthError> {
    let server = microsoft::start_server()?;
    microsoft::open_browser()?;
    Ok(server)
}

/// Block until Microsoft redirects back with an authorization code.
/// Returns `Err(AuthError::NoAuthCode)` if cancelled via `server.unblock()`.
///
/// This function is blocking and should be called from a background thread.
pub fn wait_for_code(server: &Server) -> Result<String, AuthError> {
    microsoft::wait_for_code(server)
}

/// Exchange the authorization code through the full token chain
/// (Microsoft → Xbox → XSTS → Minecraft), check game ownership, fetch
/// the player profile, and save everything to disk.
///
/// This function is blocking and should be called from a background thread.
pub fn exchange_and_fetch(code: &str) -> Result<Account, AuthError> {
    let ms_token = microsoft::exchange_code(code)?;

    let xbox_token = xbox::authenticate(&ms_token.access_token)?;
    let xsts_token = xbox::obtain_xsts(&xbox_token.token)?;

    let mc_token = minecraft::login_with_xbox(&xsts_token.user_hash, &xsts_token.token)?;
    let entitlements = minecraft::check_ownership(&mc_token.access_token)?;
    let profile = minecraft::get_profile(&mc_token.access_token)?;

    let account = Account {
        microsoft_token: ms_token,
        xbox_token,
        xsts_token,
        minecraft_token: mc_token,
        entitlements,
        profile,
    };

    storage::save(&account)?;

    Ok(account)
}

/// Check if stored tokens are still valid. If the Microsoft token has expired,
/// refresh it silently using the refresh token and re-do the Xbox → XSTS →
/// Minecraft chain. Returns the updated account.
///
/// If no account is saved, returns `Ok(None)`.
/// If the refresh token itself is rejected, returns an error (caller should
/// trigger a fresh `login()`).
///
/// This function is blocking and should be called from a background thread.
pub fn ensure_valid_token() -> Result<Option<Account>, AuthError> {
    let account = match storage::load()? {
        Some(a) => a,
        None => return Ok(None),
    };

    // Microsoft token still valid — nothing to do.
    if !is_expired(account.microsoft_token.expires_at) {
        // Minecraft token might also still be valid
        if !is_expired(account.minecraft_token.expires_at) {
            return Ok(Some(account));
        }
    }

    // Microsoft token expired — refresh it.
    let ms_token = microsoft::refresh_token(&account.microsoft_token.refresh_token)?;

    // Re-do the Xbox → XSTS → Minecraft chain with the fresh access token.
    let xbox_token = xbox::authenticate(&ms_token.access_token)?;
    let xsts_token = xbox::obtain_xsts(&xbox_token.token)?;
    let mc_token = minecraft::login_with_xbox(&xsts_token.user_hash, &xsts_token.token)?;

    // Profile and entitlements don't change, keep the existing ones.
    let updated = Account {
        microsoft_token: ms_token,
        xbox_token,
        xsts_token,
        minecraft_token: mc_token,
        entitlements: account.entitlements,
        profile: account.profile,
    };

    storage::save(&updated)?;

    Ok(Some(updated))
}

/// Load the saved account from disk without refreshing anything.
/// Used on app startup to immediately show the username in the UI.
pub fn load_account() -> Result<Option<Account>, AuthError> {
    storage::load()
}

/// Delete the saved account (logout).
pub fn logout() -> Result<(), AuthError> {
    storage::clear()
}
