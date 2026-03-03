use std::sync::Arc;

use tiny_http::{Response, Server};

use super::types::{AuthError, MicrosoftToken, MicrosoftTokenResponse};

const CLIENT_ID: &str = "74ab16e9-5151-4478-8184-e590ba53d01d";
const REDIRECT_URI: &str = "http://localhost:8080";
const AUTH_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";
const TOKEN_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";
const SCOPES: &str = "XboxLive.signin offline_access";

/// Build the Microsoft OAuth2 authorize URL that the browser will open.
fn build_authorize_url() -> String {
    format!(
        "{AUTH_URL}?client_id={CLIENT_ID}\
         &response_type=code\
         &redirect_uri={redirect}\
         &scope={scopes}\
         &response_mode=query\
         &prompt=select_account",
        redirect = urlencoding::encode(REDIRECT_URI),
        scopes = urlencoding::encode(SCOPES),
    )
}

/// Extract the `code` query parameter from a request path like `/?code=XYZ&state=...`.
fn extract_code_from_path(path: &str) -> Option<String> {
    let query = path.split_once('?')?.1;
    for pair in query.split('&') {
        if let Some(("code", value)) = pair.split_once('=') {
            return Some(value.to_string());
        }
    }
    None
}

/// Start the local HTTP server on :8080 and return it wrapped in an `Arc`.
/// The caller keeps a clone of the `Arc` so it can call `server.unblock()`
/// to cancel the login from the UI thread.
pub fn start_server() -> Result<Arc<Server>, AuthError> {
    let server =
        Server::http("127.0.0.1:8080").map_err(|e| AuthError::ServerStart(e.to_string()))?;
    Ok(Arc::new(server))
}

/// Open the browser to Microsoft's login page.
pub fn open_browser() -> Result<(), AuthError> {
    let url = build_authorize_url();
    open::that(&url).map_err(|e| AuthError::BrowserOpen(e.to_string()))
}

/// Block until Microsoft redirects back to :8080 with an authorization code.
/// Returns `Err(AuthError::NoAuthCode)` if the server was unblocked (cancelled)
/// or if the redirect didn't contain a code.
pub fn wait_for_code(server: &Server) -> Result<String, AuthError> {
    let request = server.recv().map_err(|_| AuthError::NoAuthCode)?;

    let code = extract_code_from_path(request.url());

    let html = if code.is_some() {
        "<html><body><h2>Login successful!</h2><p>You can close this tab.</p></body></html>"
    } else {
        "<html><body><h2>Login failed.</h2><p>No authorization code received.</p></body></html>"
    };

    let response = Response::from_string(html).with_header(
        tiny_http::Header::from_bytes("Content-Type", "text/html").unwrap(),
    );
    let _ = request.respond(response);

    code.ok_or(AuthError::NoAuthCode)
}

/// Post form data to the Microsoft token endpoint and parse the response.
fn post_token_request(body: &str) -> Result<MicrosoftToken, AuthError> {
    let mut resp = ureq::post(TOKEN_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send(body)
        .map_err(|e| AuthError::TokenExchange(e.to_string()))?;

    let token_resp: MicrosoftTokenResponse = resp
        .body_mut()
        .read_json()
        .map_err(|e| AuthError::TokenExchange(e.to_string()))?;

    Ok(token_resp.into())
}

/// Exchange an authorization code for a Microsoft access + refresh token.
pub fn exchange_code(code: &str) -> Result<MicrosoftToken, AuthError> {
    let body = format!(
        "client_id={CLIENT_ID}\
         &scope={scopes}\
         &code={code}\
         &redirect_uri={redirect}\
         &grant_type=authorization_code",
        scopes = urlencoding::encode(SCOPES),
        code = urlencoding::encode(code),
        redirect = urlencoding::encode(REDIRECT_URI),
    );

    post_token_request(&body)
}

/// Use a stored refresh token to obtain a fresh Microsoft token pair.
pub fn refresh_token(refresh_token: &str) -> Result<MicrosoftToken, AuthError> {
    let body = format!(
        "client_id={CLIENT_ID}\
         &scope={scopes}\
         &refresh_token={rt}\
         &grant_type=refresh_token",
        scopes = urlencoding::encode(SCOPES),
        rt = urlencoding::encode(refresh_token),
    );

    post_token_request(&body)
}
