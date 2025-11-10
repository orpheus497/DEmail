use crate::core::accounts::{create_account, delete_account, get_accounts};
use crate::core::auth::{handle_oauth_callback, start_oauth_flow, AppState, OAuth2StateMap};
use crate::core::config::{load_config, save_config};
use crate::core::export::export_account;
use crate::core::validation;
use crate::error::DEmailError;
use crate::models::{Account, OAuthProviderConfig};
use std::collections::HashMap;
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub async fn add_account(
    app_handle: AppHandle,
    email_address: String,
) -> Result<String, DEmailError> {
    validation::validate_email(&email_address)?;
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
    let pool = &app_state.db_pool;
    get_accounts(pool)
}

#[tauri::command]
pub fn delete_account(app_handle: AppHandle, account_id: i64) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    delete_account(pool, account_id)
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
    let pool = &app_state.db_pool;
    let conn = pool
        .get()
        .map_err(|_| DEmailError::Database(rusqlite::Error::InvalidQuery))?;

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
    let pool = &app_state.db_pool;
    let conn = pool
        .get()
        .map_err(|_| DEmailError::Database(rusqlite::Error::InvalidQuery))?;

    let mut stmt = conn.prepare(
        "SELECT id, subject, from_header, date, is_read, has_attachments, is_starred FROM messages WHERE folder_id = ?1 ORDER BY date DESC",
    )?;
    let message_iter = stmt.query_map([&folder_id], |row| {
        Ok(crate::models::MessageHeader {
            id: row.get(0)?,
            subject: row.get(1)?,
            from: row.get(2)?,
            date: row.get(3)?,
            is_read: row.get(4)?,
            has_attachments: row.get(5)?,
            is_starred: row.get(6)?,
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
    let pool = &app_state.db_pool;
    let conn = pool
        .get()
        .map_err(|_| DEmailError::Database(rusqlite::Error::InvalidQuery))?;

    let mut stmt = conn.prepare(
        "SELECT id, account_id, folder_id, imap_uid, message_id_header, in_reply_to_header,
         from_header, to_header, cc_header, subject, date, body_plain, body_html,
         has_attachments, is_read, is_starred, thread_id
         FROM messages WHERE id = ?1",
    )?;
    let mut message = stmt.query_row([&message_id], |row| {
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
            is_starred: row.get(15)?,
            thread_id: row.get(16)?,
            attachments: Vec::new(),
        })
    })?;

    message.attachments = crate::core::cache::db::get_attachments_for_message(pool, message_id)?;

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
    // Validate inputs
    validation::validate_email(&to)?;
    validation::validate_subject(&subject)?;
    validation::validate_body(&body)?;

    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    let conn = pool
        .get()
        .map_err(|_| DEmailError::Database(rusqlite::Error::InvalidQuery))?;

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

    crate::core::sync::smtp_send::send_email(message, &provider, &email_address, access_token).await
}

#[tauri::command]
pub fn start_export(
    app_handle: AppHandle,
    account_id: i64,
    destination_path: String,
) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    let conn = pool
        .get()
        .map_err(|_| DEmailError::Database(rusqlite::Error::InvalidQuery))?;

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

#[tauri::command]
pub fn mark_message_read(app_handle: AppHandle, message_id: i64) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::update_message_read_status(pool, message_id, true)
}

#[tauri::command]
pub fn mark_message_unread(app_handle: AppHandle, message_id: i64) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::update_message_read_status(pool, message_id, false)
}

#[tauri::command]
pub async fn refresh_account(app_handle: AppHandle, account_id: i64) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    let conn = pool
        .get()
        .map_err(|_| DEmailError::Database(rusqlite::Error::InvalidQuery))?;

    let mut stmt =
        conn.prepare("SELECT provider_type, email_address FROM accounts WHERE id = ?1")?;
    let (provider, email_address) = stmt.query_row([&account_id], |row| {
        let provider: String = row.get(0)?;
        let email: String = row.get(1)?;
        Ok((provider, email))
    })?;
    drop(stmt);
    drop(conn);

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
            return Err(DEmailError::Api(format!(
                "Unsupported provider: {}",
                provider
            )))
        }
    };
    drop(app_config);

    let app_state_arc = std::sync::Arc::new(AppState {
        db_pool: app_state.db_pool.clone(),
        app_config: app_state.app_config.clone(),
    });

    crate::core::sync::imap_sync::sync_account_manually(
        app_state_arc,
        account_id,
        &provider,
        &client,
        &email_address,
    )
    .await
}

