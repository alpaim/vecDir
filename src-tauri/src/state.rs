use std::path::PathBuf;

use crate::{ai::AI, database::DbPool};

pub struct AppState {
    pub db: DbPool,
    pub openai_client: AI,
}

impl AppState {
    pub fn new(db: DbPool, app_dir: PathBuf, openai_client: AI) -> Self {
        Self { 
            db, 
            openai_client: openai_client
        }
    }
}