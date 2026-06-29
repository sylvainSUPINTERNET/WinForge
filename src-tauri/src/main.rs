// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing::{debug, info};

fn main() {
    tracing_subscriber::fmt::init();

    let n = std::thread::available_parallelism().unwrap().get();
    info!("Available parallelism: {}", n);
    for id in 0..n {
        std::thread::spawn(move || {
            debug!("Thread {} started", id);
        });
    }

    winforge_lib::run()
}
