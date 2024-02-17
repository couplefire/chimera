use lance::db::connect;

async fn main() -> Result<()> {
    let uri = "data";
    let db = connect(uri).await?;

    let embeddings = db.open_table_with_params("my_table", Default::default())
    .await
    .unwrap();

    print!("Embeddings table opened\n");

}

