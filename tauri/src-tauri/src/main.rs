// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use tauri::Manager;

use db::{DbConnection, init_db};
use indexer::start_indexing;

mod indexer;
mod db;
mod similarity_search;

#[derive(Serialize)]
struct SearchResult {
    filename: String,
    directory: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn search(search_text: &str, state: tauri::State<'_, DbConnection>) -> Result<Vec<SearchResult>, ()> {
    println!("{:?}", state.table_names().await.unwrap());

    similarity_search::search(state.inner().clone(), vec![1.0, 2.0, 3.0]).await.unwrap();

    let result = vec![
        SearchResult {
            filename: "Hello".to_string(),
            directory: "World".to_string(),
        },
        SearchResult {
            filename: search_text.to_string(),
            directory: "suck".to_string(),
        },
    ];

    Ok(result)
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
        .setup(|app| { 
            // we perform the initialization code on a new task so the app doesn't freeze 
            let handle = tauri::async_runtime::spawn(async move { 
                let db = init_db().await;
                db
            });
            let db = tauri::async_runtime::block_on(handle).unwrap();
            app.manage(db);
            Ok(()) 
        }) 
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
