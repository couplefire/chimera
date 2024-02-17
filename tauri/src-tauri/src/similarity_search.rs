use arrow_array::RecordBatch;
use futures::TryStreamExt;
use anyhow::Result;

use crate::db::DbConnection;

pub async fn search(db: DbConnection, prompt: Vec<f32>) -> Result<()> {
    let dot_products_with_indices = cosine_similarity_search(db, &prompt).await?;

    for batch in dot_products_with_indices {
        for column in batch.columns() {
            println!("{:?}", column);
        } 
    }
    // for (index, cosine_similarity) in dot_products_with_indices.iter().take(5) {
    //     println!("Index: {}, Cosine Similarity: {}", index, cosine_similarity);
    // }

    Ok(())
}

async fn cosine_similarity_search(db: DbConnection, prompt: &[f32]) -> Result<Vec<RecordBatch>> {
    let _embeddings = db.open_table_with_params("files", Default::default())
        .await
        .unwrap();

    let stream = _embeddings.query().nearest_to(prompt)
        .refine_factor(5)
        .nprobes(10)
        .execute_stream()
        .await
        .unwrap();

    let batches: Vec<RecordBatch> = stream.try_collect().await.unwrap();
    Ok(batches)
}

