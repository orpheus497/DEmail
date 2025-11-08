// Email threading module - Groups related emails into conversation threads

use crate::error::DEmailError;
use crate::models::{Message, Thread};
use rusqlite::Connection;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Creates or updates a thread for a message based on In-Reply-To and References headers
/// Uses subject-based threading with hash matching for grouping conversations
pub fn create_or_update_thread(
    conn: &Connection,
    message: &Message,
) -> Result<i64, DEmailError> {
    // Normalize subject by removing Re:, Fwd:, etc.
    let normalized_subject = normalize_subject(&message.subject);
    let subject_hash = calculate_subject_hash(&normalized_subject);

    // Try to find existing thread by subject hash for this account
    let existing_thread: Option<i64> = conn
        .query_row(
            "SELECT id FROM threads WHERE subject_hash = ?1 AND account_id = ?2 LIMIT 1",
            rusqlite::params![&subject_hash, message.account_id],
            |row| row.get(0),
        )
        .ok();

    if let Some(thread_id) = existing_thread {
        // Update existing thread
        update_thread(conn, thread_id, message.id)?;

        // Link message to thread
        conn.execute(
            "UPDATE messages SET thread_id = ?1 WHERE id = ?2",
            rusqlite::params![thread_id, message.id],
        )?;

        Ok(thread_id)
    } else {
        // Create new thread
        let now = chrono::Utc::now().timestamp();
        conn.execute(
            "INSERT INTO threads (subject_hash, first_message_id, last_message_id, message_count, account_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, 1, ?4, ?5, ?6)",
            rusqlite::params![&subject_hash, message.id, message.id, message.account_id, now, now],
        )?;

        let thread_id = conn.last_insert_rowid();

        // Link message to thread
        conn.execute(
            "UPDATE messages SET thread_id = ?1 WHERE id = ?2",
            rusqlite::params![thread_id, message.id],
        )?;

        Ok(thread_id)
    }
}

/// Gets all messages in a thread, ordered by date
pub fn get_thread_messages(
    conn: &Connection,
    thread_id: i64,
) -> Result<Vec<Message>, DEmailError> {
    let mut stmt = conn.prepare(
        "SELECT id, account_id, folder_id, imap_uid, message_id_header, from_header, to_header, cc_header,
         subject, date, body_plain, body_html, has_attachments, is_read, is_starred, thread_id
         FROM messages WHERE thread_id = ?1 ORDER BY date ASC",
    )?;

    let message_iter = stmt.query_map([thread_id], |row| {
        Ok(Message {
            id: row.get(0)?,
            account_id: row.get(1)?,
            folder_id: row.get(2)?,
            imap_uid: row.get(3)?,
            message_id_header: row.get(4)?,
            from_header: row.get(5)?,
            to_header: row.get(6)?,
            cc_header: row.get(7)?,
            subject: row.get(8)?,
            date: row.get(9)?,
            body_plain: row.get(10)?,
            body_html: row.get(11)?,
            has_attachments: row.get(12)?,
            is_read: row.get(13)?,
            is_starred: row.get(14)?,
            thread_id: row.get(15)?,
            attachments: Vec::new(), // Loaded separately if needed
        })
    })?;

    let mut messages = Vec::new();
    for message in message_iter {
        messages.push(message?);
    }

    Ok(messages)
}

/// Gets thread information by ID
pub fn get_thread(conn: &Connection, thread_id: i64) -> Result<Thread, DEmailError> {
    conn.query_row(
        "SELECT id, subject_hash, first_message_id, last_message_id, message_count, account_id, created_at, updated_at
         FROM threads WHERE id = ?1",
        [thread_id],
        |row| {
            Ok(Thread {
                id: row.get(0)?,
                subject_hash: row.get(1)?,
                first_message_id: row.get(2)?,
                last_message_id: row.get(3)?,
                message_count: row.get(4)?,
                account_id: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        },
    )
    .map_err(|e| DEmailError::Database(e))
}

/// Updates thread metadata when a new message is added
fn update_thread(conn: &Connection, thread_id: i64, new_message_id: i64) -> Result<(), DEmailError> {
    let now = chrono::Utc::now().timestamp();

    conn.execute(
        "UPDATE threads SET last_message_id = ?1, message_count = message_count + 1, updated_at = ?2
         WHERE id = ?3",
        rusqlite::params![new_message_id, now, thread_id],
    )?;

    Ok(())
}

/// Normalizes email subject for threading by removing prefixes
fn normalize_subject(subject: &str) -> String {
    let mut normalized = subject.trim().to_lowercase();

    // Remove common reply/forward prefixes
    loop {
        let before = normalized.clone();

        // Remove Re:, RE:, re:
        if normalized.starts_with("re:") {
            normalized = normalized[3..].trim_start().to_string();
            continue;
        }

        // Remove Fwd:, FWD:, fwd:
        if normalized.starts_with("fwd:") || normalized.starts_with("fw:") {
            let prefix_len = if normalized.starts_with("fwd:") { 4 } else { 3 };
            normalized = normalized[prefix_len..].trim_start().to_string();
            continue;
        }

        // Remove [list-name] prefixes
        if normalized.starts_with('[') {
            if let Some(end_bracket) = normalized.find(']') {
                normalized = normalized[end_bracket + 1..].trim_start().to_string();
                continue;
            }
        }

        // No changes made, break
        if before == normalized {
            break;
        }
    }

    normalized
}

/// Calculates a hash of the normalized subject for thread grouping
fn calculate_subject_hash(subject: &str) -> String {
    let mut hasher = DefaultHasher::new();
    subject.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_subject() {
        assert_eq!(normalize_subject("Re: Hello World"), "hello world");
        assert_eq!(normalize_subject("RE: Re: Test"), "test");
        assert_eq!(normalize_subject("Fwd: Important"), "important");
        assert_eq!(normalize_subject("[List] Re: Topic"), "topic");
        assert_eq!(normalize_subject("  Re:  Spaces  "), "spaces");
    }

    #[test]
    fn test_subject_hash_consistency() {
        let hash1 = calculate_subject_hash("hello world");
        let hash2 = calculate_subject_hash("hello world");
        assert_eq!(hash1, hash2);

        let hash3 = calculate_subject_hash("different");
        assert_ne!(hash1, hash3);
    }
}
