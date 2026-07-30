use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use tracing::{debug, error, field::debug, info};

pub fn remove_folder(
    pool: &Pool<SqliteConnectionManager>,
    paths: &Vec<String>
) {
    let mut conn = pool.get().expect("Failed to get a connection from the pool");
    let tx = conn.transaction().expect("Failed to start a transaction");

    for path in paths {
        match tx.execute("DELETE FROM folders WHERE resource_path = ?1", params![path]) {
            Ok(_) => {
                debug!("Successfully removed folder with resource_path {} from the database", path);
            },
            Err(e) => {
                error!("Failed to remove folder with resource_path {} from the database: {e}", path);
            }
        }
    }

    match tx.commit() {
        Ok(_) => {
            debug!("Transaction committed successfully");
        },
        Err(e) => {
            error!("Failed to commit transaction: {e}");
        }
    }
}