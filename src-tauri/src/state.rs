use std::path::PathBuf;

use crate::{ai::AI, database::DbPool, vector_store::manager::VectorIndexManager};

pub struct AppState {
    pub db: DbPool,
    pub vector_index_manager: VectorIndexManager,
    pub openai_client: AI,
}

impl AppState {
    pub fn new(db: DbPool, app_dir: PathBuf, openai_client: AI) -> Self {
        Self { 
            db,
            vector_index_manager: VectorIndexManager::new(app_dir),  
            openai_client: openai_client
        }
    }
}