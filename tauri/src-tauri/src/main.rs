// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use indexer::start_indexing;

mod indexer;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    std::thread::spawn(|| {
        println!("Starting indexing process...");
        match start_indexing() {
            Ok(_) => println!("Indexing process finished successfully!"),
            Err(e) => println!("Error while indexing: {}", e),
        }
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
