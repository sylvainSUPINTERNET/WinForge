use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::windows::named_pipe::ClientOptions,
    time::sleep,
};
use windows_sys::Win32::Foundation::ERROR_PIPE_BUSY;
use std::env;
use tracing::{debug};
use image::ImageFormat;
use std::path::Path;

const PIPE_NAME: &str = r"\\.\pipe\winforge";


// server : 
// https://chatgpt.com/c/6a42f15a-ceb0-83ed-b7ec-ae852cc42a4d


fn verify_command(args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let cmd_name = &args[1];
    let cmd_param = &args[2];

    match cmd_name.as_str() {
        "imagePngToJpeg" => {
            let path = Path::new(cmd_param);

            match image::ImageFormat::from_path(path) {
                Ok(ImageFormat::Png) => Ok(()),
                Ok(_) => {
                    debug!("File is not a PNG: {:?}", cmd_param);
                    return Err("File is not a PNG".into());
                },
                Err(e) => {
                    debug!("Error determining image format: {:?}", e);
                    return Err(e.into());
                },
            }
        }

        _ => Err("Unknown command".into()),
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

    let result = match verify_command(&args) {
        Ok(r) => r,
        Err(e) => {
            debug!("Command verification failed: {}", e);
            return Ok(());
        }
    };
    debug!("Command verified successfully: {:?}", args);


    let mut client = loop {
        match ClientOptions::new().open(PIPE_NAME) {
            Ok(pipe) => {
                debug!("Connected to named pipe: {}", PIPE_NAME);
                break pipe;
            },

            Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY as i32) => {
                sleep(Duration::from_millis(100)).await;
            }

            Err(e) => return Err(e),
        }
    };

    client.write_all(b"ping").await?;

    let mut response = Vec::new();
    client.read_to_end(&mut response).await?;

    println!("{}", String::from_utf8_lossy(&response));

    Ok(())
}