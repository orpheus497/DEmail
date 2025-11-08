use crate::core::cache::db::{delete_draft, get_drafts, save_draft};
use crate::error::DEmailError;
use crate::models::Draft;
use rusqlite::Connection;

pub fn create_or_update_draft(
    conn: &Connection,
    draft: &Draft,
) -> Result<i64, DEmailError> {
    save_draft(conn, draft)
}

pub fn load_drafts_for_account(
    conn: &Connection,
    account_id: i64,
) -> Result<Vec<Draft>, DEmailError> {
    get_drafts(conn, account_id)
}

pub fn remove_draft(conn: &Connection, draft_id: i64) -> Result<(), DEmailError> {
    delete_draft(conn, draft_id)
}

pub fn auto_save_draft(
    conn: &Connection,
    draft: &Draft,
) -> Result<i64, DEmailError> {
    log::debug!("Auto-saving draft for account {}", draft.account_id);
    save_draft(conn, draft)
}
