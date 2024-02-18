// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use tauri::Manager;

use db::{DbConnection, init_db};
use indexer::start_indexing;

mod indexer;
mod db;
mod similarity_search;
mod embeddings;
mod parser;

pub const EMBEDDING_DIM: i32 = 128;

#[derive(Serialize)]
struct SearchResult {
    fileName: String,
    directory: String,
    fileSize: u64, 
    numPages: Option<u64>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn search(search_text: &str, state: tauri::State<'_, DbConnection>) -> Result<Vec<SearchResult>, ()> {
    let prompt_embed = embeddings::create_embedding_prompt(search_text).unwrap();
    let result = similarity_search::search(state.inner().clone(), prompt_embed).await.unwrap();

    Ok(result)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| { 
            let initialize_db = true;

            let handle = tauri::async_runtime::spawn(async move { 
                let db = init_db(initialize_db).await;
                db
            });
            let db = tauri::async_runtime::block_on(handle).unwrap();
            app.manage(db.clone());

            if initialize_db {
                tauri::async_runtime::spawn(async move {
                    println!("Starting indexing process...");
                    match start_indexing(db).await {
                        Ok(_) => println!("Indexing process finished successfully!"),
                        Err(e) => println!("Error while indexing: {}", e),
                    }
                });
            }

            Ok(()) 
        }) 
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
