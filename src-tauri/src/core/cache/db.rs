use crate::core::cache::schema::initialize_schema;
use crate::core::migrations;
use crate::error::DEmailError;
use crate::models::{Attachment, AppSetting, Draft, EmailSignature, Folder, Message};
use ammonia;
use r2d2;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;
use tauri::api::path::app_data_dir;
use tauri::Config;
use tracing::{debug, info};

/// Database connection pool type
pub type Pool = r2d2::Pool<SqliteConnectionManager>;

/// Gets the path to the SQLite database file
pub fn get_db_path(config: &Config) -> Result<PathBuf, DEmailError> {
    let path = app_data_dir(config)
        .ok_or_else(|| DEmailError::Init("Could not determine app data directory".to_string()))?;
    let path = path.join("demail.sqlite");
    Ok(path)
}

/// Initializes the database connection pool
/// This replaces the old initialize_database() function
pub fn initialize_pool(config: &Config) -> Result<Pool, DEmailError> {
    let path = get_db_path(config)?;

    // Create parent directory if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    info!("Initializing database pool at {:?}", path);

    // Create connection manager
    let manager = SqliteConnectionManager::file(path);

    // Build pool with configuration
    let pool = r2d2::Pool::builder()
        .max_size(15) // Maximum 15 connections in pool
        .build(manager)
        .map_err(|e| DEmailError::Init(format!("Failed to create connection pool: {}", e)))?;

    // Initialize schema and run migrations on a single connection
    {
        let conn = pool
            .get()
            .map_err(|e| DEmailError::Init(format!("Failed to get connection from pool: {}", e)))?;

        initialize_schema(&conn)?;
        migrations::apply_migrations(&conn)?;

        info!("Database schema initialized and migrations applied");
    }

    Ok(pool)
}

/// Legacy function for compatibility - creates a single connection
/// Used in tests and specific scenarios where pool is not needed
pub fn initialize_database(config: &Config) -> Result<Connection, DEmailError> {
    let path = get_db_path(config)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let conn = Connection::open(path)?;
    initialize_schema(&conn)?;
    Ok(conn)
}

// ============================================================================
// FOLDER OPERATIONS
// ============================================================================

pub fn save_folder(pool: &Pool, folder: &mut Folder) -> Result<(), DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    conn.execute(
        "INSERT INTO folders (account_id, name, path, parent_id, uid_validity) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![
            folder.account_id,
            folder.name,
            folder.path,
            folder.parent_id,
            0 // Placeholder for uid_validity
        ],
    )?;
    folder.id = conn.last_insert_rowid();
    Ok(())
}

// ============================================================================
// MESSAGE OPERATIONS
// ============================================================================

fn sanitize_html(html: &Option<String>) -> Option<String> {
    html.as_ref().map(|h| ammonia::clean(h))
}

pub fn save_message(pool: &Pool, message: &Message) -> Result<(), DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;
    let sanitized_html = sanitize_html(&message.body_html);

    conn.execute(
        "INSERT OR REPLACE INTO messages (id, account_id, folder_id, imap_uid, message_id_header, from_header, to_header, cc_header, subject, date, body_plain, body_html, has_attachments, is_read, is_starred, thread_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
        rusqlite::params![
            message.id,
            message.account_id,
            message.folder_id,
            message.imap_uid,
            message.message_id_header,
            message.from_header,
            message.to_header,
            message.cc_header,
            message.subject,
            message.date,
            message.body_plain,
            sanitized_html,
            message.has_attachments,
            message.is_read,
            0, // is_starred defaults to false
            None::<i64>, // thread_id will be set by threading module
        ],
    )?;
    Ok(())
}

pub fn update_message_read_status(
    pool: &Pool,
    message_id: i64,
    is_read: bool,
) -> Result<(), DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    conn.execute(
        "UPDATE messages SET is_read = ?1 WHERE id = ?2",
        rusqlite::params![is_read, message_id],
    )?;
    Ok(())
}

pub fn delete_message(pool: &Pool, message_id: i64) -> Result<(), DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    conn.execute("DELETE FROM messages WHERE id = ?1", [message_id])?;
    Ok(())
}

pub fn move_message(pool: &Pool, message_id: i64, target_folder_id: i64) -> Result<(), DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    conn.execute(
        "UPDATE messages SET folder_id = ?1 WHERE id = ?2",
        rusqlite::params![target_folder_id, message_id],
    )?;
    Ok(())
}

