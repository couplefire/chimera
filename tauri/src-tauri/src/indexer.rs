use arrow_array::types::Float32Type;
use arrow_array::{FixedSizeListArray, RecordBatch, RecordBatchIterator, StringArray, Int32Array, UInt64Array};
use walkdir::WalkDir;
use std::fs;
use std::io::Read;
use std::sync::Arc;
use anyhow::{Context, Result};

use crate::db::DbConnection;
use crate::{embeddings, EMBEDDING_DIM};
use crate::parser::{ParsedFile, parse};

pub async fn start_indexing(db: DbConnection) -> Result<()> {
    let tbl = db.db
        .open_table_with_params("files", Default::default())
        .await
        .unwrap();

    let folder_path = std::env::current_dir().unwrap().join("../../files-to-index");
    let mut parsed_files = Vec::new();
    for entry in WalkDir::new(folder_path) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let parsed_file = parse(path.to_str().unwrap());
            parsed_files.push(parsed_file);
            println!("Discovered file {:?}", path);
        }
    }
    println!("Indexing...");

    let file_embeddings = embeddings::create_embedding_files(parsed_files.clone()).await?;
    let parsed_file_names = parsed_files.iter().map(|x| x.name.clone());
    let parsed_file_sizes = parsed_files.iter().map(|x| x.file_size as i32);
    let parsed_file_num_pages = parsed_files.iter().map(|x| x.num_pages.unwrap_or_default() as i32);

    tbl.add(Box::new(RecordBatchIterator::new(
        vec![RecordBatch::try_new(
            db.schema.clone(),
            vec![
                Arc::new(StringArray::from_iter_values(parsed_file_names)),
                Arc::new(
                    FixedSizeListArray::from_iter_primitive::<Float32Type, _, _>(
                        file_embeddings.into_iter().map(|embed| Some(embed.into_iter().map(Some).collect::<Vec<_>>())),
                        EMBEDDING_DIM as i32,
                    ),
                ),
                Arc::new(Int32Array::from_iter_values(parsed_file_sizes)),
                Arc::new(Int32Array::from_iter_values(parsed_file_num_pages))
            ],
        )
        .unwrap()].into_iter().map(Ok),
        db.schema.clone(),
    )), None).await.expect("Failed to add to vector db");

    Ok(())
}
