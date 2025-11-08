use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: i64,
    pub email_address: String,
    pub display_name: String,
    pub provider_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Folder {
    pub id: i64,
    pub account_id: i64,
    pub name: String,
    pub path: String,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageHeader {
    pub id: i64,
    pub subject: String,
    pub from: String,
    pub date: i64,
    pub is_read: bool,
    pub has_attachments: bool,
    pub is_starred: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: i64,
    pub account_id: i64,
    pub folder_id: i64,
    pub imap_uid: u32,
    pub message_id_header: String,
    pub from_header: String,
    pub to_header: String,
    pub cc_header: Option<String>,
    pub subject: String,
    pub date: i64,
    pub body_plain: Option<String>,
    pub body_html: Option<String>,
    pub has_attachments: bool,
    pub is_read: bool,
    pub is_starred: bool,
    pub thread_id: Option<i64>,
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attachment {
    pub id: i64,
    pub message_id: i64,
    pub filename: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub local_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthProviderConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Draft {
    pub id: i64,
    pub account_id: i64,
    pub to_addresses: String,
    pub cc_addresses: Option<String>,
    pub bcc_addresses: Option<String>,
    pub subject: String,
    pub body_plain: Option<String>,
    pub body_html: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailSignature {
    pub id: i64,
    pub account_id: i64,
    pub name: String,
    pub content_html: String,
    pub content_plain: String,
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSetting {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thread {
    pub id: i64,
    pub subject_hash: String,
    pub first_message_id: i64,
    pub last_message_id: i64,
    pub message_count: i64,
    pub account_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contact {
    pub id: i64,
    pub email: String,
    pub name: Option<String>,
    pub last_used: i64,
    pub use_count: i64,
}