pub fn get_messages_paginated(
    pool: &Pool,
    folder_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<crate::models::MessageHeader>, DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    let mut stmt = conn.prepare(
        "SELECT id, subject, from_header, date, is_read, has_attachments, is_starred
         FROM messages
         WHERE folder_id = ?1
         ORDER BY date DESC
         LIMIT ?2 OFFSET ?3",
    )?;

    let message_iter = stmt.query_map(rusqlite::params![folder_id, limit, offset], |row| {
        Ok(crate::models::MessageHeader {
            id: row.get(0)?,
            subject: row.get(1)?,
            from: row.get(2)?,
            date: row.get(3)?,
            is_read: row.get(4)?,
            has_attachments: row.get(5)?,
            is_starred: row.get(6).unwrap_or(false),
        })
    })?;

    let mut messages = Vec::new();
    for message in message_iter {
        messages.push(message?);
    }
    Ok(messages)
}

pub fn count_messages_in_folder(pool: &Pool, folder_id: i64) -> Result<i64, DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM messages WHERE folder_id = ?1",
        [folder_id],
        |row| row.get(0),
    )?;
    Ok(count)
}

// ============================================================================
// STARRING OPERATIONS (NEW)
// ============================================================================

pub fn star_message(pool: &Pool, message_id: i64) -> Result<(), DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    conn.execute(
        "UPDATE messages SET is_starred = 1 WHERE id = ?1",
        [message_id],
    )?;
    debug!("Starred message {}", message_id);
    Ok(())
}

pub fn unstar_message(pool: &Pool, message_id: i64) -> Result<(), DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    conn.execute(
        "UPDATE messages SET is_starred = 0 WHERE id = ?1",
        [message_id],
    )?;
    debug!("Unstarred message {}", message_id);
    Ok(())
}

pub fn get_starred_messages(
    pool: &Pool,
    account_id: i64,
) -> Result<Vec<crate::models::MessageHeader>, DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    let mut stmt = conn.prepare(
        "SELECT id, subject, from_header, date, is_read, has_attachments, is_starred
         FROM messages
         WHERE account_id = ?1 AND is_starred = 1
         ORDER BY date DESC",
    )?;

    let message_iter = stmt.query_map([account_id], |row| {
        Ok(crate::models::MessageHeader {
            id: row.get(0)?,
            subject: row.get(1)?,
            from: row.get(2)?,
            date: row.get(3)?,
            is_read: row.get(4)?,
            has_attachments: row.get(5)?,
            is_starred: row.get(6).unwrap_or(true),
        })
    })?;

    let mut messages = Vec::new();
    for message in message_iter {
        messages.push(message?);
    }
    Ok(messages)
}

// ============================================================================
// BULK OPERATIONS (NEW)
// ============================================================================

pub fn bulk_mark_read(pool: &Pool, message_ids: &[i64]) -> Result<(), DEmailError> {
    if message_ids.is_empty() {
        return Ok(());
    }

    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    // Build placeholders for IN clause
    let placeholders = message_ids
        .iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(",");

    let query = format!("UPDATE messages SET is_read = 1 WHERE id IN ({})", placeholders);

    let params: Vec<&dyn rusqlite::ToSql> = message_ids
        .iter()
        .map(|id| id as &dyn rusqlite::ToSql)
        .collect();

    conn.execute(&query, rusqlite::params_from_iter(params))?;
    debug!("Bulk marked {} messages as read", message_ids.len());
    Ok(())
}

pub fn bulk_delete_messages(pool: &Pool, message_ids: &[i64]) -> Result<(), DEmailError> {
    if message_ids.is_empty() {
        return Ok(());
    }

    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    let placeholders = message_ids
        .iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(",");

    let query = format!("DELETE FROM messages WHERE id IN ({})", placeholders);

    let params: Vec<&dyn rusqlite::ToSql> = message_ids
        .iter()
        .map(|id| id as &dyn rusqlite::ToSql)
        .collect();

    conn.execute(&query, rusqlite::params_from_iter(params))?;
    debug!("Bulk deleted {} messages", message_ids.len());
    Ok(())
}

pub fn bulk_mark_unread(pool: &Pool, message_ids: &[i64]) -> Result<(), DEmailError> {
    if message_ids.is_empty() {
        return Ok(());
    }

    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    let placeholders = message_ids
        .iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(",");

    let query = format!("UPDATE messages SET is_read = 0 WHERE id IN ({})", placeholders);

    let params: Vec<&dyn rusqlite::ToSql> = message_ids
        .iter()
        .map(|id| id as &dyn rusqlite::ToSql)
        .collect();

    conn.execute(&query, rusqlite::params_from_iter(params))?;
    debug!("Bulk marked {} messages as unread", message_ids.len());
    Ok(())
}

