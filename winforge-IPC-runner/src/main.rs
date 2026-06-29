use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::windows::named_pipe::ClientOptions,
    time::sleep,
};
use windows_sys::Win32::Foundation::ERROR_PIPE_BUSY;
use std::env;
use tracing::{debug};

const PIPE_NAME: &str = r"\\.\pipe\winforge";

fn test(args: Vec<String>) {
    debug!("Args: {:?}", args);

    let cmdName = &args[1];
    let cmdParamStr = &args[2];

    match cmdName.as_str() {
        "imagePngToJpeg" => {
            debug!("Cmd : {:?}", cmdName);
        },
        _ => {
            debug!("Unknown command: {:?}", cmdName);
        }
    }
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    // "imagePngToJpeg", "D:\\images\\2025-04-12 18-39-38.jpeg"
    let args: Vec<String> = env::args().collect();

    if args.len() != 3  {
        debug!("Usage: winforge-IPC-runner <command> [<file>]");
        return Ok(());
    }

    test(args);

    // let mut client = loop {
    //     match ClientOptions::new().open(PIPE_NAME) {
    //         Ok(pipe) => break pipe,

    //         Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY as i32) => {
    //             sleep(Duration::from_millis(100)).await;
    //         }

    //         Err(e) => return Err(e),
    //     }
    // };

    // client.write_all(b"ping").await?;

    // let mut response = Vec::new();
    // client.read_to_end(&mut response).await?;

    // println!("{}", String::from_utf8_lossy(&response));

    Ok(())
}