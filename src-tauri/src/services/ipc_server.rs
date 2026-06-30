#[cfg(windows)]
mod windows {
    use std::sync::OnceLock;

    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::windows::named_pipe::{NamedPipeServer, ServerOptions},
    };

    use tracing::{error, info};

    const PIPE_NAME: &str = r"\\.\pipe\winforge";
    static STARTED: OnceLock<()> = OnceLock::new();

    pub fn start() {
        if STARTED.set(()).is_err() {
            return;
        }

        tauri::async_runtime::spawn(async {
            loop {
                if let Err(e) = run().await {
                    error!("IPC server crashed: {e}");

                    // avoid infinite loop of crashing, wait a bit before restarting
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                    info!("Restarting IPC server...");
                }
            }
        });
    }

    async fn run() -> std::io::Result<()> {
        info!("IPC server listening on {PIPE_NAME}");

        loop {
            let mut pipe = ServerOptions::new().create(PIPE_NAME)?;

            if let Err(e) = pipe.connect().await {
                error!("Connect failed: {e}");
                continue;
            }

            tauri::async_runtime::spawn(async move {
                handle_client(pipe).await;
            });
        }
    }

    async fn handle_client(mut pipe: NamedPipeServer) {
        let mut buffer = [0u8; 4096];

        let bytes_read = match pipe.read(&mut buffer).await {
            Ok(0) => return,
            Ok(n) => n,
            Err(e) => {
                error!("Read failed: {e}");
                return;
            }
        };

        let message = String::from_utf8_lossy(&buffer[..bytes_read]);

        info!("IPC message received: {message}");

        // don't care it's a ping or a command, just respond with pong for now
        // if let Err(e) = pipe.write_all(b"pong").await {
        //     error!("Write failed: {e}");
        // }
    }
}

#[cfg(not(windows))]
mod windows {
    pub fn start() {}
}

pub use windows::start;