use vectordb::connect;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect(); 

    let mut prompt: Vec<u64> = args[1..].iter().map(|s| s.parse().unwrap()).collect();

    let dot_products_with_indices = cosine_similarity_search(&mut prompt);

    for (index, cosine_similarity) in dot_products_with_indices.iter().take(5) {
        println!("Index: {}, Cosine Similarity: {}", index, cosine_similarity);
    }

    .await
    .unwrap();

    Ok(())
}

fn cosine_similarity_search(prompt: &mut[u64]) {
    let uri = "test_data";
    let db = connect(uri).await?;

    let _embeddings = db.open_table_with_params("embeddings", Default::default())
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

