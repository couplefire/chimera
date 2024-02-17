// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use indexer::start_indexing;
use serde::Serialize;
mod indexer;


#[derive(Serialize)]
struct SearchResult {
    filename: String,
    directory: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn search(searchText: &str) -> Vec<SearchResult> {
    let result = vec![
        SearchResult {
            filename: "Hello".to_string(),
            directory: "World".to_string(),
        },
        SearchResult {
            filename: "you".to_string(),
            directory: "suck".to_string(),
        },
    ];
    result
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
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
