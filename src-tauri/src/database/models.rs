use specta::Type;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use sqlx::types::Json;

// APP CONFIG
#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct AppConfig {
    // UI
    #[serde(default = "default_theme")]
    pub theme: String, // "light", "dark", "system"

    // Indexer Settings
    #[serde(default = "default_parallelism")]
    #[specta(type = i32)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct LLMConfig {
    pub model: String,
    pub system_prompt: String,
    pub user_prompt: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct EmbeddingConfig {
    pub model: String,
    pub dimensions: i32,
}

#[derive(Debug, FromRow, Serialize, Type)]
pub struct Space {
    pub id: i32,

    pub name: String,
    pub description: Option<String>,

    #[specta(type = EmbeddingConfig)]
    pub embedding_config: Json<EmbeddingConfig>,

    #[specta(type = LLMConfig)] 
    pub llm_config: Json<LLMConfig>,

    pub created_at: DateTime<Utc>,
}

// ROOTS

#[derive(Debug, FromRow, Serialize, Type)]
pub struct IndexedRoot {
    pub id: i32,
    pub space_id: i32,
    pub path: String,
    pub status: String, // "active", "paused"
}

// FILES

#[derive(Debug, FromRow, Serialize, Type)]
pub struct FileMetadata {
    pub id: i32,
    pub root_id: i32,

    pub absolute_path: String,
    pub filename: String,
    pub file_extension: String,

    #[specta(type = i32)]
    pub file_size: u32,

    pub description: Option<String>,

    pub modified_at_fs: DateTime<Utc>,
    pub last_indexed_at: Option<DateTime<Utc>>,
    pub content_hash: Option<String>,
    
    pub indexing_status: String,
    pub indexing_error_message: Option<String>,
}

// CHUNKS
#[derive(Debug, FromRow, Serialize, Type)]
pub struct FileChunk {
    pub id: i32,
    pub file_id: i32,

    pub chunk_index: i32,
    pub content: String,

    pub start_char_idx: Option<i32>,
    pub end_char_idx: Option<i32>
}

// VECTOR SEARCH RESULT
#[derive(Debug, FromRow, Serialize, Type)]
pub struct VectorSearchResult {
    pub chunk_id: i32,
    pub content: String,

    pub file_id: i32,
    pub absolute_path: String,
    pub filename: String,

    pub distance: f32,
}
