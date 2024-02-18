use arrow_array::{FixedSizeListArray, Float32Array, PrimitiveArray, RecordBatch, StringArray};
use futures::TryStreamExt;
use anyhow::Result;

use crate::{db::DbConnection, SearchResult};

pub async fn search(db: DbConnection, prompt: Vec<f32>) -> Result<Vec<SearchResult>> {
    let dot_products_with_indices = cosine_similarity_search(db, &prompt).await?;

    let mut file_names = Vec::new();
    for batch in dot_products_with_indices {
        file_names.append(&mut batch.column(0).as_any().downcast_ref::<StringArray>().unwrap().into_iter().map(|x| x.unwrap().to_string()).collect::<Vec<_>>());

        // ask chatgpt: https://chat.openai.com/share/eb087c64-3453-4b7e-a316-345b966470d8
        // scores.append(&mut batch.column(1).as_any().downcast_ref::<FixedSizeListArray>().unwrap());
    }

    Ok(file_names.into_iter().map(|filename| SearchResult { filename, directory: "_placeholder".to_string() }).collect())
}

async fn cosine_similarity_search(db: DbConnection, prompt: &[f32]) -> Result<Vec<RecordBatch>> {
    let embeddings = db.db.open_table_with_params("files", Default::default())
        .await
        .unwrap();

    let stream = embeddings.query()
        .execute_stream()
        .await
        .unwrap();

    let batches: Vec<RecordBatch> = stream.try_collect().await.unwrap();
    Ok(batches)
}

