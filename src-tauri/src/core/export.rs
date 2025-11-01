use crate::error::DEmailError;
use crate::models::{Account, Message};
use headless_chrome::Browser;
use rusqlite::Connection;
use std::fs;
use std::path::Path;

pub fn export_account(
    conn: &Connection,
    account: &Account,
    destination_path: &str,
) -> Result<(), DEmailError> {
    let account_path = Path::new(destination_path).join(&account.email_address);
    fs::create_dir_all(&account_path)?;

    let mut stmt = conn.prepare("SELECT * FROM messages WHERE account_id = ?1")?;
    let messages_iter = stmt.query_map([&account.id], |row| {
        Ok(Message {
            id: row.get(0)?,
            account_id: row.get(1)?,
            folder_id: row.get(2)?,
            imap_uid: row.get(3)?,
            message_id_header: row.get(4)?,
            from_header: row.get(6)?,
            to_header: row.get(7)?,
            cc_header: row.get(8)?,
            subject: row.get(9)?,
            date: row.get(10)?,
            body_plain: row.get(11)?,
            body_html: row.get(12)?,
            has_attachments: row.get(13)?,
            is_read: row.get(14)?,
            attachments: Vec::new(), // Will be loaded separately
        })
    })?;

    for message in messages_iter {
        let message = message?;
        export_message(&message, &account_path)?;
    }

    Ok(())
}

fn export_message(message: &Message, account_path: &Path) -> Result<(), DEmailError> {
    let date = chrono::NaiveDateTime::from_timestamp(message.date, 0);
    let year = date.format("%Y").to_string();
    let month = date.format("%m-%B").to_string();
    let day = date.format("%d").to_string();

    let message_folder_name = format!(
        "{}",
        message.subject.replace(|c: char| !c.is_alphanumeric(), "_")
    );

    let message_path = account_path
        .join(year)
        .join(month)
        .join(day)
        .join(message_folder_name);
    fs::create_dir_all(&message_path)?;

    if let Some(body_plain) = &message.body_plain {
        fs::write(message_path.join("email.txt"), body_plain)?;
    }

    if let Some(body_html) = &message.body_html {
        fs::write(message_path.join("email.html"), body_html)?;
        export_as_pdf(body_html, &message_path.join("email.pdf"))?;
    }

    Ok(())
}

fn export_as_pdf(html: &str, path: &Path) -> Result<(), DEmailError> {
    let browser = Browser::default().map_err(|e| DEmailError::Pdf(e.to_string()))?;
    let tab = browser
        .new_tab()
        .map_err(|e| DEmailError::Pdf(e.to_string()))?;

    let pdf = tab
        .navigate_to(&format!("data:text/html,{}", html))
        .map_err(|e| DEmailError::Pdf(e.to_string()))?
        .wait_for_element("body")
        .map_err(|e| DEmailError::Pdf(e.to_string()))?
        .print_to_pdf(None)
        .map_err(|e| DEmailError::Pdf(e.to_string()))?;

    fs::write(path, pdf)?;

    Ok(())
}