pub fn bulk_star_messages(pool: &Pool, message_ids: &[i64]) -> Result<(), DEmailError> {
    if message_ids.is_empty() {
        return Ok(());
    }

    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    let placeholders = message_ids
        .iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(",");

    let query = format!("UPDATE messages SET is_starred = 1 WHERE id IN ({})", placeholders);

    let params: Vec<&dyn rusqlite::ToSql> = message_ids
        .iter()
        .map(|id| id as &dyn rusqlite::ToSql)
        .collect();

    conn.execute(&query, rusqlite::params_from_iter(params))?;
    debug!("Bulk starred {} messages", message_ids.len());
    Ok(())
}

pub fn bulk_unstar_messages(pool: &Pool, message_ids: &[i64]) -> Result<(), DEmailError> {
    if message_ids.is_empty() {
        return Ok(());
    }

    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    let placeholders = message_ids
        .iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(",");

    let query = format!("UPDATE messages SET is_starred = 0 WHERE id IN ({})", placeholders);

    let params: Vec<&dyn rusqlite::ToSql> = message_ids
        .iter()
        .map(|id| id as &dyn rusqlite::ToSql)
        .collect();

    conn.execute(&query, rusqlite::params_from_iter(params))?;
    debug!("Bulk unstarred {} messages", message_ids.len());
    Ok(())
}

// ============================================================================
// SEARCH OPERATIONS
// ============================================================================

pub fn search_messages_fts(
    pool: &Pool,
    account_id: i64,
    query: &str,
) -> Result<Vec<crate::models::MessageHeader>, DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    let mut stmt = conn.prepare(
        "SELECT m.id, m.subject, m.from_header, m.date, m.is_read, m.has_attachments, m.is_starred
         FROM messages m
         INNER JOIN messages_fts fts ON m.id = fts.rowid
         WHERE m.account_id = ?1 AND messages_fts MATCH ?2
         ORDER BY m.date DESC",
    )?;

    let message_iter = stmt.query_map(rusqlite::params![account_id, query], |row| {
        Ok(crate::models::MessageHeader {
            id: row.get(0)?,
            subject: row.get(1)?,
            from: row.get(2)?,
            date: row.get(3)?,
            is_read: row.get(4)?,
            has_attachments: row.get(5)?,
            is_starred: row.get(6).unwrap_or(false),
        })
    })?;

    let mut messages = Vec::new();
    for message in message_iter {
        messages.push(message?);
    }
    Ok(messages)
}

// ============================================================================
// ATTACHMENT OPERATIONS
// ============================================================================

pub fn save_attachment(pool: &Pool, attachment: &Attachment) -> Result<(), DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    conn.execute(
        "INSERT INTO attachments (message_id, filename, mime_type, size_bytes, local_path) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![
            attachment.message_id,
            attachment.filename,
            attachment.mime_type,
            attachment.size_bytes,
            attachment.local_path,
        ],
    )?;
    Ok(())
}

pub fn get_attachments_for_message(
    pool: &Pool,
    message_id: i64,
) -> Result<Vec<Attachment>, DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    let mut stmt = conn.prepare(
        "SELECT id, message_id, filename, mime_type, size_bytes, local_path
         FROM attachments WHERE message_id = ?1",
    )?;

    let attachment_iter = stmt.query_map([message_id], |row| {
        Ok(Attachment {
            id: row.get(0)?,
            message_id: row.get(1)?,
            filename: row.get(2)?,
            mime_type: row.get(3)?,
            size_bytes: row.get(4)?,
            local_path: row.get(5)?,
        })
    })?;

    let mut attachments = Vec::new();
    for attachment in attachment_iter {
        attachments.push(attachment?);
    }
    Ok(attachments)
}

pub fn save_attachment_data(pool: &Pool, attachment_id: i64, data: &[u8]) -> Result<(), DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    conn.execute(
        "INSERT OR REPLACE INTO attachment_data (attachment_id, data) VALUES (?1, ?2)",
        rusqlite::params![attachment_id, data],
    )?;
    Ok(())
}

pub fn get_attachment_data(pool: &Pool, attachment_id: i64) -> Result<Option<Vec<u8>>, DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    let result = conn.query_row(
        "SELECT data FROM attachment_data WHERE attachment_id = ?1",
        [attachment_id],
        |row| row.get(0),
    );

    match result {
        Ok(data) => Ok(Some(data)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(DEmailError::from(e)),
    }
}

// ============================================================================
// DRAFT OPERATIONS
// ============================================================================

