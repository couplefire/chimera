// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use serde::Serialize;
use tauri::Manager;
use tauri::GlobalShortcutManager;

use db::{DbConnection, init_db};
use indexer::start_indexing;

mod indexer;
mod db;
mod similarity_search;
mod embeddings;
mod parser;

pub const EMBEDDING_DIM: u32 = 128;

#[derive(Serialize)]
struct SearchResult {
    fileName: String,
    directory: String,
    fileSize: u64, 
    numPages: Option<u64>,
}

#[tauri::command]
async fn search(search_text: &str, state: tauri::State<'_, DbConnection>) -> Result<Vec<SearchResult>, ()> {
    let prompt_embed = embeddings::create_embedding_prompt(search_text).await.unwrap();
    let result = similarity_search::search(state.inner().clone(), prompt_embed).await.unwrap();

    Ok(result)
}

#[tauri::command]
async fn open(path: &str, _state: tauri::State<'_, DbConnection>) -> Result<(), ()> {
    open::that(path).unwrap();
    Ok(())
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

            let window = app.get_window("main").unwrap();
            window.hide().unwrap();
            let window_rc1 = Arc::new(window);
            let window_rc2 = Arc::clone(&window_rc1);

            {
                let _toggle = app.app_handle().global_shortcut_manager().register("Cmd+]", move || {
                    println!("Cmd+] pressed");
                    if window_rc1.is_visible().unwrap() {
                        window_rc1.hide().unwrap();
                    } else {
                        window_rc1.show().unwrap();
                    }
                });
            }

            {
                let _hide_esc = app.app_handle().global_shortcut_manager().register("esc", move || {
                    println!("esc pressed");
                    window_rc2.hide().unwrap();
                });
            }

            Ok(()) 
        }) 
        .invoke_handler(tauri::generate_handler![search, open])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
