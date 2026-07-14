
#[cfg(windows)]
mod windows {
    use crate::worker_pool::TX;
    use std::sync::OnceLock;

    use tokio::{
        io::AsyncReadExt,
        net::windows::named_pipe::{NamedPipeServer, ServerOptions},
    };

    use tracing::{error, info};

    const PIPE_NAME: &str = r"\\.\pipe\winforge";
    const MAX_MESSAGE_SIZE: usize = 1024 * 1024;
    const READ_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);
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
            let pipe = ServerOptions::new().create(PIPE_NAME)?;

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
        let mut message = Vec::new();
        let mut buffer = [0u8; 4096];

        loop {
            let bytes_read = match tokio::time::timeout(READ_TIMEOUT, pipe.read(&mut buffer)).await
            {
                Ok(Ok(0)) => break,
                Ok(Ok(n)) => n,
                Ok(Err(e)) => {
                    error!("Read failed: {e}");
                    return;
                }
                Err(_) => {
                    error!("Read timed out");
                    return;
                }
            };

            if message.len() + bytes_read > MAX_MESSAGE_SIZE {
                error!("IPC message exceeds the {MAX_MESSAGE_SIZE} byte limit");
                return;
            }

            message.extend_from_slice(&buffer[..bytes_read]);
        }

        if message.is_empty() {
            return;
        }

        let message_cmd = match String::from_utf8(message) {
            Ok(message) => message,
            Err(e) => {
                error!("IPC message is not valid UTF-8: {e}");
                return;
            }
        };

        info!("IPC cmd received: {message_cmd}");
        TX.get().map(|tx| {
            match tx.try_send(message_cmd) {
                Ok(_) => {
                    info!("Sent message to worker pool");
                }
                Err(crossbeam_channel::TrySendError::Full(_)) => {
                    // worker pool is full
                    error!("Failed to send message to worker pool: full");
                }
                Err(crossbeam_channel::TrySendError::Disconnected(_)) => {
                    // worker pool has been dropped, this should not happen
                    error!("Failed to  to worker pool: disconnected");
                }
            }
        });

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
