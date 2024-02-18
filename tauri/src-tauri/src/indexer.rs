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
    for entry in WalkDir::new(folder_path) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let parsed_file = parse(path.to_str().unwrap());
            let embed = embeddings::create_embedding_file(parsed_file.clone()).await.with_context(|| format!("Failed to index file {}", parsed_file.name))?;

            tbl.add(Box::new(RecordBatchIterator::new(
                vec![RecordBatch::try_new(
                    db.schema.clone(),
                    vec![
                        Arc::new(StringArray::from_iter_values(vec![parsed_file.name])),
                        Arc::new(
                            FixedSizeListArray::from_iter_primitive::<Float32Type, _, _>(
                                vec![Some(embed.into_iter().map(Some).collect::<Vec<_>>())].into_iter(),
                                EMBEDDING_DIM as i32,
                            ),
                        ),
                        Arc::new(Int32Array::from_iter_values(vec![parsed_file.file_size as i32])),
                        Arc::new(Int32Array::from_iter_values(vec![parsed_file.num_pages.unwrap_or_default() as i32]))
                    ],
                )
                .unwrap()].into_iter().map(Ok),
                db.schema.clone(),
            )), None).await.expect("Failed to add to vector db");

            println!("Indexed file {:?}", path);
        }
    }
    Ok(())
}