#[tauri::command]
pub fn search_messages(
    app_handle: AppHandle,
    account_id: i64,
    query: String,
) -> Result<Vec<crate::models::MessageHeader>, DEmailError> {
    validation::validate_search_query(&query)?;

    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::search_messages_fts(pool, account_id, &query)
}

#[tauri::command]
pub fn save_draft(app_handle: AppHandle, draft: crate::models::Draft) -> Result<i64, DEmailError> {
    // Validate draft fields
    validation::validate_subject(&draft.subject)?;
    if let Some(ref body) = draft.body_plain {
        validation::validate_body(body)?;
    }
    if let Some(ref body) = draft.body_html {
        validation::validate_body(body)?;
    }

    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::drafts::create_or_update_draft(pool, &draft)
}

#[tauri::command]
pub fn get_drafts(
    app_handle: AppHandle,
    account_id: i64,
) -> Result<Vec<crate::models::Draft>, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::drafts::load_drafts_for_account(pool, account_id)
}

#[tauri::command]
pub fn delete_draft(app_handle: AppHandle, draft_id: i64) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::drafts::remove_draft(pool, draft_id)
}

#[tauri::command]
pub fn save_signature(
    app_handle: AppHandle,
    signature: crate::models::EmailSignature,
) -> Result<i64, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::save_signature(pool, &signature)
}

#[tauri::command]
pub fn get_signatures(
    app_handle: AppHandle,
    account_id: i64,
) -> Result<Vec<crate::models::EmailSignature>, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::get_signatures(pool, account_id)
}

#[tauri::command]
pub fn delete_signature(app_handle: AppHandle, signature_id: i64) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::delete_signature(pool, signature_id)
}

#[tauri::command]
pub fn download_attachment(
    app_handle: AppHandle,
    attachment_id: i64,
    destination_path: String,
) -> Result<(), DEmailError> {
    // Validate destination path
    let dest_path = std::path::Path::new(&destination_path);
    validation::sanitize_filename(
        dest_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| DEmailError::Validation("Invalid destination path".to_string()))?,
    )?;

    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    let conn = pool
        .get()
        .map_err(|_| DEmailError::Database(rusqlite::Error::InvalidQuery))?;

    let mut stmt = conn.prepare("SELECT * FROM attachments WHERE id = ?1")?;
    let attachment = stmt.query_row([attachment_id], |row| {
        Ok(crate::models::Attachment {
            id: row.get(0)?,
            message_id: row.get(1)?,
            filename: row.get(2)?,
            mime_type: row.get(3)?,
            size_bytes: row.get(4)?,
            local_path: row.get(5)?,
        })
    })?;

    crate::core::attachments::save_attachment_to_disk(&conn, &attachment, &destination_path)
}

#[tauri::command]
pub fn get_messages_paginated(
    app_handle: AppHandle,
    folder_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<crate::models::MessageHeader>, DEmailError> {
    validation::validate_pagination(limit, offset)?;

    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::get_messages_paginated(pool, folder_id, limit, offset)
}

#[tauri::command]
pub fn count_messages_in_folder(app_handle: AppHandle, folder_id: i64) -> Result<i64, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::count_messages_in_folder(pool, folder_id)
}

#[tauri::command]
pub fn delete_message(app_handle: AppHandle, message_id: i64) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::delete_message(pool, message_id)
}

#[tauri::command]
pub fn move_message(
    app_handle: AppHandle,
    message_id: i64,
    target_folder_id: i64,
) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::move_message(pool, message_id, target_folder_id)
}

