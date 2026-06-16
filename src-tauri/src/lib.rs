mod menu;
mod services;

use std::any::type_name;
use tracing::{info, debug};

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

// mod services;

// use crate::services::image_converter;


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
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            // When you select multiple element on explorer, will block the windows and just send infos here
            // so each "info" ( event ) = 1 individual action 

            info!("new instance blocked, args: {:?}", args);
            // print_type_of(&args);

            for (index, arg) in args.iter().enumerate() {
                match index {
                    1 => {
                        info!("Cmd : {:?}", arg);
                    }
                    2 => {
                        info!(" > File : {:?}", arg);
                    }
                    _ => {

                    }
                }
            }

            info!("==============================================");

        }))
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
