// Contact management module - Extracts and manages email contacts for autocomplete

use crate::error::DEmailError;
use crate::core::validation;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contact {
    pub id: i64,
    pub email: String,
    pub name: Option<String>,
    pub last_used: i64,
    pub use_count: i64,
}

/// Extracts email addresses from a message and saves/updates contacts
pub fn extract_and_save_contacts(
    conn: &Connection,
    from: &str,
    to: &str,
    cc: Option<&str>,
) -> Result<(), DEmailError> {
    let now = chrono::Utc::now().timestamp();

    // Extract and save from address
    if let Some((email, name)) = parse_email_address(from) {
        if validation::validate_email(&email).is_ok() {
            save_or_update_contact(conn, &email, name.as_deref(), now)?;
        }
    }

    // Extract and save to addresses
    for addr in split_email_addresses(to) {
        if let Some((email, name)) = parse_email_address(&addr) {
            if validation::validate_email(&email).is_ok() {
                save_or_update_contact(conn, &email, name.as_deref(), now)?;
            }
        }
    }

    // Extract and save cc addresses if present
    if let Some(cc_str) = cc {
        for addr in split_email_addresses(cc_str) {
            if let Some((email, name)) = parse_email_address(&addr) {
                if validation::validate_email(&email).is_ok() {
                    save_or_update_contact(conn, &email, name.as_deref(), now)?;
                }
            }
        }
    }

    Ok(())
}

/// Searches contacts by email or name (for autocomplete)
pub fn search_contacts(
    conn: &Connection,
    query: &str,
) -> Result<Vec<Contact>, DEmailError> {
    let search_pattern = format!("%{}%", query.to_lowercase());

    let mut stmt = conn.prepare(
        "SELECT id, email, name, last_used, use_count FROM contacts
         WHERE LOWER(email) LIKE ?1 OR LOWER(name) LIKE ?1
         ORDER BY use_count DESC, last_used DESC
         LIMIT 20",
    )?;

    let contact_iter = stmt.query_map([&search_pattern], |row| {
        Ok(Contact {
            id: row.get(0)?,
            email: row.get(1)?,
            name: row.get(2)?,
            last_used: row.get(3)?,
            use_count: row.get(4)?,
        })
    })?;

    let mut contacts = Vec::new();
    for contact in contact_iter {
        contacts.push(contact?);
    }

    Ok(contacts)
}

/// Gets recently used contacts
pub fn get_recent_contacts(
    conn: &Connection,
    limit: i64,
) -> Result<Vec<Contact>, DEmailError> {
    let mut stmt = conn.prepare(
        "SELECT id, email, name, last_used, use_count FROM contacts
         ORDER BY last_used DESC
         LIMIT ?1",
    )?;

    let contact_iter = stmt.query_map([limit], |row| {
        Ok(Contact {
            id: row.get(0)?,
            email: row.get(1)?,
            name: row.get(2)?,
            last_used: row.get(3)?,
            use_count: row.get(4)?,
        })
    })?;

    let mut contacts = Vec::new();
    for contact in contact_iter {
        contacts.push(contact?);
    }

    Ok(contacts)
}

/// Gets frequently used contacts
pub fn get_frequent_contacts(
    conn: &Connection,
    limit: i64,
) -> Result<Vec<Contact>, DEmailError> {
    let mut stmt = conn.prepare(
        "SELECT id, email, name, last_used, use_count FROM contacts
         ORDER BY use_count DESC, last_used DESC
         LIMIT ?1",
    )?;

    let contact_iter = stmt.query_map([limit], |row| {
        Ok(Contact {
            id: row.get(0)?,
            email: row.get(1)?,
            name: row.get(2)?,
            last_used: row.get(3)?,
            use_count: row.get(4)?,
        })
    })?;

    let mut contacts = Vec::new();
    for contact in contact_iter {
        contacts.push(contact?);
    }

    Ok(contacts)
}

/// Saves or updates a contact in the database
fn save_or_update_contact(
    conn: &Connection,
    email: &str,
    name: Option<&str>,
    timestamp: i64,
) -> Result<(), DEmailError> {
    // Check if contact exists
    let existing: Option<i64> = conn
        .query_row(
            "SELECT id FROM contacts WHERE email = ?1",
            [email],
            |row| row.get(0),
        )
        .ok();

    if let Some(contact_id) = existing {
        // Update existing contact
        if let Some(new_name) = name {
            // Update name if provided and not empty
            if !new_name.trim().is_empty() {
                conn.execute(
                    "UPDATE contacts SET name = ?1, last_used = ?2, use_count = use_count + 1 WHERE id = ?3",
                    rusqlite::params![new_name, timestamp, contact_id],
                )?;
            } else {
                conn.execute(
                    "UPDATE contacts SET last_used = ?1, use_count = use_count + 1 WHERE id = ?2",
                    rusqlite::params![timestamp, contact_id],
                )?;
            }
        } else {
            conn.execute(
                "UPDATE contacts SET last_used = ?1, use_count = use_count + 1 WHERE id = ?2",
                rusqlite::params![timestamp, contact_id],
            )?;
        }
    } else {
        // Insert new contact
        conn.execute(
            "INSERT INTO contacts (email, name, last_used, use_count) VALUES (?1, ?2, ?3, 1)",
            rusqlite::params![email, name, timestamp],
        )?;
    }

    Ok(())
}

/// Parses an email address string to extract email and optional name
/// Supports formats like "John Doe <john@example.com>" and "john@example.com"
fn parse_email_address(addr: &str) -> Option<(String, Option<String>)> {
    let trimmed = addr.trim();

    // Check for "Name <email>" format
    if let Some(start) = trimmed.find('<') {
        if let Some(end) = trimmed.find('>') {
            let email = trimmed[start + 1..end].trim().to_string();
            let name = trimmed[..start].trim();
            let name_cleaned = name.trim_matches('"').trim();

            return Some((
                email,
                if name_cleaned.is_empty() {
                    None
                } else {
                    Some(name_cleaned.to_string())
                },
            ));
        }
    }

    // Plain email address
    if !trimmed.is_empty() && trimmed.contains('@') {
        return Some((trimmed.to_string(), None));
    }

    None
}

/// Splits a comma or semicolon-separated list of email addresses
fn split_email_addresses(addresses: &str) -> Vec<String> {
    addresses
        .split(&[',', ';'][..])
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_email_address() {
        // Name and email format
        let (email, name) = parse_email_address("John Doe <john@example.com>").unwrap();
        assert_eq!(email, "john@example.com");
        assert_eq!(name, Some("John Doe".to_string()));

        // Plain email
        let (email, name) = parse_email_address("jane@example.com").unwrap();
        assert_eq!(email, "jane@example.com");
        assert_eq!(name, None);

        // Quoted name
        let (email, name) = parse_email_address("\"Alice Smith\" <alice@example.com>").unwrap();
        assert_eq!(email, "alice@example.com");
        assert_eq!(name, Some("Alice Smith".to_string()));

        // Invalid format
        assert!(parse_email_address("not an email").is_none());
        assert!(parse_email_address("").is_none());
    }

    #[test]
    fn test_split_email_addresses() {
        let addrs = split_email_addresses("john@example.com, jane@example.com");
        assert_eq!(addrs.len(), 2);
        assert_eq!(addrs[0], "john@example.com");
        assert_eq!(addrs[1], "jane@example.com");

        let addrs = split_email_addresses("alice@example.com; bob@example.com");
        assert_eq!(addrs.len(), 2);

        let addrs = split_email_addresses("single@example.com");
        assert_eq!(addrs.len(), 1);
    }
}
