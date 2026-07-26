use tracing::{debug};

use rusqlite::{params, Connection, Result};

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS folders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uid TEXT NOT NULL UNIQUE,
            resource_path TEXT NOT NULL,
            prompt TEXT,
            created_at TEXT NOT NULL DEFAULT (
                STRFTIME('%Y-%m-%dT%H:%M:%fZ', 'now')
            )
        );

        CREATE TABLE IF NOT EXISTS jobs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uid TEXT NOT NULL UNIQUE,
            folder_id INTEGER NOT NULL,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (
                STRFTIME('%Y-%m-%dT%H:%M:%fZ', 'now')
            ),
            FOREIGN KEY (folder_id)
                REFERENCES folders(id)
                ON DELETE CASCADE
        );
        ",
    )?;

    debug!("Database initialized successfully");
    
    Ok(())
}