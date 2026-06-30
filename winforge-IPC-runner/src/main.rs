use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::windows::named_pipe::ClientOptions,
    time::sleep,
};
use windows_sys::Win32::Foundation::ERROR_PIPE_BUSY;
use std::env;
use tracing::{debug, field::debug};
use image::ImageFormat;
use std::path::Path;

use windows::{
    core::w,
    Win32::UI::WindowsAndMessaging::{
        MessageBoxW,
        MB_ICONERROR,
        MB_ICONINFORMATION,
        MB_OK,
    },
};

const PIPE_NAME: &str = r"\\.\pipe\winforge";

fn verify_command(args: &Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let cmd_name = &args[1];
    let cmd_param = &args[2];

    match cmd_name.as_str() {
        "imagePngToJpeg" => {
            let path = Path::new(cmd_param);

            match image::ImageFormat::from_path(path) {
                Ok(ImageFormat::Png) => Ok(cmd_name.to_string()),
                Ok(_) => {
                    debug!("File is not a PNG: {:?}", cmd_param);
                        unsafe {
                            MessageBoxW(
                            None,
                            w!("This file is not a PNG."),
                            w!("WinForge"),
                            MB_OK | MB_ICONERROR);
                        }
                    return Err("File is not a PNG".into());
                },
                Err(e) => {
                    debug!("Error determining image format: {:?}", e);
                        unsafe {
                            MessageBoxW(
                            None,
                            w!("Error determining image format, PNG expected."),
                            w!("WinForge"),
                            MB_OK | MB_ICONERROR);
                        }
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

    let cmd_to_execute = match verify_command(&args) {
        Ok(r) => r,
        Err(e) => {
            debug!("Command verification failed: {}", e);
            return Ok(());
        }
    };

    debug!("Command verified successfully: {:?}", args);


    debug!("Attempting to connect to named pipe: {}", PIPE_NAME);
    let mut client = loop {
        match ClientOptions::new().open(PIPE_NAME) {
            Ok(pipe) => {
                debug!("Connected to named pipe: {}", PIPE_NAME);
                break pipe;
            },

            Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY as i32) => {
                debug!("Named pipe is busy, retrying in 100ms...");
                sleep(Duration::from_millis(100)).await;
            }

            Err(e) => {
                debug!("Failed to connect to named pipe: {}", e);
                return Err(e);
            },
        }
    };


    debug!("Client IPC write command: {:?}", cmd_to_execute);
    client.write_all(cmd_to_execute.as_bytes()).await?;


    // blocking code to read the response from the server 
    // else on aura (os error 232)
    // let mut response = Vec::new();
    // client.read_to_end(&mut response).await?;

    // println!("{}", String::from_utf8_lossy(&response));

    Ok(())

}