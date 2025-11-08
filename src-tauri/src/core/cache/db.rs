use crate::core::cache::schema::initialize_schema;
use crate::error::DEmailError;
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;
use tauri::api::path::app_data_dir;
use tauri::Config;

use crate::models::{Attachment, Draft, EmailSignature, Folder, Message, AppSetting};

pub fn get_db_path(config: &Config) -> Result<PathBuf, DEmailError> {
    let path = app_data_dir(config)
        .ok_or_else(|| DEmailError::Init("Could not determine app data directory".to_string()))?;
    let path = path.join("demail.sqlite");
    Ok(path)
}

pub fn initialize_database(config: &Config) -> Result<Connection, DEmailError> {
    let path = get_db_path(config)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let conn = Connection::open(path)?;
    initialize_schema(&conn)?;
    Ok(conn)
}

pub fn save_folder(conn: &Connection, folder: &mut Folder) -> Result<(), DEmailError> {
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

pub fn save_message(conn: &Connection, message: &Message) -> Result<(), DEmailError> {
    conn.execute(
        "INSERT OR REPLACE INTO messages (id, account_id, folder_id, imap_uid, message_id_header, from_header, to_header, cc_header, subject, date, body_plain, body_html, has_attachments, is_read) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
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
            message.body_html,
            message.has_attachments,
            message.is_read,
        ],
    )?;
    Ok(())
}

pub fn save_attachment(conn: &Connection, attachment: &Attachment) -> Result<(), DEmailError> {
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

pub fn update_message_read_status(
    conn: &Connection,
    message_id: i64,
    is_read: bool,
) -> Result<(), DEmailError> {
    conn.execute(
        "UPDATE messages SET is_read = ?1 WHERE id = ?2",
        rusqlite::params![is_read, message_id],
    )?;
    Ok(())
}

pub fn search_messages_fts(
    conn: &Connection,
    account_id: i64,
    query: &str,
) -> Result<Vec<crate::models::MessageHeader>, DEmailError> {
    let mut stmt = conn.prepare(
        "SELECT m.id, m.subject, m.from_header, m.date, m.is_read, m.has_attachments
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
        })
    })?;

    let mut messages = Vec::new();
    for message in message_iter {
        messages.push(message?);
    }
    Ok(messages)
}

pub fn save_draft(conn: &Connection, draft: &Draft) -> Result<i64, DEmailError> {
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

pub fn get_drafts(conn: &Connection, account_id: i64) -> Result<Vec<Draft>, DEmailError> {
    let mut stmt = conn.prepare(
        "SELECT id, account_id, to_addresses, cc_addresses, bcc_addresses, subject, body_plain, body_html, created_at, updated_at
         FROM drafts WHERE account_id = ?1 ORDER BY updated_at DESC"
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

pub fn delete_draft(conn: &Connection, draft_id: i64) -> Result<(), DEmailError> {
    conn.execute("DELETE FROM drafts WHERE id = ?1", [draft_id])?;
    Ok(())
}

pub fn save_signature(conn: &Connection, signature: &EmailSignature) -> Result<i64, DEmailError> {
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

pub fn get_signatures(conn: &Connection, account_id: i64) -> Result<Vec<EmailSignature>, DEmailError> {
    let mut stmt = conn.prepare(
        "SELECT id, account_id, name, content_html, content_plain, is_default
         FROM signatures WHERE account_id = ?1 ORDER BY is_default DESC, name"
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

pub fn delete_signature(conn: &Connection, signature_id: i64) -> Result<(), DEmailError> {
    conn.execute("DELETE FROM signatures WHERE id = ?1", [signature_id])?;
    Ok(())
}

pub fn save_setting(conn: &Connection, key: &str, value: &str) -> Result<(), DEmailError> {
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        rusqlite::params![key, value],
    )?;
    Ok(())
}

pub fn get_setting(conn: &Connection, key: &str) -> Result<Option<String>, DEmailError> {
    let result = conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        [key],
        |row| row.get(0)
    );

    match result {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(DEmailError::from(e)),
    }
}

pub fn get_all_settings(conn: &Connection) -> Result<Vec<AppSetting>, DEmailError> {
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

pub fn save_attachment_data(conn: &Connection, attachment_id: i64, data: &[u8]) -> Result<(), DEmailError> {
    conn.execute(
        "INSERT OR REPLACE INTO attachment_data (attachment_id, data) VALUES (?1, ?2)",
        rusqlite::params![attachment_id, data],
    )?;
    Ok(())
}

pub fn get_attachment_data(conn: &Connection, attachment_id: i64) -> Result<Option<Vec<u8>>, DEmailError> {
    let result = conn.query_row(
        "SELECT data FROM attachment_data WHERE attachment_id = ?1",
        [attachment_id],
        |row| row.get(0)
    );

    match result {
        Ok(data) => Ok(Some(data)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(DEmailError::from(e)),
    }
}

pub fn delete_message(conn: &Connection, message_id: i64) -> Result<(), DEmailError> {
    conn.execute("DELETE FROM messages WHERE id = ?1", [message_id])?;
    Ok(())
}

pub fn move_message(conn: &Connection, message_id: i64, target_folder_id: i64) -> Result<(), DEmailError> {
    conn.execute(
        "UPDATE messages SET folder_id = ?1 WHERE id = ?2",
        rusqlite::params![target_folder_id, message_id],
    )?;
    Ok(())
}

pub fn get_messages_paginated(
    conn: &Connection,
    folder_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<crate::models::MessageHeader>, DEmailError> {
    let mut stmt = conn.prepare(
        "SELECT id, subject, from_header, date, is_read, has_attachments
         FROM messages
         WHERE folder_id = ?1
         ORDER BY date DESC
         LIMIT ?2 OFFSET ?3"
    )?;

    let message_iter = stmt.query_map(rusqlite::params![folder_id, limit, offset], |row| {
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

pub fn count_messages_in_folder(conn: &Connection, folder_id: i64) -> Result<i64, DEmailError> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM messages WHERE folder_id = ?1",
        [folder_id],
        |row| row.get(0)
    )?;
    Ok(count)
}
