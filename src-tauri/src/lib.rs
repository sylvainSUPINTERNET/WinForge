mod services;
mod worker_pool;
mod deserializers;
mod commands;
mod jobs;

use std::{fs, path::PathBuf, process::Command, sync::Arc};

use crate::worker_pool::TX;

use crossbeam_channel::{Sender, Receiver};
use tracing::{debug, info, error};
use crate::commands::command_manager;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

use windows::{
    core::{w, HSTRING},
    Win32::UI::WindowsAndMessaging::{
        MessageBoxW, MB_ICONERROR, MB_OK,
    },
};
//const CHANNEL_CAP: usize = 1000;


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    // menu::menu::menu_select_action(std::env::args().collect());
    // let args: Vec<String> = std::env::args().collect();
    // image_converter::convert_png_to_jpeg(args);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn discover_port_winforge_background_service() -> serde_json::Value {
    let Some(data_local_dir) = dirs::data_local_dir() else {
        let message = "Unable to resolve the local application data directory";
        error!(message);
        return serde_json::json!({
            "code": 500,
            "message": { "error": message }
        });
    };

    let path = data_local_dir.join("WinForge").join("runtime-port.json");
    let contents = match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(err) => {
            let message = format!("Failed to read {}: {}", path.display(), err);
            error!("{}", message);
            return serde_json::json!({
                "code": 500,
                "message": { "error": message }
            });
        }
    };

    match serde_json::from_str::<serde_json::Value>(&contents) {
        Ok(discovery_info) => {
            info!(
                "Background service discovery info read from {}",
                path.display()
            );
            serde_json::json!({
                "code": 200,
                "message": discovery_info
            })
        }
        Err(err) => {
            let message = format!("Invalid JSON in {}: {}", path.display(), err);
            error!("{}", message);
            serde_json::json!({
                "code": 500,
                "message": { "error": message }
            })
        }
    }
}

#[tauri::command]
fn error_command(app: tauri::AppHandle) {
    app.dialog()
        .message("Hello")
        .blocking_show();
}

#[tauri::command]
fn open_folder_in_explorer(path: String) -> Result<(), String> {
    let folder_path = PathBuf::from(path.trim());

    if !folder_path.is_absolute() {
        return Err("Folder path must be absolute".to_string());
    }

    let metadata = fs::metadata(&folder_path)
        .map_err(|error| format!("Folder is not accessible: {error}"))?;

    if !metadata.is_dir() {
        return Err("The selected path is not a folder".to_string());
    }

    Command::new("explorer.exe")
        .arg(&folder_path)
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("Failed to start Windows Explorer: {error}"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     tracing_subscriber::fmt::init();

//     tauri::Builder::default()
//         .plugin(tauri_plugin_opener::init())
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

pub fn run(pdfium: Arc<pdfium_render::prelude::Pdfium>) {

    let (tx, rx) =  crossbeam_channel::unbounded();  //crossbeam_channel::bounded(CHANNEL_CAP);
      
    TX.set(tx).expect("tx already initialized");

    tracing_subscriber::fmt::init();

    let n = std::thread::available_parallelism().unwrap().get();
    info!("Available parallelism: {}", n);

    tauri::Builder::default()
        .setup(move |app| {
            let app_handle = app.handle().clone();

            for thread_id in 0..n {
                let rx = rx.clone();
                let pdfium_clone = pdfium.clone();
                let app_handle = app_handle.clone();

                std::thread::spawn(move || {
                    debug!("Thread {} started", thread_id);
                    loop {// will sleep if no job is available, but will wake up when a job is sent ( condvar replacement thanks to crossbeam_channel ) => litteraly 0 jump
                        match rx.recv() {
                            Ok(command_payload_ipc) => {
                                serde_json::from_str(&command_payload_ipc)
                                    .map(|job_cmd: deserializers::command_message_ipc::CommandPayloadIPC| {
                                        debug!("Thread {} received job_cmd: {:?}", thread_id, job_cmd);

                                        command_manager::execute(
                                            thread_id,
                                            job_cmd,
                                            &pdfium_clone,
                                            &app_handle,
                                        );
                                    }).unwrap_or_else(|err| {
                                        debug!("Thread {} failed to deserialize command_payload_ipc: {}, error: {}", thread_id, command_payload_ipc, err);
                                    });
                            }

                            Err(_) => {
                                debug!("Thread {} exiting", thread_id);
                                break; // kill the thread if the crossbeam channel is closed !
                            },
                        }
                    }
                });
            }

            services::ipc_server::start();
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_single_instance::init(|_app, args, _cwd| {
            // When you select multiple element on explorer, will block the windows and just send infos here
            // so each "info" ( event ) = 1 individual action

            info!("new instance blocked, args: {:?}", args);

            for (index, arg) in args.iter().enumerate() {
                match index {
                    1 => {
                        info!("Cmd : {:?}", arg);
                    }
                    2 => {
                        info!(" > File : {:?}", arg);
                    }
                    _ => {}
                }
            }

            info!("==============================================");
        }))
        .invoke_handler(tauri::generate_handler![
            greet,
            discover_port_winforge_background_service,
            error_command,
            open_folder_in_explorer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
