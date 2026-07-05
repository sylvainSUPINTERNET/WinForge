#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod command_payload_ipc;
use crate::command_payload_ipc::CommandPayloadIPC;


use std::time::Duration;
use tokio::{
    io::AsyncWriteExt,
    net::windows::named_pipe::ClientOptions,
    time::sleep,
};
use windows_sys::Win32::Foundation::*;
use std::env;
use tracing::debug;
use image::ImageFormat;
use std::path::Path;

use windows::{
    core::w,
    Win32::UI::WindowsAndMessaging::*,
    core::{
        HSTRING
    }

};
use lopdf::Document;


const PIPE_NAME: &str = r"\\.\pipe\winforge";
const PDF_TO_JPG_COMMAND: &str = "pdfToJpg";
const PDF_MERGE_COMMAND: &str = "pdfMerge";

fn verify_command(args: &Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let cmd_name = &args[1];
    let cmd_param = &args[2];

    let cmd_payload_ipc = CommandPayloadIPC {
        cmd_name: cmd_name.clone(),
        resource_path: cmd_param.clone(),
        password: None,
    };

    let cmd_payload_json_str = match serde_json::to_string(&cmd_payload_ipc) {
        Ok(s) => s,
        Err(e) => {
            debug!("Failed to serialize command payload to JSON: {}", e);
            return Err(e.into());
        }
    };
    let path = Path::new(cmd_param);

    match cmd_name.as_str() {
        PDF_TO_JPG_COMMAND | PDF_MERGE_COMMAND => {
            let doc = Document::load(path).unwrap();
            
            if doc.is_encrypted() {
                if doc.encryption_state.is_some() {
                    debug!("Encrypted but no password, automatically trying to decrypt with empty password");
                } else {
                    debug!("Encrypted and password required, cannot proceed");
                    unsafe {
                        // TODO send a box to see the name AND enter the password !
                        MessageBoxW(
                        None,
                        &HSTRING::from("This PDF is encrypted and requires a password."),
                        w!("WinForge"),
                        MB_OK | MB_ICONERROR);
                    }
                    return Err("This PDF is encrypted and requires a password.".into());
                }
            }
            
            return Ok(cmd_payload_json_str);
        },
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

    let cmd_to_execute: String = match verify_command(&args) {
        Ok(command_payload_ipc_serialized) => command_payload_ipc_serialized,
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
