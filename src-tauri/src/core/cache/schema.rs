use rusqlite::{Connection, Result};

/// Initializes the complete database schema for new installations
/// For existing databases, migrations will handle schema updates
pub fn initialize_schema(conn: &Connection) -> Result<()> {
    // Core tables
    create_accounts_table(conn)?;
    create_folders_table(conn)?;
    create_messages_table(conn)?;
    create_attachments_table(conn)?;
    create_message_flags_table(conn)?;

    // FTS5 virtual table and triggers
    create_fts_table(conn)?;
    create_fts_triggers(conn)?;

    // Feature tables
    create_drafts_table(conn)?;
    create_signatures_table(conn)?;
    create_settings_table(conn)?;
    create_attachment_data_table(conn)?;

    // New tables for enhanced features
    create_migrations_table(conn)?;
    create_threads_table(conn)?;
    create_contacts_table(conn)?;

    // Indexes
    create_indexes(conn)?;

    Ok(())
}

fn create_accounts_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email_address TEXT NOT NULL UNIQUE,
            display_name TEXT NOT NULL,
            provider_type TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

fn create_folders_table(conn: &Connection) -> Result<()> {
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
    Ok(())
}

fn create_messages_table(conn: &Connection) -> Result<()> {
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
            is_starred INTEGER NOT NULL DEFAULT 0,
            thread_id INTEGER,
            FOREIGN KEY (account_id) REFERENCES accounts (id) ON DELETE CASCADE,
            FOREIGN KEY (folder_id) REFERENCES folders (id) ON DELETE CASCADE,
            FOREIGN KEY (thread_id) REFERENCES threads (id) ON DELETE SET NULL,
            UNIQUE (account_id, folder_id, imap_uid)
        )",
        [],
    )?;
    Ok(())
}

fn create_attachments_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS attachments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            message_id INTEGER NOT NULL,
            filename TEXT NOT NULL,
            mime_type TEXT NOT NULL,
            size_bytes INTEGER NOT NULL,
            local_path TEXT,
            FOREIGN KEY (message_id) REFERENCES messages (id) ON DELETE CASCADE
        )",
        [],
    )?;
    Ok(())
}

fn create_message_flags_table(conn: &Connection) -> Result<()> {
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

fn create_fts_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS messages_fts USING fts5(
            subject,
            from_header,
            to_header,
            body_plain,
            content=messages,
            content_rowid=id
        )",
        [],
    )?;
    Ok(())
}

fn create_fts_triggers(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS messages_fts_insert AFTER INSERT ON messages BEGIN
            INSERT INTO messages_fts(rowid, subject, from_header, to_header, body_plain)
            VALUES (new.id, new.subject, new.from_header, new.to_header, new.body_plain);
        END",
        [],
    )?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS messages_fts_delete AFTER DELETE ON messages BEGIN
            DELETE FROM messages_fts WHERE rowid = old.id;
        END",
        [],
    )?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS messages_fts_update AFTER UPDATE ON messages BEGIN
            DELETE FROM messages_fts WHERE rowid = old.id;
            INSERT INTO messages_fts(rowid, subject, from_header, to_header, body_plain)
            VALUES (new.id, new.subject, new.from_header, new.to_header, new.body_plain);
        END",
        [],
    )?;

    Ok(())
}

fn create_drafts_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS drafts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            account_id INTEGER NOT NULL,
            to_addresses TEXT NOT NULL,
            cc_addresses TEXT,
            bcc_addresses TEXT,
            subject TEXT NOT NULL,
            body_plain TEXT,
            body_html TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (account_id) REFERENCES accounts (id) ON DELETE CASCADE
        )",
        [],
    )?;
    Ok(())
}

fn create_signatures_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS signatures (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            account_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            content_html TEXT NOT NULL,
            content_plain TEXT NOT NULL,
            is_default INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (account_id) REFERENCES accounts (id) ON DELETE CASCADE
        )",
        [],
    )?;
    Ok(())
}

fn create_settings_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

fn create_attachment_data_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS attachment_data (
            attachment_id INTEGER PRIMARY KEY,
            data BLOB NOT NULL,
            FOREIGN KEY (attachment_id) REFERENCES attachments (id) ON DELETE CASCADE
        )",
        [],
    )?;
    Ok(())
}

fn create_migrations_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS migrations (
            version INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            applied_at INTEGER NOT NULL,
            applied INTEGER NOT NULL DEFAULT 0
        )",
        [],
    )?;
    Ok(())
}

fn create_threads_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS threads (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            subject_hash TEXT NOT NULL,
            first_message_id INTEGER NOT NULL,
            last_message_id INTEGER NOT NULL,
            message_count INTEGER NOT NULL DEFAULT 1,
            account_id INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (first_message_id) REFERENCES messages (id) ON DELETE CASCADE,
            FOREIGN KEY (last_message_id) REFERENCES messages (id) ON DELETE CASCADE,
            FOREIGN KEY (account_id) REFERENCES accounts (id) ON DELETE CASCADE
        )",
        [],
    )?;
    Ok(())
}

fn create_contacts_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS contacts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL UNIQUE,
            name TEXT,
            last_used INTEGER NOT NULL,
            use_count INTEGER NOT NULL DEFAULT 1
        )",
        [],
    )?;
    Ok(())
}

fn create_indexes(conn: &Connection) -> Result<()> {
    // Message indexes
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_messages_date ON messages(date DESC)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_messages_folder ON messages(folder_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_messages_account ON messages(account_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_messages_thread_id ON messages(thread_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_messages_is_starred ON messages(is_starred)",
        [],
    )?;

    // Thread indexes
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_threads_account ON threads(account_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_threads_subject_hash ON threads(subject_hash)",
        [],
    )?;

    // Contact indexes
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_contacts_email ON contacts(email)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_contacts_last_used ON contacts(last_used DESC)",
        [],
    )?;

    Ok(())
}
