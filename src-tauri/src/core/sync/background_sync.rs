use crate::core::accounts::get_accounts;
use crate::core::auth::{get_google_client, get_microsoft_client};
use crate::core::config::load_config;
use crate::core::sync::imap_sync::ImapSync;
use crate::error::DEmailError;
use crate::AppState;

use std::time::Duration;
use tauri::{AppHandle, Manager};
use tokio::time;

pub fn start_background_sync(app_handle: AppHandle) {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(300)); // 5 minutes
        loop {
            interval.tick().await;
            if let Err(e) = sync_all_accounts(app_handle.clone()).await {
                log::error!("Error during background sync: {}", e);
            }
        }
    });
}

async fn sync_all_accounts(app_handle: AppHandle) -> Result<(), DEmailError> {
    let app_state = app_handle.state::<AppState>();
    let conn = app_state.db_conn.lock().unwrap();
    let accounts = get_accounts(&conn)?;
    let config = app_state.app_config.lock().unwrap();
    let configs = load_config(&config)?;

    for account in accounts {
        let provider = account.provider_type.as_str();
        let provider_config = configs.get(provider).ok_or_else(|| {
            DEmailError::Config(format!("OAuth config for {} not found", provider))
        })?;

        let client = match provider {
            "google" => get_google_client(
                provider_config.client_id.clone(),
                provider_config.client_secret.clone(),
            )?,
            "microsoft" => get_microsoft_client(
                provider_config.client_id.clone(),
                provider_config.client_secret.clone(),
            )?,
            _ => continue,
        };

        let imap_sync = ImapSync::new(app_state.clone());
        imap_sync
            .initial_sync(
                account.id,
                &account.provider_type,
                &client,
                &account.email_address,
            )
            .await?;
    }

    Ok(())
}
