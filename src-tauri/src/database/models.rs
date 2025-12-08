use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use sqlx::types::Json;

// APP CONFIG
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    // UI
    #[serde(default = "default_theme")]
    pub theme: String, // "light", "dark", "system"

    // Indexer Settings
    #[serde(default = "default_parallelism")]
    pub indexer_parallelism: usize, // How many files vecDir needs to index in parallel (2-4)
    
    // AI Settings (Global defaults)
    pub default_openai_url: Option<String>,
}

// Default values
fn default_theme() -> String { "system".to_string() }
fn default_parallelism() -> usize { 2 }

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            indexer_parallelism: default_parallelism(),
            default_openai_url: None
        }
    }
}

// SPACES

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LLMConfig {
    pub open_ai_base_url: String,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddingConfig {
    pub open_ai_base_url: String,
    pub model: String,
    pub dimensions: i32,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Space {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub embedding_config: Json<EmbeddingConfig>, 
    pub created_at: DateTime<Utc>,
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
    pub modified_at_fs: DateTime<Utc>,
    pub last_indexed_at: Option<DateTime<Utc>>,
    pub content_hash: Option<String>,
    pub indexing_status: String,
    pub indexing_error_message: Option<String>,
}