use crate::error::DEmailError;
use std::path::{Path, PathBuf};
use validator::ValidateEmail;

const MAX_SUBJECT_LENGTH: usize = 998; // RFC 2822 limit
const MAX_BODY_LENGTH: usize = 10_000_000; // 10MB limit for email body
const MAX_TO_ADDRESSES_LENGTH: usize = 2000; // Multiple email addresses
const MAX_FILENAME_LENGTH: usize = 255; // Most filesystems limit

/// Validates an email address using RFC 5322 standards
pub fn validate_email(email: &str) -> Result<(), DEmailError> {
    if email.is_empty() {
        return Err(DEmailError::Validation(
            "Email address cannot be empty".to_string(),
        ));
    }

    if !email.validate_email() {
        return Err(DEmailError::Validation(format!(
            "Invalid email address format: {}",
            email
        )));
    }

    Ok(())
}

/// Validates email subject line
pub fn validate_subject(subject: &str) -> Result<(), DEmailError> {
    if subject.len() > MAX_SUBJECT_LENGTH {
        return Err(DEmailError::Validation(format!(
            "Subject too long: {} characters (max {})",
            subject.len(),
            MAX_SUBJECT_LENGTH
        )));
    }

    // Check for control characters that could break email headers
    if subject.chars().any(|c| c.is_control() && c != '\t') {
        return Err(DEmailError::Validation(
            "Subject contains invalid control characters".to_string(),
        ));
    }

    Ok(())
}

/// Validates email body content
pub fn validate_body(body: &str) -> Result<(), DEmailError> {
    if body.len() > MAX_BODY_LENGTH {
        return Err(DEmailError::Validation(format!(
            "Body too long: {} bytes (max {})",
            body.len(),
            MAX_BODY_LENGTH
        )));
    }

    Ok(())
}

/// Validates To/CC/BCC address lists (comma or semicolon separated)
pub fn validate_address_list(addresses: &str) -> Result<(), DEmailError> {
    if addresses.is_empty() {
        return Err(DEmailError::Validation(
            "Address list cannot be empty".to_string(),
        ));
    }

    if addresses.len() > MAX_TO_ADDRESSES_LENGTH {
        return Err(DEmailError::Validation(format!(
            "Address list too long: {} characters (max {})",
            addresses.len(),
            MAX_TO_ADDRESSES_LENGTH
        )));
    }

    // Split by common delimiters and validate each email
    let emails: Vec<&str> = addresses
        .split(|c| c == ',' || c == ';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if emails.is_empty() {
        return Err(DEmailError::Validation(
            "No valid email addresses found in list".to_string(),
        ));
    }

    for email in emails {
        validate_email(email)?;
    }

    Ok(())
}

/// Sanitizes a filename to prevent path traversal and invalid characters
pub fn sanitize_filename(filename: &str) -> Result<String, DEmailError> {
    if filename.is_empty() {
        return Err(DEmailError::Validation(
            "Filename cannot be empty".to_string(),
        ));
    }

    if filename.len() > MAX_FILENAME_LENGTH {
        return Err(DEmailError::Validation(format!(
            "Filename too long: {} characters (max {})",
            filename.len(),
            MAX_FILENAME_LENGTH
        )));
    }

    // Use sanitize-filename crate to remove dangerous characters
    let sanitized = sanitize_filename::sanitize(filename);

    if sanitized.is_empty() {
        return Err(DEmailError::Validation(
            "Filename contains only invalid characters".to_string(),
        ));
    }

    // Additional check: prevent directory traversal attempts
    if sanitized.contains("..") || sanitized.contains('/') || sanitized.contains('\\') {
        return Err(DEmailError::Validation(
            "Filename contains path traversal characters".to_string(),
        ));
    }

    Ok(sanitized)
}

/// Validates and sanitizes a file path to ensure it's within an allowed directory
pub fn validate_path(path: &Path, allowed_base: &Path) -> Result<PathBuf, DEmailError> {
    // Canonicalize both paths to resolve any .. or symlinks
    let canonical_base = allowed_base
        .canonicalize()
        .map_err(|e| DEmailError::Validation(format!("Invalid base directory: {}", e)))?;

    let canonical_path = path.canonicalize().unwrap_or_else(|_| {
        // If path doesn't exist yet, try to construct it
        if let Some(parent) = path.parent() {
            if let Ok(canonical_parent) = parent.canonicalize() {
                if let Some(filename) = path.file_name() {
                    return canonical_parent.join(filename);
                }
            }
        }
        path.to_path_buf()
    });

    // Ensure the path is within the allowed base directory
    if !canonical_path.starts_with(&canonical_base) {
        return Err(DEmailError::Validation(format!(
            "Path '{}' is outside allowed directory '{}'",
            canonical_path.display(),
            canonical_base.display()
        )));
    }

    Ok(canonical_path)
}

