use std::path::PathBuf;

use crate::{database::DbPool, vector_store::manager::VectorIndexManager};

pub struct AppState {
    pub db: DbPool,
    pub vector_index_manager: VectorIndexManager,
}

impl AppState {
    pub fn new(db: DbPool, app_dir: PathBuf) -> Self {
        Self { 
            db,
            vector_index_manager: VectorIndexManager::new(app_dir),  
        }
    }
}