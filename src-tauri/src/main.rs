// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use pdfium_render::prelude::Pdfium;
fn assert_send_sync<T: Send + Sync>() {}

fn main() {
    let pdfium = Pdfium::new(
        // https://github.com/bblanchon/pdfium-binaries
        Pdfium::bind_to_library("./pdfium-win-x64.dll").expect("Failed to bind to pdfium library"),
    );

    let a = Arc::new(pdfium);
    winforge_lib::run(a)
}
