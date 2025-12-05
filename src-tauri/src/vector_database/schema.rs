use arrow_schema::{DataType, Field, Schema};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct VectorSearchResult {
    pub file_id: i64,
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

