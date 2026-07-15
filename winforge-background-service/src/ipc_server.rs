#[cfg(windows)]
mod platform {
    use std::any::Any; 

use crossbeam_channel::Sender;
use tokio::{
        io::AsyncReadExt,
        net::windows::named_pipe::{NamedPipeServer, ServerOptions},
    };
use tracing::{error, info, debug};

use crate::types::cmd_event::CmdEvent;

    const PIPE_NAME: &str = r"\\.\pipe\winforge";
    const MAX_MESSAGE_SIZE: usize = 1024 * 1024;
    const READ_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

    pub async fn run(sender:&Sender<CmdEvent>) -> std::io::Result<()> {
        info!("IPC server listening on {PIPE_NAME}");

        loop {
            let pipe = ServerOptions::new().create(PIPE_NAME)?;

            if let Err(error) = pipe.connect().await {
                error!("Connect failed: {error}");
                continue;
            }

            tokio::spawn(handle_client(pipe, sender.clone()));
        }
    }

    async fn handle_client(mut pipe: NamedPipeServer, sender: Sender<CmdEvent>) {
        let mut message = Vec::new();
        let mut buffer = [0_u8; 4096];

        loop {
            let bytes_read = match tokio::time::timeout(READ_TIMEOUT, pipe.read(&mut buffer)).await
            {
                Ok(Ok(0)) => break,
                Ok(Ok(bytes_read)) => bytes_read,
                Ok(Err(error)) => {
                    error!("Read failed: {error}");
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

        match String::from_utf8(message) {
            Ok(command) => {
                match serde_json::from_str::<CmdEvent>(&command) {
                    Ok(cmd) => {
                        debug!("IPC cmd received: {cmd:?}");
                        sender.send(cmd).unwrap();
                    }
                    Err(err) => {
                        error!("Invalid JSON: {err}");
                    }
                }
            },
            Err(error) => error!("IPC message is not valid UTF-8: {error}"),
        }
    }
}

#[cfg(not(windows))]
mod platform {
    pub async fn run() -> std::io::Result<()> {
        Ok(())
    }
}

pub use platform::run;
