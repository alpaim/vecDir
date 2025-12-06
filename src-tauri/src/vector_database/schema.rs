use serde::{self, Deserialize};
use arrow_schema::{DataType, Field, Schema};
use std::sync::Arc;

#[derive(Debug, Deserialize, Clone)]
pub struct VectorSearchResult {
    pub file_id: i64,

    #[serde(rename = "_distance")] // mapping LanceDB system _distance into distance: f32
    pub distance: f32,
}

pub fn get_embeddings_schema(dimensions: i32) -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        // files_metadata.id from SQLite schema
        Field::new("file_id", DataType::Int64, false),
        // Vector
        Field::new(
            "vector",
            DataType::FixedSizeList(
                Arc::new(Field::new("item", DataType::Float32, true)),
                dimensions,
            ),
            false,
        ),
    ]))
}

