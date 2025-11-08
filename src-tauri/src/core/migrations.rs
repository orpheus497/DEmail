use crate::error::DEmailError;
use rusqlite::Connection;
use tracing::{info, warn};

/// Gets the current schema version from the database
pub fn get_current_version(conn: &Connection) -> Result<i64, DEmailError> {
    // Check if migrations table exists
    let table_exists: bool = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='migrations'",
        [],
        |row| row.get::<_, i64>(0),
    )? > 0;

    if !table_exists {
        // No migrations table means version 0 (initial state)
        return Ok(0);
    }

    // Get the highest version number
    let version: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM migrations WHERE applied = 1",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    Ok(version)
}

/// Applies all pending migrations up to the current version
pub fn apply_migrations(conn: &Connection) -> Result<(), DEmailError> {
    let current_version = get_current_version(conn)?;
    info!("Current database schema version: {}", current_version);

    // Define all migrations
    let migrations: Vec<(i64, &str, fn(&Connection) -> Result<(), DEmailError>)> = vec![
        (1, "Initial schema", migration_v1_initial),
        (2, "Add starring and threading", migration_v2_starring_threading),
        (3, "Add contacts table", migration_v3_contacts),
    ];

    for (version, description, migration_fn) in migrations {
        if version > current_version {
            info!("Applying migration v{}: {}", version, description);
            migration_fn(conn)?;
            record_migration(conn, version, description)?;
            info!("Migration v{} applied successfully", version);
        }
    }

    Ok(())
}

/// Records that a migration was applied
fn record_migration(conn: &Connection, version: i64, description: &str) -> Result<(), DEmailError> {
    conn.execute(
        "INSERT INTO migrations (version, description, applied_at, applied) VALUES (?1, ?2, ?3, 1)",
        rusqlite::params![version, description, chrono::Utc::now().timestamp()],
    )?;
    Ok(())
}

/// Migration v1: Initial schema (for new installations)
/// This creates the base schema that existing installations already have
fn migration_v1_initial(conn: &Connection) -> Result<(), DEmailError> {
    // Create migrations table first
    conn.execute(
        "CREATE TABLE IF NOT EXISTS migrations (
            version INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            applied_at INTEGER NOT NULL,
            applied INTEGER NOT NULL DEFAULT 0
        )",
        [],
    )?;

    // Note: For existing databases, the tables already exist
    // This migration is primarily for documentation and new installations
    // The schema.rs initialize_schema() already creates these tables

    Ok(())
}

/// Migration v2: Add starring and threading support
fn migration_v2_starring_threading(conn: &Connection) -> Result<(), DEmailError> {
    // Add is_starred column to messages table
    // Use ALTER TABLE ADD COLUMN which is safe (won't fail if column exists in newer SQLite)
    match conn.execute(
        "ALTER TABLE messages ADD COLUMN is_starred INTEGER NOT NULL DEFAULT 0",
        [],
    ) {
        Ok(_) => info!("Added is_starred column to messages table"),
        Err(e) => {
            // Column might already exist
            warn!("Could not add is_starred column (may already exist): {}", e);
        }
    }

    // Add thread_id column to messages table
    match conn.execute(
        "ALTER TABLE messages ADD COLUMN thread_id INTEGER",
        [],
    ) {
        Ok(_) => info!("Added thread_id column to messages table"),
        Err(e) => {
            warn!("Could not add thread_id column (may already exist): {}", e);
        }
    }

    // Create threads table
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

    // Create index on thread lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_messages_thread_id ON messages(thread_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_threads_account ON threads(account_id)",
        [],
    )?;

    Ok(())
}

/// Migration v3: Add contacts table for autocomplete
fn migration_v3_contacts(conn: &Connection) -> Result<(), DEmailError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS contacts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL UNIQUE,
            name TEXT,
            last_used INTEGER NOT NULL,
            use_count INTEGER NOT NULL DEFAULT 1,
            UNIQUE(email)
        )",
        [],
    )?;

    // Create index for fast contact lookups
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

/// Rolls back the last applied migration (for development/testing)
/// WARNING: This can cause data loss!
pub fn rollback_last_migration(conn: &Connection) -> Result<(), DEmailError> {
    let current_version = get_current_version(conn)?;

    if current_version == 0 {
        return Err(DEmailError::Database(rusqlite::Error::InvalidQuery));
    }

    warn!("Rolling back migration from version {}", current_version);

    // Mark the migration as not applied
    conn.execute(
        "UPDATE migrations SET applied = 0 WHERE version = ?1",
        [current_version],
    )?;

    // Note: We don't actually drop tables/columns in rollback as SQLite doesn't support
    // DROP COLUMN easily. In production, this should be handled more carefully.

    warn!("Migration v{} marked as rolled back", current_version);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_get_current_version_empty_db() {
        let conn = Connection::open_in_memory().unwrap();
        let version = get_current_version(&conn).unwrap();
        assert_eq!(version, 0);
    }

    #[test]
    fn test_apply_migrations() {
        let conn = Connection::open_in_memory().unwrap();

        // Initialize base schema first
        crate::core::cache::schema::initialize_schema(&conn).unwrap();

        // Apply migrations
        apply_migrations(&conn).unwrap();

        let version = get_current_version(&conn).unwrap();
        assert!(version >= 1);
    }

    #[test]
    fn test_migration_idempotency() {
        let conn = Connection::open_in_memory().unwrap();

        crate::core::cache::schema::initialize_schema(&conn).unwrap();

        // Apply migrations twice - should not error
        apply_migrations(&conn).unwrap();
        apply_migrations(&conn).unwrap();

        let version = get_current_version(&conn).unwrap();
        assert!(version >= 1);
    }
}
