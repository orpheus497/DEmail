use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DEmailError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("IMAP error: {0}")]
    Imap(#[from] imap::Error),
    #[error("SMTP error: {0}")]
    Smtp(#[from] lettre::transport::smtp::Error),
    #[error("OAuth error: {0}")]
    OAuth(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),
    #[error("Initialization error: {0}")]
    Init(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("API error: {0}")]
    Api(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Authentication failed: {0}")]
    Auth(String),
    #[error("PDF generation error: {0}")]
    Pdf(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl Serialize for DEmailError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
