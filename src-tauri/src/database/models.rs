use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use sqlx::types::Json;

// SPACES

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddingConfig {
    pub provider: String, // "openai", "local"
    pub model: String,
    pub dimensions: i32,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Space {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub embedding_config: Json<EmbeddingConfig>, 
    pub created_at: NaiveDateTime,
}

// ROOTS

#[derive(Debug, FromRow, Serialize)]
pub struct IndexedRoot {
    pub id: i64,
    pub space_id: i64,
    pub path: String,
    pub status: String, // "active", "paused"
}

// FILES

#[derive(Debug, FromRow, Serialize)]
pub struct FileMetadata {
    pub id: i64,
    pub root_id: i64,
    pub absolute_path: String,
    pub filename: String,
    pub file_extension: Option<String>,
    pub file_size: i64,
    pub modified_at_fs: NaiveDateTime,
    pub last_indexed_at: Option<NaiveDateTime>,
    pub content_hash: Option<String>,
    pub indexing_status: String,
    pub error_message: Option<String>,
}