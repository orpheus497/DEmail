use rusqlite::{Connection, Result};

pub fn initialize_schema(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email_address TEXT NOT NULL UNIQUE,
            display_name TEXT NOT NULL,
            provider_type TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS folders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            account_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            parent_id INTEGER,
            uid_validity INTEGER,
            FOREIGN KEY (account_id) REFERENCES accounts (id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            account_id INTEGER NOT NULL,
            folder_id INTEGER NOT NULL,
            imap_uid INTEGER NOT NULL,
            message_id_header TEXT,
            in_reply_to_header TEXT,
            from_header TEXT NOT NULL,
            to_header TEXT NOT NULL,
            cc_header TEXT,
            subject TEXT,
            date INTEGER NOT NULL,
            body_plain TEXT,
            body_html TEXT,
            has_attachments INTEGER NOT NULL DEFAULT 0,
            is_read INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (account_id) REFERENCES accounts (id) ON DELETE CASCADE,
            FOREIGN KEY (folder_id) REFERENCES folders (id) ON DELETE CASCADE,
            UNIQUE (account_id, folder_id, imap_uid)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS attachments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            message_id INTEGER NOT NULL,
            filename TEXT NOT NULL,
            mime_type TEXT NOT NULL,
            size_bytes INTEGER NOT NULL,
            local_path TEXT NOT NULL,
            FOREIGN KEY (message_id) REFERENCES messages (id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS message_flags (
            message_id INTEGER NOT NULL,
            flag TEXT NOT NULL,
            PRIMARY KEY (message_id, flag),
            FOREIGN KEY (message_id) REFERENCES messages (id) ON DELETE CASCADE
        )",
        [],
    )?;

    Ok(())
}
