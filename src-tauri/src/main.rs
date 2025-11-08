#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod api;
mod core;
mod error;
mod models;

use crate::core::auth::{AppState, OAuth2StateMap};
use crate::core::cache::db;
use crate::core::sync::background_sync;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{Manager, RunEvent};

fn main() {
    // Initialize logging
    env_logger::init();

    let context = tauri::generate_context!();

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            let app_config = app.config();
            let db_conn =
                db::initialize_database(&app_config).expect("Database initialization failed");

            app.manage(AppState {
                db_conn: Arc::new(std::sync::Mutex::new(db_conn)),
                app_config: Arc::new(std::sync::Mutex::new(app_config.clone())),
            });

            app.manage(OAuth2StateMap(std::sync::Mutex::new(HashMap::new())));

            background_sync::start_background_sync(handle.clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            api::add_account,
            api::handle_callback,
            api::get_accounts,
            api::delete_account,
            api::get_folders,
            api::get_messages,
            api::get_message_details,
            api::send_email,
            api::start_export,
            api::save_oauth_provider_config,
            api::get_oauth_provider_configs,
            api::mark_message_read,
            api::mark_message_unread,
            api::refresh_account,
            api::search_messages,
            api::save_draft,
            api::get_drafts,
            api::delete_draft,
            api::save_signature,
            api::get_signatures,
            api::delete_signature,
            api::download_attachment,
            api::get_messages_paginated,
            api::count_messages_in_folder,
            api::delete_message,
            api::move_message,
            api::save_setting,
            api::get_setting,
            api::get_all_settings,
            api::prepare_reply,
            api::prepare_forward,
        ])
        .build(context)
        .expect("error while building tauri application")
        .run(|_app_handle, event| {
            if let RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });
}
