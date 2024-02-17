// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use indexer::start_indexing;
use serde::Serialize;
mod indexer;

use tauri::Manager;
use vectordb::{connect, Connection};
use arrow_schema::{DataType, Schema, Field};
use arrow_array::{types::Float32Type, FixedSizeListArray, Int32Array, RecordBatch, RecordBatchIterator};

#[derive(Serialize)]
struct SearchResult {
    filename: String,
    directory: String,
}

type DbConnection = Arc<dyn Connection>;

const TOTAL: usize = 1000;
const DIM: usize = 128;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn search(search_text: &str, state: tauri::State<'_, DbConnection>) -> Result<Vec<SearchResult>, ()> {
    println!("{:?}", state.table_names().await.unwrap());

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

async fn init_db() -> DbConnection {
    let db = connect("../../lancedb-data/sample-lancedb").await.expect("Failed to start lancedb");

    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new(
            "vector",
            DataType::FixedSizeList(
                Arc::new(Field::new("item", DataType::Float32, true)),
                DIM as i32,
            ),
            true,
        ),
    ]));
    
    // Create a RecordBatch stream.
    let batches = RecordBatchIterator::new(
        vec![RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(Int32Array::from_iter_values(0..TOTAL as i32)),
                Arc::new(
                    FixedSizeListArray::from_iter_primitive::<Float32Type, _, _>(
                        (0..TOTAL).map(|_| Some(vec![Some(1.0); DIM])),
                        DIM as i32,
                    ),
                ),
            ],
        )
        .unwrap()]
        .into_iter()
        .map(Ok),
        schema.clone(),
    );
    let tbl = db
        .create_table("my_table", Box::new(batches), None)
        .await;
    if tbl.is_err() {
        // table probably alredy exists
        let _ = db
            .open_table_with_params("my_table", Default::default())
            .await
            .unwrap();
    }
    db
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
