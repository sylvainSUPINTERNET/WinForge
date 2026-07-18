use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use tracing::{debug};

// update port found for the UI discovery to find the background service
fn write_port_file(port: u16) {
    let path = dirs::data_local_dir().unwrap().join("WinForge").join("runtime-port.json");
    let pid = std::process::id();
    std::fs::create_dir_all(&path.parent().unwrap()).unwrap();
    let content = serde_json::json!({ "port": port, "pid": pid });
    std::fs::write(&path, content.to_string()).unwrap();

    debug!("Write discovery info PORT: {:?} - PID: {:?} - to conf path: {:?}", port, pid, &path.as_path());
}


pub type DbPool = Pool<SqliteConnectionManager>;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub port: u16,
}

impl AppState {
    pub fn new(pool: DbPool, port: u16) -> Self {
        write_port_file(port);
        Self { pool, port }
    }
}
