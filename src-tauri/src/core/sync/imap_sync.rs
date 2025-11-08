use crate::core::accounts::get_refresh_token;
use crate::core::cache::db::{save_folder, save_message};
use crate::error::DEmailError;
use crate::models::{Folder, Message};
use imap::{
    types::{Fetch, Uid},
    Client, Session,
};
use mail_parser::Message as ParsedMessage;
use native_tls::TlsStream;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AccessToken, RefreshToken, TokenResponse,
};
use rusqlite::Connection;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

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

        let (domain, user_email) = match provider {
            "google" => ("imap.gmail.com", ""), // User email will be used
            "microsoft" => ("outlook.office365.com", ""),
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
        let conn = self.app_state.db_conn.lock().unwrap();
        let mut folders = Vec::new();
        for mailbox in mailboxes.iter() {
            let mut folder = Folder {
                id: 0, // Will be set by the database
                account_id,
                name: mailbox.name().to_string(),
                path: mailbox.name().to_string(),
                parent_id: None,
            };
            save_folder(&conn, &mut folder)?;
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
        let conn = self.app_state.db_conn.lock().unwrap();

        for msg in fetch.iter() {
            if let Some(body) = msg.body() {
                if let Ok(parsed_message) = ParsedMessage::parse(body) {
                    let message = Message {
                        id: 0,
                        account_id,
                        folder_id: folder.id,
                        imap_uid: msg.uid.unwrap_or(0),
                        message_id_header: parsed_message
                            .message_id()
                            .unwrap_or_default()
                            .to_string(),
                        from_header: parsed_message
                            .from()
                            .map(|f| f.to_string())
                            .unwrap_or_default(),
                        to_header: parsed_message
                            .to()
                            .map(|t| t.to_string())
                            .unwrap_or_default(),
                        cc_header: None,
                        subject: parsed_message.subject().unwrap_or_default().to_string(),
                        date: parsed_message.date().map_or(0, |d| d.timestamp()),
                        body_plain: parsed_message.body_text(0).map(|s| s.to_string()),
                        body_html: parsed_message.body_html(0).map(|s| s.to_string()),
                        has_attachments: !parsed_message.attachments().is_empty(),
                        is_read: false, // Will be updated later
                        attachments: Vec::new(),
                    };
                    save_message(&conn, &message)?;
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
