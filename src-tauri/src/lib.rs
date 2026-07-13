mod services;
mod worker_pool;
mod deserializers;
mod commands;

use std::sync::Arc;

use crate::worker_pool::TX;

use crossbeam_channel::{Sender, Receiver};
use tracing::{debug, info};
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
fn error_command(app: tauri::AppHandle) {
    app.dialog()
        .message("Hello")
        .blocking_show();
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

    for thread_id in 0..n {
        let rx = rx.clone();
        let pdfium_clone = pdfium.clone();


        std::thread::spawn(move || {
            debug!("Thread {} started", thread_id);
            loop {// will sleep if no job is available, but will wake up when a job is sent ( condvar replacement thanks to crossbeam_channel ) => litteraly 0 jump
                match rx.recv() {
                    Ok(command_payload_ipc) => {

                        serde_json::from_str(&command_payload_ipc)
                                    .map(|job_cmd: deserializers::command_message_ipc::CommandPayloadIPC| { 
                                        debug!("Thread {} received job_cmd: {:?}", thread_id, job_cmd);
                                        
                                        command_manager::execute(thread_id, job_cmd, &pdfium_clone);
        
                        
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


    tauri::Builder::default()
        .setup(|_app| {
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
        .invoke_handler(tauri::generate_handler![greet, error_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