/// Validates a message ID to prevent injection attacks
pub fn validate_message_id(message_id: i64) -> Result<(), DEmailError> {
    if message_id <= 0 {
        return Err(DEmailError::Validation(format!(
            "Invalid message ID: {}",
            message_id
        )));
    }
    Ok(())
}

/// Validates an account ID
pub fn validate_account_id(account_id: i64) -> Result<(), DEmailError> {
    if account_id <= 0 {
        return Err(DEmailError::Validation(format!(
            "Invalid account ID: {}",
            account_id
        )));
    }
    Ok(())
}

/// Validates a folder ID
pub fn validate_folder_id(folder_id: i64) -> Result<(), DEmailError> {
    if folder_id <= 0 {
        return Err(DEmailError::Validation(format!(
            "Invalid folder ID: {}",
            folder_id
        )));
    }
    Ok(())
}

/// Validates pagination parameters
pub fn validate_pagination(limit: i64, offset: i64) -> Result<(), DEmailError> {
    if limit <= 0 || limit > 1000 {
        return Err(DEmailError::Validation(format!(
            "Invalid limit: {} (must be 1-1000)",
            limit
        )));
    }

    if offset < 0 {
        return Err(DEmailError::Validation(format!(
            "Invalid offset: {} (must be >= 0)",
            offset
        )));
    }

    Ok(())
}

/// Validates a search query to prevent injection
pub fn validate_search_query(query: &str) -> Result<(), DEmailError> {
    if query.is_empty() {
        return Err(DEmailError::Validation(
            "Search query cannot be empty".to_string(),
        ));
    }

    if query.len() > 500 {
        return Err(DEmailError::Validation(format!(
            "Search query too long: {} characters (max 500)",
            query.len()
        )));
    }

    // Check for SQL injection patterns (basic check, FTS5 should handle this)
    let lowercase = query.to_lowercase();
    if lowercase.contains("--") || lowercase.contains(";") || lowercase.contains("/*") {
        return Err(DEmailError::Validation(
            "Search query contains suspicious characters".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_validate_email() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("test.email+filter@domain.co.uk").is_ok());
        assert!(validate_email("").is_err());
        assert!(validate_email("not-an-email").is_err());
        assert!(validate_email("@example.com").is_err());
    }

    #[test]
    fn test_validate_subject() {
        assert!(validate_subject("Normal subject").is_ok());
        assert!(validate_subject("").is_ok()); // Empty subject is technically valid
        assert!(validate_subject("A".repeat(998)).is_ok());
        assert!(validate_subject("A".repeat(999)).is_err());
        assert!(validate_subject("Subject\nwith\nnewlines").is_err());
    }

    #[test]
    fn test_validate_body() {
        assert!(validate_body("Normal email body").is_ok());
        assert!(validate_body("A".repeat(1_000_000)).is_ok());
        assert!(validate_body("A".repeat(10_000_001)).is_err());
    }

    #[test]
    fn test_validate_address_list() {
        assert!(validate_address_list("user@example.com").is_ok());
        assert!(validate_address_list("user1@example.com, user2@example.com").is_ok());
        assert!(validate_address_list("user1@example.com; user2@example.com").is_ok());
        assert!(validate_address_list("").is_err());
        assert!(validate_address_list("invalid, also-invalid").is_err());
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("normal.txt").unwrap(), "normal.txt");
        assert_eq!(
            sanitize_filename("file with spaces.pdf").unwrap(),
            "file with spaces.pdf"
        );
        assert!(sanitize_filename("../../../etc/passwd").is_err());
        assert!(sanitize_filename("").is_err());
        assert!(sanitize_filename("A".repeat(300)).is_err());
    }

    #[test]
    fn test_validate_message_id() {
        assert!(validate_message_id(1).is_ok());
        assert!(validate_message_id(9999).is_ok());
        assert!(validate_message_id(0).is_err());
        assert!(validate_message_id(-1).is_err());
    }

    #[test]
    fn test_validate_pagination() {
        assert!(validate_pagination(25, 0).is_ok());
        assert!(validate_pagination(100, 50).is_ok());
        assert!(validate_pagination(0, 0).is_err());
        assert!(validate_pagination(1001, 0).is_err());
        assert!(validate_pagination(50, -1).is_err());
    }

    #[test]
    fn test_validate_search_query() {
        assert!(validate_search_query("normal search").is_ok());
        assert!(validate_search_query("email@example.com").is_ok());
        assert!(validate_search_query("").is_err());
        assert!(validate_search_query("A".repeat(501)).is_err());
        assert!(validate_search_query("DROP TABLE--").is_err());
        assert!(validate_search_query("/* comment */").is_err());
    }
}
