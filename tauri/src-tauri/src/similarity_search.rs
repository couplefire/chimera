use arrow_array::{RecordBatch, StringArray, UInt64Array};
use futures::TryStreamExt;
use anyhow::Result;

use crate::{db::DbConnection, SearchResult, EMBEDDING_DIM};

pub async fn search(db: DbConnection, prompt: Vec<f32>) -> Result<Vec<SearchResult>> {
    let dot_products_with_indices = cosine_similarity_search(db, &prompt).await?;

    let mut metadata: Vec<SearchResult> = Vec::new();
    for batch in dot_products_with_indices {
        let file_names = batch.column(0).as_any().downcast_ref::<StringArray>().unwrap().into_iter().map(|x| x.unwrap().to_string()).collect::<Vec<_>>();
        let directories = batch.column(2).as_any().downcast_ref::<StringArray>().unwrap().into_iter().map(|x| x.unwrap().to_string()).collect::<Vec<_>>();
        let file_sizes = batch.column(3).as_any().downcast_ref::<UInt64Array>().unwrap().into_iter().map(|x| x.unwrap()).collect::<Vec<_>>();
        let num_pagess = batch.column(4).as_any().downcast_ref::<UInt64Array>().unwrap().into_iter().map(|x| x.unwrap()).collect::<Vec<_>>();
        metadata.append(&mut file_names.iter().zip(directories.iter()).zip(file_sizes.iter()).zip(num_pagess.iter()).map(|(((file_name, directory), file_size), num_pages)| {
            SearchResult {
                fileName: file_name.to_string(),
                directory: directory.to_string(),
                fileSize: *file_size,
                numPages: Some(*num_pages),
            }
        }).collect());
    }

    Ok(metadata)
}

async fn cosine_similarity_search(db: DbConnection, prompt: &[f32]) -> Result<Vec<RecordBatch>> {
    if prompt.len() != 2 * EMBEDDING_DIM as usize {
        return Err(anyhow::anyhow!("Prompt must be 128-dimensional"));
    }

    let embeddings = db.db.open_table_with_params("files", Default::default())
        .await
        .unwrap();

    let stream = embeddings.query().nearest_to(prompt).refine_factor(5).nprobes(10)
        .execute_stream()
        .await
        .unwrap();

    let batches: Vec<RecordBatch> = stream.try_collect().await.unwrap();
    Ok(batches)
}

