use crate::core::cache::db::{get_attachment_data, save_attachment_data};
use crate::error::DEmailError;
use crate::models::Attachment;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::fs;
use std::path::Path;

pub fn save_attachment_to_disk(
    pool: &Pool<SqliteConnectionManager>,
    attachment: &Attachment,
    destination_path: &str,
) -> Result<(), DEmailError> {
    let data = get_attachment_data(pool, attachment.id)?
        .ok_or_else(|| DEmailError::NotFound(format!("Attachment data not found for ID {}", attachment.id)))?;

    fs::write(destination_path, data)?;
    tracing::info!("Attachment saved to: {}", destination_path);
    Ok(())
}

pub fn store_attachment_data(
    pool: &Pool<SqliteConnectionManager>,
    attachment_id: i64,
    data: Vec<u8>,
) -> Result<(), DEmailError> {
    save_attachment_data(pool, attachment_id, &data)
}

pub fn validate_attachment_safety(filename: &str, size_bytes: i64) -> Result<(), DEmailError> {
    const MAX_ATTACHMENT_SIZE: i64 = 25 * 1024 * 1024; // 25 MB

    if size_bytes > MAX_ATTACHMENT_SIZE {
        return Err(DEmailError::Unknown(format!(
            "Attachment too large: {} bytes (max: {} bytes)",
            size_bytes, MAX_ATTACHMENT_SIZE
        )));
    }

    let extension = Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    const DANGEROUS_EXTENSIONS: &[&str] = &[
        "exe", "bat", "cmd", "com", "pif", "scr", "vbs", "js", "jar", "msi", "dll", "sh"
    ];

    if DANGEROUS_EXTENSIONS.contains(&extension.to_lowercase().as_str()) {
        tracing::warn!("Potentially dangerous attachment type: {}", extension);
    }

    Ok(())
}

pub fn read_file_for_attachment(file_path: &str) -> Result<Vec<u8>, DEmailError> {
    let data = fs::read(file_path)?;

    let filename = Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    validate_attachment_safety(filename, data.len() as i64)?;

    Ok(data)
}
