use std::{iter, sync::Arc};


use vectordb::{connect, Connection};
use arrow_schema::{DataType, Schema, Field};
use arrow_array::RecordBatchIterator;

pub type DbConnection = Arc<dyn Connection>;

const EMBEDDING_DIM: i32 = 128;

pub async fn init_db() -> DbConnection {
    let db = connect("../../lancedb-data/sample-lancedb").await.expect("Failed to start lancedb");

    let schema = Arc::new(Schema::new(vec![
        Field::new("file_name", DataType::Utf8, false),
        Field::new(
            "file_name_embedding",
            DataType::FixedSizeList(
                Arc::new(Field::new("item", DataType::Float32, true)),
                EMBEDDING_DIM,
            ),
            true,
        ),
        Field::new(
            "content_embedding",
            DataType::FixedSizeList(
                Arc::new(Field::new("item", DataType::Float32, true)),
                EMBEDDING_DIM,
            ),
            true,
        ),
    ]));
    
    let batches = RecordBatchIterator::new(
        iter::empty(),
        schema.clone(),
    );

    match db
        .create_table("files", Box::new(batches), None)
        .await {
        Ok(_) => {}
        Err(vectordb::Error::TableAlreadyExists { name: _ }) => {
            let _ = db
                .open_table_with_params("files", Default::default())
                .await
                .unwrap();
        }
        Err(e) => panic!("Error while creating table: {}", e)
    }

    db
}
