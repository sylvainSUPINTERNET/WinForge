mod menu;
mod services;
mod worker_pool;

use crate::worker_pool::TX;

use crossbeam_channel::{Sender, Receiver};
use tracing::{debug, info};


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    menu::menu::menu_select_action(std::env::args().collect());
    // let args: Vec<String> = std::env::args().collect();
    // image_converter::convert_png_to_jpeg(args);
    format!("Hello, {}! You've been greeted from Rust!", name)
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

pub fn run() {

    let (tx, rx) = crossbeam_channel::unbounded();    
    TX.set(tx).expect("tx already initialized");

    tracing_subscriber::fmt::init();

    let n = std::thread::available_parallelism().unwrap().get();
    info!("Available parallelism: {}", n);

    for id in 0..n {
        let rx = rx.clone();
        
        std::thread::spawn(move || {
            debug!("Thread {} started", id);
            loop {// will sleep if no job is available, but will wake up when a job is sent ( condvar replacement thanks to crossbeam_channel ) => litteraly 0 jump
                match rx.recv() {
                    Ok(job) => {
                        debug!("Thread {} received job: {}", id, job);
                    }
                    Err(_) => {
                        debug!("Thread {} exiting", id);
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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
