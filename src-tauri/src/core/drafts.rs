use crate::core::cache::db::{delete_draft, get_drafts, save_draft, Pool};
use crate::error::DEmailError;
use crate::models::Draft;

pub fn create_or_update_draft(pool: &Pool, draft: &Draft) -> Result<i64, DEmailError> {
    save_draft(pool, draft)
}

pub fn load_drafts_for_account(pool: &Pool, account_id: i64) -> Result<Vec<Draft>, DEmailError> {
    get_drafts(pool, account_id)
}

pub fn remove_draft(pool: &Pool, draft_id: i64) -> Result<(), DEmailError> {
    delete_draft(pool, draft_id)
}

pub fn auto_save_draft(pool: &Pool, draft: &Draft) -> Result<i64, DEmailError> {
    tracing::debug!("Auto-saving draft for account {}", draft.account_id);
    save_draft(pool, draft)
}
