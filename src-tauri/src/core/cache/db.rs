use crate::core::cache::schema::initialize_schema;
use crate::error::DEmailError;
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;
use tauri::api::path::app_data_dir;
use tauri::Config;

use crate::models::{Attachment, Folder, Message};

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
