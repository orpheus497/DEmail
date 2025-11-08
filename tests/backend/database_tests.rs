#[cfg(test)]
mod database_tests {
    use rusqlite::Connection;

    #[test]
    fn test_database_creation() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS accounts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email_address TEXT NOT NULL UNIQUE,
                display_name TEXT NOT NULL,
                provider_type TEXT NOT NULL
            )",
            [],
        ).unwrap();

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM accounts", [], |row| row.get(0)).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_account_insertion() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE accounts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email_address TEXT NOT NULL UNIQUE,
                display_name TEXT NOT NULL,
                provider_type TEXT NOT NULL
            )",
            [],
        ).unwrap();

        conn.execute(
            "INSERT INTO accounts (email_address, display_name, provider_type) VALUES (?1, ?2, ?3)",
            &["test@example.com", "Test User", "google"],
        ).unwrap();

        let email: String = conn.query_row(
            "SELECT email_address FROM accounts WHERE id = 1",
            [],
            |row| row.get(0)
        ).unwrap();

        assert_eq!(email, "test@example.com");
    }

    #[test]
    fn test_draft_operations() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE drafts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                account_id INTEGER NOT NULL,
                to_addresses TEXT NOT NULL,
                cc_addresses TEXT,
                bcc_addresses TEXT,
                subject TEXT NOT NULL,
                body_plain TEXT,
                body_html TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        ).unwrap();

        let now = 1234567890i64;
        conn.execute(
            "INSERT INTO drafts (account_id, to_addresses, subject, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            &[&1i64, &"test@example.com", &"Test Subject", &now, &now],
        ).unwrap();

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM drafts", [], |row| row.get(0)).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_signature_operations() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE signatures (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                account_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                content_html TEXT NOT NULL,
                content_plain TEXT NOT NULL,
                is_default INTEGER NOT NULL DEFAULT 0
            )",
            [],
        ).unwrap();

        conn.execute(
            "INSERT INTO signatures (account_id, name, content_html, content_plain, is_default)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            &[&1i64, &"Professional", &"<p>Regards</p>", &"Regards", &1i64],
        ).unwrap();

        let name: String = conn.query_row(
            "SELECT name FROM signatures WHERE is_default = 1",
            [],
            |row| row.get(0)
        ).unwrap();

        assert_eq!(name, "Professional");
    }

    #[test]
    fn test_fts5_virtual_table() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                subject TEXT,
                from_header TEXT,
                to_header TEXT,
                body_plain TEXT
            )",
            [],
        ).unwrap();

        conn.execute(
            "CREATE VIRTUAL TABLE messages_fts USING fts5(
                subject,
                from_header,
                to_header,
                body_plain,
                content=messages,
                content_rowid=id
            )",
            [],
        ).unwrap();

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM messages_fts",
            [],
            |row| row.get(0)
        ).unwrap();

        assert_eq!(count, 0);
    }

    #[test]
    fn test_pagination_query() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                folder_id INTEGER NOT NULL,
                subject TEXT,
                date INTEGER NOT NULL
            )",
            [],
        ).unwrap();

        for i in 1..=100 {
            conn.execute(
                "INSERT INTO messages (folder_id, subject, date) VALUES (?1, ?2, ?3)",
                &[&1i64, &format!("Message {}", i), &(i as i64)],
            ).unwrap();
        }

        let mut stmt = conn.prepare(
            "SELECT COUNT(*) FROM messages WHERE folder_id = ?1 LIMIT ?2 OFFSET ?3"
        ).unwrap();

        let count: i64 = stmt.query_row(&[&1i64, &50i64, &0i64], |row| row.get(0)).unwrap();
        assert!(count <= 50);
    }
}
