use std::{iter, sync::Arc};


use vectordb::{connect, Connection};
use arrow_schema::{DataType, Schema, Field};
use arrow_array::RecordBatchIterator;
use crate::indexer::start_indexing;

use crate::EMBEDDING_DIM;

#[derive(Clone)]
pub struct DbConnection {
    pub db: Arc<dyn Connection>,
    pub schema: Arc<Schema>,
}

pub async fn init_db(initialize_db: bool) -> DbConnection {
    let our_db = connect("../../lancedb-data/sample-lancedb").await.expect("Failed to start lancedb");

    let our_schema = Arc::new(Schema::new(vec![
        Field::new("file_name", DataType::Utf8, false),
        Field::new(
            "vector",
            DataType::FixedSizeList(
                Arc::new(Field::new("item", DataType::Float32, true)),
                (EMBEDDING_DIM * 2) as i32,
            ),
            true,
        ),
        Field::new("directory", DataType::Utf8, false),
        Field::new("file_size", DataType::UInt64, false),
        Field::new("num_pages", DataType::UInt64, false)
    ]));
    
    let batches = RecordBatchIterator::new(
        iter::empty(),
        our_schema.clone(),
    );

    if initialize_db {
        let _ = our_db.drop_table("files").await;
        let _ = our_db.create_table("files", Box::new(batches), None).await.expect("Failed to create table");
    }

    DbConnection {
        db: our_db,
        schema: our_schema,
    }
}
