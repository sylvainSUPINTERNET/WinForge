mod ipc_server;
mod types;

use std::{any::Any, path::Path, sync::mpsc, thread};

use rusqlite::{params, Connection, Result};

use crossbeam_channel::{bounded, unbounded};

use crate::types::cmd_event::CmdEvent;
use tracing::{debug, error, field::debug, info};

use r2d2;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::CustomizeConnection;

use notify::{Event as WatcherEvent, RecursiveMode, Result as WatcherResult, Watcher};



#[derive(Debug)]
struct SqliteInitializer;

impl CustomizeConnection<Connection, rusqlite::Error> for SqliteInitializer {
    fn on_acquire(&self, conn: &mut Connection) -> Result<(), rusqlite::Error> {
        conn.execute_batch("
            PRAGMA journal_mode=WAL;
            PRAGMA foreign_keys=ON;
            PRAGMA busy_timeout=5000;
        ")?;
        Ok(())
    }
}

fn init_db(conn: &Connection) -> Result<()> {

    conn.execute(
        "CREATE TABLE IF NOT EXISTS folders (
            id TEXT PRIMARY KEY,
            resource_path TEXT NOT NULL,
            prompt TEXT
        )",
        [],
    )?;

    debug!("Database initialized successfully");
    Ok(())
}


#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();


    // limit the number of threads to avoid excessive resource usage especially to write to the database sqlite
    let (s, r) = bounded::<CmdEvent>(1);
    let (tx, rx) = mpsc::channel::<WatcherResult<WatcherEvent>>();
        
    let mut watcher = match notify::recommended_watcher(tx) {
        Ok(w) => w,
        Err(e) => {
            error!("Failed to create watcher: {e}");
            return;
        }
    };


    // TODO => bouger la DB pour préparer la distribution dans app local
    // TODO => init on doit lister les folder 
    // TODO => mais aussi rajouter quand on reçoit un event nouveau

    // TODO test
    let p1 = Path::new("C:\\Workspace\\perso\\WinForge\\winforge-background-service\\watch1");
    let p2 = Path::new("C:\\Workspace\\perso\\WinForge\\winforge-background-service\\watch2");

    watcher.watch(p1, RecursiveMode::Recursive).expect(&format!("Failed to watch path {}", p1.to_str().unwrap()));
    watcher.watch(p2, RecursiveMode::Recursive).expect(&format!("Failed to watch path {}", p2.to_str().unwrap()));


    thread::spawn( move || {
        debug!("Starting filesystem watcher thread - (sleeping and waiting for new events)");

        for w_ev in rx { // blocking waiting for msg
            match w_ev {
                Ok(event) => {

                    if event.kind.is_create() {
                        debug!("Received filesystem event: {:?}", event);
                    }

                }
                Err(e) => {
                    error!("Watch error: {e}");
                }
            }
        }
    });




    // Init pool for sqlite connections
    let manager = SqliteConnectionManager::file("winforge.db");
    let pool: r2d2::Pool<SqliteConnectionManager> = r2d2::Pool::builder()
        .connection_customizer(Box::new(SqliteInitializer))
        .max_size(15) // Limit the number of connections in the pool
        .build(manager)
        .expect("Failed to create connection pool");

    // Initialize the database schema
    if let Err(e) = init_db(&pool.get().expect("Failed to get connection from pool")) {
        tracing::error!("Failed to initialize database: {e}");
        return;
    }


    // Start the IPC server "listener" in a separate thread
    // Grab 1 connection from the pool to use in the thread when a new command_event is received from the IPC server and the event is send to the channel correctly.
    thread::spawn( move || {
        debug!("Starting IPC server listener thread - (sleeping and waiting for new channel messages)");
        loop {
            let command_event: CmdEvent = r.recv().unwrap(); //condvar (wakeup only if new message)
            debug!("  > Processing command_event: {:?}", command_event);

            let conn: r2d2::PooledConnection<SqliteConnectionManager> = pool.get().expect("Failed to get connection from pool");
            let id_cmd = command_event.get_id();
            let prompt: Option<String> = None; // NULL 

            match conn.execute(
                "INSERT INTO folders (id, resource_path, prompt) VALUES (?1, ?2, ?3) ON CONFLICT(id) DO UPDATE SET prompt=excluded.prompt",
                params![id_cmd, command_event.resource_path, prompt]) {
                    Ok(_) => {
                        debug!("Successfully executed SQL: {}", "INSERT INTO folders (id, resource_path, prompt) VALUES (?1, ?2, ?3) ON CONFLICT(id) DO UPDATE SET prompt=excluded.prompt");
                    },
                    Err(error) => {
                        error!("Failed to insert or update command_event in database: {error}");
                    }
                }

        }
    });




    
    if let Err(error) = ipc_server::run(&s).await {
        tracing::error!("IPC server stopped: {error}");
    }


}
