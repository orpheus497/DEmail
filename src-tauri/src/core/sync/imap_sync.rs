use crate::core::accounts::get_refresh_token;
use crate::core::cache::db::{save_folder, save_message, save_attachment, save_attachment_data, Pool};
use crate::core::contacts;
use crate::core::threading;
use crate::error::DEmailError;
use crate::models::{Attachment, Folder, Message};
use imap::{
    types::{Fetch, Uid},
    Client, Session,
};
use mail_parser::Message as ParsedMessage;
use native_tls::TlsStream;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AccessToken, RefreshToken, TokenResponse,
};
use std::net::TcpStream;
use std::sync::Arc;

use crate::AppState;

pub struct ImapSync {
    app_state: Arc<AppState>,
}

impl ImapSync {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }

    pub async fn initial_sync(
        &self,
        account_id: i64,
        provider: &str,
        client: &BasicClient,
        user_email: &str,
    ) -> Result<(), DEmailError> {
        let refresh_token = get_refresh_token(account_id)?;

        let token_result = client
            .exchange_refresh_token(&RefreshToken::new(refresh_token))
            .request_async(async_http_client)
            .await
            .map_err(|e| DEmailError::OAuth(e.to_string()))?;

        let access_token = token_result.access_token();

        let domain = match provider {
            "google" => "imap.gmail.com",
            "microsoft" => "outlook.office365.com",
            _ => {
                return Err(DEmailError::Imap(imap::Error::Parse(
                    "Unsupported provider",
                )))
            }
        };

        let mut imap_session = self.connect(domain, user_email, access_token).await?;

        let folders = self.fetch_folders(&mut imap_session, account_id).await?;

        for folder in folders {
            self.sync_folder(&mut imap_session, account_id, &folder)
                .await?;
        }

        imap_session.logout()?;

        Ok(())
    }

    async fn connect(
        &self,
        domain: &str,
        user_email: &str,
        access_token: &AccessToken,
    ) -> Result<Session<TlsStream<TcpStream>>, DEmailError> {
        let tls = native_tls::TlsConnector::builder().build()?;
        let client = imap::connect((domain, 993), domain, &tls)?;
        let session = client
            .login_with_oauth2(user_email, access_token.secret())
            .map_err(|(e, _)| e)?;
        Ok(session)
    }

    async fn fetch_folders(
        &self,
        session: &mut Session<TlsStream<TcpStream>>,
        account_id: i64,
    ) -> Result<Vec<Folder>, DEmailError> {
        let mailboxes = session.list(Some(""), Some("*"))?;
        let pool = &self.app_state.db_pool;
        let mut folders = Vec::new();
        for mailbox in mailboxes.iter() {
            let mut folder = Folder {
                id: 0, // Will be set by the database
                account_id,
                name: mailbox.name().to_string(),
                path: mailbox.name().to_string(),
                parent_id: None,
            };
            save_folder(pool, &mut folder)?;
            folders.push(folder);
        }
        Ok(folders)
    }

    async fn sync_folder(
        &self,
        session: &mut Session<TlsStream<TcpStream>>,
        account_id: i64,
        folder: &Folder,
    ) -> Result<(), DEmailError> {
        session.select(&folder.path)?;
        let uids: Vec<Uid> = session.uid_search("ALL")?.into_iter().collect();
        if uids.is_empty() {
            return Ok(());
        }

        let seq_set = uids
            .iter()
            .map(|uid| uid.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let fetch = session.uid_fetch(seq_set, "(RFC822)")?;
        let pool = &self.app_state.db_pool;
        let conn = pool.get().map_err(|_| {
            DEmailError::Database(rusqlite::Error::InvalidQuery)
        })?;

        for msg in fetch.iter() {
            if let Some(body) = msg.body() {
                if let Ok(parsed_message) = ParsedMessage::parse(body) {
                    let from_str = parsed_message
                        .from()
                        .map(|f| f.to_string())
                        .unwrap_or_default();
                    let to_str = parsed_message
                        .to()
                        .map(|t| t.to_string())
                        .unwrap_or_default();
                    let cc_str = parsed_message
                        .cc()
                        .map(|c| c.to_string());

                    let mut message = Message {
                        id: 0,
                        account_id,
                        folder_id: folder.id,
                        imap_uid: msg.uid.unwrap_or(0),
                        message_id_header: parsed_message
                            .message_id()
                            .unwrap_or_default()
                            .to_string(),
                        from_header: from_str.clone(),
                        to_header: to_str.clone(),
                        cc_header: cc_str.clone(),
                        subject: parsed_message.subject().unwrap_or_default().to_string(),
                        date: parsed_message.date().map_or(0, |d| d.timestamp()),
                        body_plain: parsed_message.body_text(0).map(|s| s.to_string()),
                        body_html: parsed_message.body_html(0).map(|s| s.to_string()),
                        has_attachments: !parsed_message.attachments().is_empty(),
                        is_read: false,
                        is_starred: false,
                        thread_id: None,
                        attachments: Vec::new(),
                    };
                    save_message(pool, &message)?;

                    let message_id = conn.last_insert_rowid();
                    message.id = message_id;

                    // Phase 3: Threading integration (stub call for Phase 2)
                    if let Ok(_thread_id) = threading::create_or_update_thread(&conn, &message) {
                        // Thread created/updated successfully
                    }

                    // Phase 3: Contacts extraction (stub call for Phase 2)
                    let _ = contacts::extract_and_save_contacts(
                        &conn,
                        &from_str,
                        &to_str,
                        cc_str.as_deref(),
                    );

                    for attachment in parsed_message.attachments() {
                        let filename = attachment
                            .attachment_name()
                            .unwrap_or("unnamed_attachment")
                            .to_string();
                        let mime_type = attachment
                            .content_type()
                            .and_then(|ct| ct.ctype())
                            .map(|t| t.to_string())
                            .unwrap_or_else(|| "application/octet-stream".to_string());
                        let content = attachment.contents();
                        let size_bytes = content.len() as i64;

                        let mut att = Attachment {
                            id: 0,
                            message_id,
                            filename: filename.clone(),
                            mime_type,
                            size_bytes,
                            local_path: None,
                        };

                        save_attachment(pool, &att)?;
                        let attachment_id = conn.last_insert_rowid();
                        att.id = attachment_id;

                        save_attachment_data(pool, attachment_id, content)?;
                    }
                }
            }
        }

        Ok(())
    }
}

pub async fn sync_account_manually(
    app_state: Arc<AppState>,
    account_id: i64,
    provider: &str,
    client: &BasicClient,
    user_email: &str,
) -> Result<(), DEmailError> {
    let syncer = ImapSync::new(app_state);
    syncer
        .initial_sync(account_id, provider, client, user_email)
        .await
}
