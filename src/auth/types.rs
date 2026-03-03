use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------------------------------------------------------------
// Utility
// ---------------------------------------------------------------------------

/// Compute a unix-epoch timestamp `expires_in` seconds from now.
pub fn expires_at_from_now(expires_in: u64) -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + expires_in
}

/// Check whether a unix-epoch timestamp is in the past.
pub fn is_expired(expires_at: u64) -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    now >= expires_at
}

// ---------------------------------------------------------------------------
// Microsoft OAuth2 Token
// ---------------------------------------------------------------------------

/// Raw JSON returned by `POST /consumers/oauth2/v2.0/token`.
#[derive(Debug, Deserialize)]
pub struct MicrosoftTokenResponse {
    pub token_type: String,
    pub scope: String,
    pub expires_in: u64,
    pub ext_expires_in: u64,
    pub access_token: String,
    pub refresh_token: String,
}

/// Persisted form — includes the computed `expires_at` timestamp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrosoftToken {
    pub token_type: String,
    pub scope: String,
    pub expires_in: u64,
    pub ext_expires_in: u64,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: u64,
}

impl From<MicrosoftTokenResponse> for MicrosoftToken {
    fn from(r: MicrosoftTokenResponse) -> Self {
        Self {
            expires_at: expires_at_from_now(r.expires_in),
            token_type: r.token_type,
            scope: r.scope,
            expires_in: r.expires_in,
            ext_expires_in: r.ext_expires_in,
            access_token: r.access_token,
            refresh_token: r.refresh_token,
        }
    }
}

// ---------------------------------------------------------------------------
// Xbox Live / XSTS
// ---------------------------------------------------------------------------

/// Raw JSON returned by Xbox Live and XSTS endpoints.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxResponse {
    pub issue_instant: String,
    pub not_after: String,
    pub token: String,
    pub display_claims: DisplayClaims,
}

#[derive(Debug, Deserialize)]
pub struct DisplayClaims {
    pub xui: Vec<XuiClaim>,
}

#[derive(Debug, Deserialize)]
pub struct XuiClaim {
    pub uhs: String,
}

/// Persisted form for Xbox Live token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XboxToken {
    pub issue_instant: String,
    pub not_after: String,
    pub token: String,
    pub user_hash: String,
}

impl From<XboxResponse> for XboxToken {
    fn from(r: XboxResponse) -> Self {
        Self {
            issue_instant: r.issue_instant,
            not_after: r.not_after,
            token: r.token,
            user_hash: r.display_claims.xui[0].uhs.clone(),
        }
    }
}

/// Persisted form for XSTS token (same shape, separate type for clarity).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XstsToken {
    pub issue_instant: String,
    pub not_after: String,
    pub token: String,
    pub user_hash: String,
}

impl From<XboxResponse> for XstsToken {
    fn from(r: XboxResponse) -> Self {
        Self {
            issue_instant: r.issue_instant,
            not_after: r.not_after,
            token: r.token,
            user_hash: r.display_claims.xui[0].uhs.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Minecraft
// ---------------------------------------------------------------------------

/// Raw JSON returned by `POST /authentication/login_with_xbox`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftToken {
    pub username: String,
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub roles: Vec<String>,
    pub metadata: serde_json::Value,
    #[serde(default)]
    pub expires_at: u64,
}

/// A single skin entry from the player profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skin {
    pub id: String,
    pub state: String,
    pub url: String,
    #[serde(default)]
    pub texture_key: String,
    pub variant: String,
}

/// A single cape entry from the player profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cape {
    pub id: String,
    pub state: String,
    pub url: String,
    #[serde(default)]
    pub alias: String,
}

/// Raw JSON returned by `GET /minecraft/profile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProfile {
    pub id: String,
    pub name: String,
    pub skins: Vec<Skin>,
    pub capes: Vec<Cape>,
    #[serde(default)]
    pub profile_actions: serde_json::Value,
}

/// A single entitlement item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entitlement {
    pub name: String,
    pub signature: String,
}

/// Raw JSON returned by `GET /entitlements/mcstore`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitlementResponse {
    pub items: Vec<Entitlement>,
    pub signature: String,
    pub key_id: String,
}

// ---------------------------------------------------------------------------
// Persisted account (everything saved to accounts.json)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub microsoft_token: MicrosoftToken,
    pub xbox_token: XboxToken,
    pub xsts_token: XstsToken,
    pub minecraft_token: MinecraftToken,
    pub entitlements: EntitlementResponse,
    pub profile: PlayerProfile,
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum XstsError {
    AccountBanned,
    NoXboxAccount,
    CountryUnavailable,
    AdultVerificationRequired,
    ChildAccount,
    Unknown(u64),
}

impl XstsError {
    pub fn from_code(code: u64) -> Self {
        match code {
            2148916227 => Self::AccountBanned,
            2148916233 => Self::NoXboxAccount,
            2148916235 => Self::CountryUnavailable,
            2148916236 | 2148916237 => Self::AdultVerificationRequired,
            2148916238 => Self::ChildAccount,
            other => Self::Unknown(other),
        }
    }
}

impl std::fmt::Display for XstsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AccountBanned => write!(f, "This Xbox account is banned"),
            Self::NoXboxAccount => write!(f, "This Microsoft account has no Xbox account"),
            Self::CountryUnavailable => {
                write!(f, "Xbox Live is not available in this account's country")
            }
            Self::AdultVerificationRequired => {
                write!(f, "This account requires adult verification (South Korea)")
            }
            Self::ChildAccount => write!(
                f,
                "This is a child account and must be added to a Family by an adult"
            ),
            Self::Unknown(code) => write!(f, "Unknown XSTS error (code {code})"),
        }
    }
}

/// Raw JSON returned on XSTS failure (HTTP 401).
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct XstsErrorResponse {
    #[serde(rename = "Identity")]
    pub identity: String,
    #[serde(rename = "XErr")]
    pub xerr: u64,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Redirect")]
    pub redirect: String,
}

#[derive(Debug)]
pub enum AuthError {
    ServerStart(String),
    BrowserOpen(String),
    NoAuthCode,
    TokenExchange(String),
    XboxAuth(String),
    XstsAuth(XstsError),
    MinecraftAuth(String),
    NoGameOwnership,
    ProfileNotFound,
    ProfileFetch(String),
    Storage(String),
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ServerStart(e) => write!(f, "Failed to start local auth server: {e}"),
            Self::BrowserOpen(e) => write!(f, "Failed to open browser: {e}"),
            Self::NoAuthCode => write!(f, "No authorization code received"),
            Self::TokenExchange(e) => write!(f, "Microsoft token exchange failed: {e}"),
            Self::XboxAuth(e) => write!(f, "Xbox Live authentication failed: {e}"),
            Self::XstsAuth(e) => write!(f, "XSTS authentication failed: {e}"),
            Self::MinecraftAuth(e) => write!(f, "Minecraft authentication failed: {e}"),
            Self::NoGameOwnership => write!(f, "This account does not own Minecraft"),
            Self::ProfileNotFound => write!(f, "No Minecraft profile found for this account"),
            Self::ProfileFetch(e) => write!(f, "Failed to fetch Minecraft profile: {e}"),
            Self::Storage(e) => write!(f, "Storage error: {e}"),
        }
    }
}

impl std::error::Error for AuthError {}
