mod ipc_server;
mod types;

use std::thread;

use rusqlite::{params, Connection, Result};

use crossbeam_channel::{bounded, unbounded};

use crate::types::cmd_event::CmdEvent;
use tracing::{debug, error, field::debug, info};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // limit the number of threads to avoid excessive resource usage especially to write to the database sqlite
    let (s, r) = bounded::<CmdEvent>(1);


    thread::spawn( move || {
        loop {
            let command = r.recv().unwrap(); //condvar
            debug!("Processing command: {:?} ...", command);
        }
    });

    // for writer avoid // ! (db locking)
    let conn = match Connection::open("winforge.db") {
        Ok(conn) => {
            if let Err(e) = conn.pragma_update(None, "journal_mode", "WAL") {
                tracing::error!("Failed to set journal_mode: {e}");
                return;
            }
            if let Err(e) = conn.pragma_update(None, "synchronous", "NORMAL") {
                tracing::error!("Failed to set synchronous: {e}");
                return;
            }
            if let Err(e) = conn.pragma_update(None, "busy_timeout", 5000) {
                tracing::error!("Failed to set busy_timeout: {e}");
                return;
            }
            conn
        },
        Err(error) => {
            tracing::error!("Failed to open database: {error}");
            return;
        }
    };

    
    if let Err(error) = ipc_server::run(&s).await {
        tracing::error!("IPC server stopped: {error}");
    }


}
