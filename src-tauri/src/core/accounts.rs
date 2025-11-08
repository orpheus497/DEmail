use crate::core::cache::db::Pool;
use crate::error::DEmailError;
use crate::models::Account;
use rusqlite::Result;

pub fn create_account(
    pool: &Pool,
    email_address: &str,
    display_name: &str,
    provider_type: &str,
) -> Result<Account, DEmailError> {
    let conn = pool
        .get()
        .map_err(|e| DEmailError::Database(rusqlite::Error::InvalidQuery))?;

    conn.execute(
        "INSERT INTO accounts (email_address, display_name, provider_type) VALUES (?1, ?2, ?3)",
        &[email_address, display_name, provider_type],
    )?;
    let id = conn.last_insert_rowid();
    Ok(Account {
        id,
        email_address: email_address.to_string(),
        display_name: display_name.to_string(),
        provider_type: provider_type.to_string(),
    })
}

pub fn get_accounts(pool: &Pool) -> Result<Vec<Account>, DEmailError> {
    let conn = pool
        .get()
        .map_err(|e| DEmailError::Database(rusqlite::Error::InvalidQuery))?;

    let mut stmt =
        conn.prepare("SELECT id, email_address, display_name, provider_type FROM accounts")?;
    let account_iter = stmt.query_map([], |row| {
        Ok(Account {
            id: row.get(0)?,
            email_address: row.get(1)?,
            display_name: row.get(2)?,
            provider_type: row.get(3)?,
        })
    })?;

    let mut accounts = Vec::new();
    for account in account_iter {
        accounts.push(account?);
    }
    Ok(accounts)
}

pub fn delete_account(pool: &Pool, account_id: i64) -> Result<(), DEmailError> {
    let conn = pool
        .get()
        .map_err(|e| DEmailError::Database(rusqlite::Error::InvalidQuery))?;

    conn.execute("DELETE FROM accounts WHERE id = ?1", &[&account_id])?;
    // The database schema is set up with ON DELETE CASCADE, so all related data will be deleted automatically.
    Ok(())
}

pub fn store_refresh_token(account_id: i64, refresh_token: &str) -> Result<(), DEmailError> {
    let entry = keyring::Entry::new("demail", &format!("account_{}", account_id));
    entry.set_password(refresh_token)?;
    Ok(())
}

pub fn get_refresh_token(account_id: i64) -> Result<String, DEmailError> {
    let entry = keyring::Entry::new("demail", &format!("account_{}", account_id));
    let refresh_token = entry.get_password()?;
    Ok(refresh_token)
}
