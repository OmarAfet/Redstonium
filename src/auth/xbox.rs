use serde_json::json;
use ureq::Agent;

use super::types::{
    AuthError, XboxResponse, XboxToken, XstsError, XstsErrorResponse, XstsToken,
};

const XBOX_LIVE_URL: &str = "https://user.auth.xboxlive.com/user/authenticate";
const XSTS_URL: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";

/// Exchange a Microsoft access token for an Xbox Live token.
pub fn authenticate(microsoft_access_token: &str) -> Result<XboxToken, AuthError> {
    let body = json!({
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            "RpsTicket": format!("d={microsoft_access_token}")
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT"
    });

    let mut resp = ureq::post(XBOX_LIVE_URL)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .send(body.to_string().as_str())
        .map_err(|e| AuthError::XboxAuth(e.to_string()))?;

    let xbox_resp: XboxResponse = resp
        .body_mut()
        .read_json()
        .map_err(|e| AuthError::XboxAuth(e.to_string()))?;

    Ok(xbox_resp.into())
}

/// Exchange an Xbox Live token for an XSTS token scoped to Minecraft services.
///
/// Uses an agent with `http_status_as_error(false)` so we can read the
/// response body on 401 errors (XSTS returns XErr codes in the body).
pub fn obtain_xsts(xbox_token: &str) -> Result<XstsToken, AuthError> {
    let body = json!({
        "Properties": {
            "SandboxId": "RETAIL",
            "UserTokens": [xbox_token]
        },
        "RelyingParty": "rp://api.minecraftservices.com/",
        "TokenType": "JWT"
    });

    let agent: Agent = Agent::config_builder()
        .http_status_as_error(false)
        .build()
        .into();

    let mut response = agent
        .post(XSTS_URL)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .send(body.to_string().as_str())
        .map_err(|e| AuthError::XboxAuth(e.to_string()))?;

    let status = response.status();

    if status == 200 {
        let xbox_resp: XboxResponse = response
            .body_mut()
            .read_json()
            .map_err(|e| AuthError::XboxAuth(e.to_string()))?;
        Ok(xbox_resp.into())
    } else if status == 401 {
        let body_str = response
            .body_mut()
            .read_to_string()
            .map_err(|e| AuthError::XboxAuth(e.to_string()))?;
        let xsts_err: XstsErrorResponse =
            serde_json::from_str(&body_str).map_err(|e| AuthError::XboxAuth(e.to_string()))?;
        Err(AuthError::XstsAuth(XstsError::from_code(xsts_err.xerr)))
    } else {
        let body_str = response
            .body_mut()
            .read_to_string()
            .unwrap_or_default();
        Err(AuthError::XboxAuth(format!(
            "Unexpected status {status}: {body_str}"
        )))
    }
}
