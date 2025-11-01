use crate::core::accounts::{create_account, store_refresh_token};
use crate::core::config::load_config;
use crate::error::DEmailError;
use crate::models::Account;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use rusqlite::Connection;
use std::collections::HashMap;
use tauri::{AppHandle, Config, Manager, State};

pub struct AppState {
    pub db_conn: std::sync::Mutex<Connection>,
    pub app_config: std::sync::Mutex<Config>,
}

#[derive(Debug)]
pub struct OAuth2State {
    pub client: BasicClient,
    pub pkce_verifier: PkceCodeVerifier,
    pub csrf_token: CsrfToken,
    pub provider: String,
}

pub type OAuth2StateMap = std::sync::Mutex<HashMap<String, OAuth2State>>;

pub fn get_google_client(
    client_id: String,
    client_secret: String,
) -> Result<BasicClient, DEmailError> {
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .map_err(|e| DEmailError::OAuth(e.to_string()))?;
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
        .map_err(|e| DEmailError::OAuth(e.to_string()))?;

    Ok(BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(
        RedirectUrl::new("http://localhost:1420/callback".to_string())
            .map_err(|e| DEmailError::OAuth(e.to_string()))?,
    ))
}

pub fn get_microsoft_client(
    client_id: String,
    client_secret: String,
) -> Result<BasicClient, DEmailError> {
    let auth_url =
        AuthUrl::new("https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string())
            .map_err(|e| DEmailError::OAuth(e.to_string()))?;
    let token_url =
        TokenUrl::new("https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string())
            .map_err(|e| DEmailError::OAuth(e.to_string()))?;

    Ok(BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(
        RedirectUrl::new("http://localhost:1420/callback".to_string())
            .map_err(|e| DEmailError::OAuth(e.to_string()))?,
    ))
}

pub async fn start_oauth_flow(
    app_handle: AppHandle,
    email_address: String,
) -> Result<String, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let config = app_state.app_config.lock().unwrap();
    let configs = load_config(&config)?;

    let provider = if email_address.ends_with("@gmail.com") {
        "google".to_string()
    } else if email_address.ends_with("@outlook.com") || email_address.ends_with("@hotmail.com") {
        "microsoft".to_string()
    } else {
        return Err(DEmailError::OAuth("Unsupported provider".to_string()));
    };

    let provider_config = configs
        .get(&provider)
        .ok_or_else(|| DEmailError::Config(format!("OAuth config for {} not found", provider)))?;

    let client = match provider.as_str() {
        "google" => get_google_client(
            provider_config.client_id.clone(),
            provider_config.client_secret.clone(),
        )?,
        "microsoft" => get_microsoft_client(
            provider_config.client_id.clone(),
            provider_config.client_secret.clone(),
        )?,
        _ => return Err(DEmailError::OAuth("Unsupported provider".to_string())),
    };

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("https://mail.google.com/".to_string())) // for google
        .add_scope(Scope::new("offline_access".to_string()))
        .add_scope(Scope::new(
            "https://outlook.office.com/IMAP.AccessAsUser.All".to_string(),
        ))
        .add_scope(Scope::new(
            "https://outlook.office.com/SMTP.Send".to_string(),
        ))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let oauth_state_map = app_handle.state::<OAuth2StateMap>();
    let mut state_map = oauth_state_map.lock().unwrap();
    state_map.insert(
        csrf_token.secret().clone(),
        OAuth2State {
            client,
            pkce_verifier,
            csrf_token: csrf_token.clone(),
            provider,
        },
    );

    Ok(auth_url.to_string())
}

pub async fn handle_oauth_callback(
    app_handle: AppHandle,
    code: String,
    state: String,
) -> Result<Account, DEmailError> {
    let oauth_state_map = app_handle.state::<OAuth2StateMap>();
    let mut state_map = oauth_state_map.lock().unwrap();
    let oauth_state = state_map
        .remove(&state)
        .ok_or_else(|| DEmailError::Auth("Invalid OAuth state".to_string()))?;

    let token_result = oauth_state
        .client
        .exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(oauth_state.pkce_verifier)
        .request_async(async_http_client)
        .await
        .map_err(|e| DEmailError::OAuth(e.to_string()))?;

    let refresh_token = token_result
        .refresh_token()
        .ok_or_else(|| DEmailError::OAuth("No refresh token found".to_string()))?;

    let access_token = token_result.access_token();

    let (email_address, display_name) = match oauth_state.provider.as_str() {
        "google" => {
            let client = reqwest::Client::new();
            let response: serde_json::Value = client
                .get("https://www.googleapis.com/oauth2/v2/userinfo")
                .bearer_auth(access_token.secret())
                .send()
                .await
                .map_err(|e| DEmailError::OAuth(e.to_string()))?
                .json()
                .await
                .map_err(|e| DEmailError::OAuth(e.to_string()))?;

            let email = response["email"]
                .as_str()
                .ok_or_else(|| DEmailError::OAuth("Email not found in userinfo".to_string()))?
                .to_string();
            let name = response["name"].as_str().unwrap_or(&email).to_string();
            (email, name)
        }
        "microsoft" => {
            let client = reqwest::Client::new();
            let response: serde_json::Value = client
                .get("https://graph.microsoft.com/v1.0/me")
                .bearer_auth(access_token.secret())
                .send()
                .await
                .map_err(|e| DEmailError::OAuth(e.to_string()))?
                .json()
                .await
                .map_err(|e| DEmailError::OAuth(e.to_string()))?;

            let email = response["userPrincipalName"]
                .as_str()
                .ok_or_else(|| DEmailError::OAuth("Email not found in userinfo".to_string()))?
                .to_string();
            let name = response["displayName"]
                .as_str()
                .unwrap_or(&email)
                .to_string();
            (email, name)
        }
        _ => return Err(DEmailError::OAuth("Unsupported provider".to_string())),
    };

    let app_state = app_handle.state::<AppState>();
    let conn = app_state.db_conn.lock().unwrap();

    let account = create_account(&conn, &email_address, &display_name, &oauth_state.provider)?;

    store_refresh_token(account.id, refresh_token.secret())?;

    Ok(account)
}
