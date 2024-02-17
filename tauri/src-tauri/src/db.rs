use std::{iter, sync::Arc};


use vectordb::{connect, Connection};
use arrow_schema::{DataType, Schema, Field};
use arrow_array::RecordBatchIterator;

use crate::EMBEDDING_DIM;

#[derive(Clone)]
pub struct DbConnection {
    pub db: Arc<dyn Connection>,
    pub schema: Arc<Schema>,
}

pub async fn init_db() -> DbConnection {
    let db = connect("../../lancedb-data/sample-lancedb").await.expect("Failed to start lancedb");

    let schema = Arc::new(Schema::new(vec![
        Field::new("file_name", DataType::Utf8, false),
        Field::new(
            "vector",
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

    let _ = db.drop_table("files").await;
    db.create_table("files", Box::new(batches), None).await.expect("Failed to create table");

    DbConnection {
        db,
        schema,
    }
}