#[tauri::command]
pub fn save_setting(app_handle: AppHandle, key: String, value: String) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::save_setting(pool, &key, &value)
}

#[tauri::command]
pub fn get_setting(app_handle: AppHandle, key: String) -> Result<Option<String>, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::get_setting(pool, &key)
}

#[tauri::command]
pub fn get_all_settings(
    app_handle: AppHandle,
) -> Result<Vec<crate::models::AppSetting>, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::get_all_settings(pool)
}

#[tauri::command]
pub fn prepare_reply(
    app_handle: AppHandle,
    message_id: i64,
    reply_all: bool,
) -> Result<crate::core::reply_forward::ReplyData, DEmailError> {
    let message = get_message_details(app_handle, message_id)?;
    crate::core::reply_forward::prepare_reply(&message, reply_all)
}

#[tauri::command]
pub fn prepare_forward(
    app_handle: AppHandle,
    message_id: i64,
) -> Result<crate::core::reply_forward::ForwardData, DEmailError> {
    let message = get_message_details(app_handle, message_id)?;
    crate::core::reply_forward::prepare_forward(&message)
}

// ============================================================================
// STARRING OPERATIONS
// ============================================================================

#[tauri::command]
pub fn star_message(app_handle: AppHandle, message_id: i64) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::star_message(pool, message_id)
}

#[tauri::command]
pub fn unstar_message(app_handle: AppHandle, message_id: i64) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::unstar_message(pool, message_id)
}

#[tauri::command]
pub fn get_starred_messages(
    app_handle: AppHandle,
    account_id: i64,
) -> Result<Vec<crate::models::MessageHeader>, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::get_starred_messages(pool, account_id)
}

// ============================================================================
// BULK OPERATIONS
// ============================================================================

#[tauri::command]
pub fn bulk_mark_read(app_handle: AppHandle, message_ids: Vec<i64>) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::bulk_mark_read(pool, &message_ids)
}

#[tauri::command]
pub fn bulk_mark_unread(app_handle: AppHandle, message_ids: Vec<i64>) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::bulk_mark_unread(pool, &message_ids)
}

#[tauri::command]
pub fn bulk_delete_messages(app_handle: AppHandle, message_ids: Vec<i64>) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::bulk_delete_messages(pool, &message_ids)
}

#[tauri::command]
pub fn bulk_star_messages(app_handle: AppHandle, message_ids: Vec<i64>) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::bulk_star_messages(pool, &message_ids)
}

#[tauri::command]
pub fn bulk_unstar_messages(
    app_handle: AppHandle,
    message_ids: Vec<i64>,
) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::cache::db::bulk_unstar_messages(pool, &message_ids)
}

// ============================================================================
// CONTACTS OPERATIONS
// ============================================================================

#[tauri::command]
pub fn search_contacts(
    app_handle: AppHandle,
    account_id: i64,
    query: String,
) -> Result<Vec<crate::models::Contact>, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::contacts::search_contacts(pool, account_id, &query)
}

#[tauri::command]
pub fn get_recent_contacts(
    app_handle: AppHandle,
    account_id: i64,
    limit: i32,
) -> Result<Vec<crate::models::Contact>, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::contacts::get_recent_contacts(pool, account_id, limit)
}

#[tauri::command]
pub fn get_frequent_contacts(
    app_handle: AppHandle,
    account_id: i64,
    limit: i32,
) -> Result<Vec<crate::models::Contact>, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::contacts::get_frequent_contacts(pool, account_id, limit)
}

// ============================================================================
// THREADING OPERATIONS
// ============================================================================

#[tauri::command]
pub fn get_thread_messages(
    app_handle: AppHandle,
    thread_id: i64,
) -> Result<Vec<crate::models::Message>, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::threading::get_thread_messages(pool, thread_id)
}

#[tauri::command]
pub fn get_thread(
    app_handle: AppHandle,
    thread_id: i64,
) -> Result<crate::models::Thread, DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let pool = &app_state.db_pool;
    crate::core::threading::get_thread(pool, thread_id)
}
