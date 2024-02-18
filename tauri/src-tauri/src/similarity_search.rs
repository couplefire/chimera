use arrow_array::{RecordBatch, StringArray};
use futures::TryStreamExt;
use anyhow::Result;

use crate::{db::DbConnection, SearchResult, EMBEDDING_DIM};

pub async fn search(db: DbConnection, prompt: Vec<f32>) -> Result<Vec<SearchResult>> {
    let dot_products_with_indices = cosine_similarity_search(db, &prompt).await?;

    let mut file_names = Vec::new();
    for batch in dot_products_with_indices {
        file_names.append(&mut batch.column(0).as_any().downcast_ref::<StringArray>().unwrap().into_iter().map(|x| x.unwrap().to_string()).collect::<Vec<_>>());
    }

    Ok(file_names.into_iter().map(|filename| SearchResult { filename, directory: "_placeholder".to_string() }).collect())
}

async fn cosine_similarity_search(db: DbConnection, prompt: &[f32]) -> Result<Vec<RecordBatch>> {
    if prompt.len() != EMBEDDING_DIM as usize {
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

