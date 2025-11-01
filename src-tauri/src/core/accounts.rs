use crate::error::DEmailError;
use crate::models::Account;
use rusqlite::{Connection, Result};

pub fn create_account(
    conn: &Connection,
    email_address: &str,
    display_name: &str,
    provider_type: &str,
) -> Result<Account, DEmailError> {
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

pub fn get_accounts(conn: &Connection) -> Result<Vec<Account>, DEmailError> {
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

pub fn delete_account(conn: &Connection, account_id: i64) -> Result<(), DEmailError> {
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
