use serde_json::json;
use ureq::Agent;

use super::types::{
    AuthError, EntitlementResponse, MinecraftToken, PlayerProfile, expires_at_from_now,
};

const LOGIN_URL: &str = "https://api.minecraftservices.com/authentication/login_with_xbox";
const ENTITLEMENTS_URL: &str = "https://api.minecraftservices.com/entitlements/mcstore";
const PROFILE_URL: &str = "https://api.minecraftservices.com/minecraft/profile";

/// Exchange an XSTS token for a Minecraft access token.
pub fn login_with_xbox(user_hash: &str, xsts_token: &str) -> Result<MinecraftToken, AuthError> {
    let identity_token = format!("XBL3.0 x={user_hash};{xsts_token}");
    let body = json!({ "identityToken": identity_token });

    let mut resp = ureq::post(LOGIN_URL)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .send(body.to_string().as_str())
        .map_err(|e| AuthError::MinecraftAuth(e.to_string()))?;

    let mut mc_token: MinecraftToken = resp
        .body_mut()
        .read_json()
        .map_err(|e| AuthError::MinecraftAuth(e.to_string()))?;

    mc_token.expires_at = expires_at_from_now(mc_token.expires_in);
    Ok(mc_token)
}

/// Check whether the account owns Minecraft.
///
/// Returns the full entitlement response. The caller should check that
/// `items` contains an entry with `name == "game_minecraft"`.
pub fn check_ownership(minecraft_access_token: &str) -> Result<EntitlementResponse, AuthError> {
    let mut resp = ureq::get(ENTITLEMENTS_URL)
        .header("Authorization", &format!("Bearer {minecraft_access_token}"))
        .call()
        .map_err(|e| AuthError::MinecraftAuth(e.to_string()))?;

    let entitlements: EntitlementResponse = resp
        .body_mut()
        .read_json()
        .map_err(|e| AuthError::MinecraftAuth(e.to_string()))?;

    let owns_game = entitlements
        .items
        .iter()
        .any(|item| item.name == "game_minecraft");

    if !owns_game {
        return Err(AuthError::NoGameOwnership);
    }

    Ok(entitlements)
}

/// Fetch the player's Minecraft profile (username, UUID, skins, capes).
///
/// Uses an agent with `http_status_as_error(false)` to distinguish
/// "no profile" (NOT_FOUND) from network errors.
pub fn get_profile(minecraft_access_token: &str) -> Result<PlayerProfile, AuthError> {
    let agent: Agent = Agent::config_builder()
        .http_status_as_error(false)
        .build()
        .into();

    let mut response = agent
        .get(PROFILE_URL)
        .header("Authorization", &format!("Bearer {minecraft_access_token}"))
        .call()
        .map_err(|e| AuthError::ProfileFetch(e.to_string()))?;

    let status = response.status();

    if status == 200 {
        let profile: PlayerProfile = response
            .body_mut()
            .read_json()
            .map_err(|e| AuthError::ProfileFetch(e.to_string()))?;
        Ok(profile)
    } else if status == 404 {
        Err(AuthError::ProfileNotFound)
    } else {
        let body_str = response
            .body_mut()
            .read_to_string()
            .unwrap_or_default();
        Err(AuthError::ProfileFetch(format!(
            "Unexpected status {status}: {body_str}"
        )))
    }
}