pub fn save_draft(pool: &Pool, draft: &Draft) -> Result<i64, DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;
    let now = chrono::Utc::now().timestamp();

    if draft.id == 0 {
        conn.execute(
            "INSERT INTO drafts (account_id, to_addresses, cc_addresses, bcc_addresses, subject, body_plain, body_html, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                draft.account_id,
                draft.to_addresses,
                draft.cc_addresses,
                draft.bcc_addresses,
                draft.subject,
                draft.body_plain,
                draft.body_html,
                now,
                now,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    } else {
        conn.execute(
            "UPDATE drafts SET to_addresses = ?1, cc_addresses = ?2, bcc_addresses = ?3, subject = ?4, body_plain = ?5, body_html = ?6, updated_at = ?7 WHERE id = ?8",
            rusqlite::params![
                draft.to_addresses,
                draft.cc_addresses,
                draft.bcc_addresses,
                draft.subject,
                draft.body_plain,
                draft.body_html,
                now,
                draft.id,
            ],
        )?;
        Ok(draft.id)
    }
}

pub fn get_drafts(pool: &Pool, account_id: i64) -> Result<Vec<Draft>, DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    let mut stmt = conn.prepare(
        "SELECT id, account_id, to_addresses, cc_addresses, bcc_addresses, subject, body_plain, body_html, created_at, updated_at
         FROM drafts WHERE account_id = ?1 ORDER BY updated_at DESC",
    )?;

    let draft_iter = stmt.query_map([account_id], |row| {
        Ok(Draft {
            id: row.get(0)?,
            account_id: row.get(1)?,
            to_addresses: row.get(2)?,
            cc_addresses: row.get(3)?,
            bcc_addresses: row.get(4)?,
            subject: row.get(5)?,
            body_plain: row.get(6)?,
            body_html: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;

    let mut drafts = Vec::new();
    for draft in draft_iter {
        drafts.push(draft?);
    }
    Ok(drafts)
}

pub fn delete_draft(pool: &Pool, draft_id: i64) -> Result<(), DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    conn.execute("DELETE FROM drafts WHERE id = ?1", [draft_id])?;
    Ok(())
}

// ============================================================================
// SIGNATURE OPERATIONS
// ============================================================================

pub fn save_signature(pool: &Pool, signature: &EmailSignature) -> Result<i64, DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    if signature.is_default {
        conn.execute(
            "UPDATE signatures SET is_default = 0 WHERE account_id = ?1",
            [signature.account_id],
        )?;
    }

    if signature.id == 0 {
        conn.execute(
            "INSERT INTO signatures (account_id, name, content_html, content_plain, is_default)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                signature.account_id,
                signature.name,
                signature.content_html,
                signature.content_plain,
                signature.is_default,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    } else {
        conn.execute(
            "UPDATE signatures SET name = ?1, content_html = ?2, content_plain = ?3, is_default = ?4 WHERE id = ?5",
            rusqlite::params![
                signature.name,
                signature.content_html,
                signature.content_plain,
                signature.is_default,
                signature.id,
            ],
        )?;
        Ok(signature.id)
    }
}

pub fn get_signatures(pool: &Pool, account_id: i64) -> Result<Vec<EmailSignature>, DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    let mut stmt = conn.prepare(
        "SELECT id, account_id, name, content_html, content_plain, is_default
         FROM signatures WHERE account_id = ?1 ORDER BY is_default DESC, name",
    )?;

    let signature_iter = stmt.query_map([account_id], |row| {
        Ok(EmailSignature {
            id: row.get(0)?,
            account_id: row.get(1)?,
            name: row.get(2)?,
            content_html: row.get(3)?,
            content_plain: row.get(4)?,
            is_default: row.get(5)?,
        })
    })?;

    let mut signatures = Vec::new();
    for signature in signature_iter {
        signatures.push(signature?);
    }
    Ok(signatures)
}

pub fn delete_signature(pool: &Pool, signature_id: i64) -> Result<(), DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    conn.execute("DELETE FROM signatures WHERE id = ?1", [signature_id])?;
    Ok(())
}

// ============================================================================
// SETTINGS OPERATIONS
// ============================================================================

pub fn save_setting(pool: &Pool, key: &str, value: &str) -> Result<(), DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        rusqlite::params![key, value],
    )?;
    Ok(())
}

pub fn get_setting(pool: &Pool, key: &str) -> Result<Option<String>, DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    let result = conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        [key],
        |row| row.get(0),
    );

    match result {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(DEmailError::from(e)),
    }
}

pub fn get_all_settings(pool: &Pool) -> Result<Vec<AppSetting>, DEmailError> {
    let conn = pool.get().map_err(|e| DEmailError::Database(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))?;

    let mut stmt = conn.prepare("SELECT key, value FROM settings")?;

    let setting_iter = stmt.query_map([], |row| {
        Ok(AppSetting {
            key: row.get(0)?,
            value: row.get(1)?,
        })
    })?;

    let mut settings = Vec::new();
    for setting in setting_iter {
        settings.push(setting?);
    }
    Ok(settings)
}
