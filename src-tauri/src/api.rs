use crate::core::accounts::{create_account, delete_account, get_accounts};
use crate::core::auth::{handle_oauth_callback, start_oauth_flow, AppState, OAuth2StateMap};
use crate::core::config::{load_config, save_config};
use crate::core::export::export_account;
use crate::error::DEmailError;
use crate::models::{Account, OAuthProviderConfig};
use std::collections::HashMap;
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub async fn add_account(
    app_handle: AppHandle,
    email_address: String,
) -> Result<String, DEmailError> {
    start_oauth_flow(app_handle, email_address).await
}

#[tauri::command]
pub async fn handle_callback(
    app_handle: AppHandle,
    code: String,
    state: String,
) -> Result<Account, DEmailError> {
    handle_oauth_callback(app_handle, code, state).await
}

#[tauri::command]
pub fn get_accounts(app_handle: AppHandle) -> Result<Vec<Account>, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let conn = app_state.db_conn.lock().unwrap();
    get_accounts(&conn)
}

#[tauri::command]
pub fn delete_account(app_handle: AppHandle, account_id: i64) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let conn = app_state.db_conn.lock().unwrap();
    delete_account(&conn, account_id)
}

#[tauri::command]
pub fn save_oauth_provider_config(
    app_handle: AppHandle,
    provider: String,
    config: OAuthProviderConfig,
) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let app_config = app_state.app_config.lock().unwrap();
    let mut configs = load_config(&app_config)?;
    configs.insert(provider, config);
    save_config(&app_config, &configs)
}

#[tauri::command]
pub fn get_oauth_provider_configs(
    app_handle: AppHandle,
) -> Result<HashMap<String, OAuthProviderConfig>, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let app_config = app_state.app_config.lock().unwrap();
    load_config(&app_config)
}

#[tauri::command]
pub fn get_folders(
    app_handle: AppHandle,
    account_id: i64,
) -> Result<Vec<crate::models::Folder>, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let conn = app_state.db_conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, account_id, name, path, parent_id FROM folders WHERE account_id = ?1",
    )?;
    let folder_iter = stmt.query_map([&account_id], |row| {
        Ok(crate::models::Folder {
            id: row.get(0)?,
            account_id: row.get(1)?,
            name: row.get(2)?,
            path: row.get(3)?,
            parent_id: row.get(4)?,
        })
    })?;

    let mut folders = Vec::new();
    for folder in folder_iter {
        folders.push(folder?);
    }
    Ok(folders)
}

#[tauri::command]
pub fn get_messages(
    app_handle: AppHandle,
    folder_id: i64,
) -> Result<Vec<crate::models::MessageHeader>, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let conn = app_state.db_conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, subject, from_header, date, is_read, has_attachments FROM messages WHERE folder_id = ?1 ORDER BY date DESC",
    )?;
    let message_iter = stmt.query_map([&folder_id], |row| {
        Ok(crate::models::MessageHeader {
            id: row.get(0)?,
            subject: row.get(1)?,
            from: row.get(2)?,
            date: row.get(3)?,
            is_read: row.get(4)?,
            has_attachments: row.get(5)?,
        })
    })?;

    let mut messages = Vec::new();
    for message in message_iter {
        messages.push(message?);
    }
    Ok(messages)
}

#[tauri::command]
pub fn get_message_details(
    app_handle: AppHandle,
    message_id: i64,
) -> Result<crate::models::Message, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let conn = app_state.db_conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM messages WHERE id = ?1")?;
    let message = stmt.query_row([&message_id], |row| {
        Ok(crate::models::Message {
            id: row.get(0)?,
            account_id: row.get(1)?,
            folder_id: row.get(2)?,
            imap_uid: row.get(3)?,
            message_id_header: row.get(4)?,
            from_header: row.get(6)?,
            to_header: row.get(7)?,
            cc_header: row.get(8)?,
            subject: row.get(9)?,
            date: row.get(10)?,
            body_plain: row.get(11)?,
            body_html: row.get(12)?,
            has_attachments: row.get(13)?,
            is_read: row.get(14)?,
            attachments: Vec::new(), // Will be loaded separately
        })
    })?;
    Ok(message)
}

use crate::core::accounts::get_refresh_token;
use oauth2::{RefreshToken, TokenResponse};

#[tauri::command]
pub async fn send_email(
    app_handle: AppHandle,
    account_id: i64,
    to: String,
    subject: String,
    body: String,
) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let conn = app_state.db_conn.lock().unwrap();
    let mut stmt =
        conn.prepare("SELECT provider_type, email_address FROM accounts WHERE id = ?1")?;
    let (provider, email_address) = stmt.query_row([&account_id], |row| {
        let provider: String = row.get(0)?;
        let email: String = row.get(1)?;
        Ok((provider, email))
    })?;

    let message = lettre::Message::builder()
        .from(email_address.parse().map_err(|_| {
            DEmailError::Smtp(lettre::transport::smtp::Error::from("Invalid from address"))
        })?)
        .to(to.parse().map_err(|_| {
            DEmailError::Smtp(lettre::transport::smtp::Error::from("Invalid to address"))
        })?)
        .subject(subject)
        .body(body)
        .map_err(|_| DEmailError::Smtp(lettre::transport::smtp::Error::from("Invalid message")))?;

    let app_config = app_state.app_config.lock().unwrap();
    let configs = load_config(&app_config)?;
    let provider_config = configs
        .get(&provider)
        .ok_or_else(|| DEmailError::Config(format!("OAuth config for {} not found", provider)))?;

    let client = match provider.as_str() {
        "google" => crate::core::auth::get_google_client(
            provider_config.client_id.clone(),
            provider_config.client_secret.clone(),
        )?,
        "microsoft" => crate::core::auth::get_microsoft_client(
            provider_config.client_id.clone(),
            provider_config.client_secret.clone(),
        )?,
        _ => {
            return Err(DEmailError::Smtp(lettre::transport::smtp::Error::from(
                "Unsupported provider".to_string(),
            )))
        }
    };

    let refresh_token = get_refresh_token(account_id)?;
    let token_result = client
        .exchange_refresh_token(&RefreshToken::new(refresh_token))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(|e| DEmailError::OAuth(e.to_string()))?;

    let access_token = token_result.access_token();

    crate::core::sync::smtp_send::send_email(message, &provider, access_token).await
}

#[tauri::command]
pub fn start_export(
    app_handle: AppHandle,
    account_id: i64,
    destination_path: String,
) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let conn = app_state.db_conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM accounts WHERE id = ?1")?;
    let account = stmt.query_row([&account_id], |row| {
        Ok(Account {
            id: row.get(0)?,
            email_address: row.get(1)?,
            display_name: row.get(2)?,
            provider_type: row.get(3)?,
        })
    })?;

    export_account(&conn, &account, &destination_path)
}
