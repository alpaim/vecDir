use lancedb::Connection;

use crate::database::DbPool;

pub struct AppState {
    pub db: DbPool,
    pub vec_db: Connection
}

impl AppState {
    pub fn new(db: DbPool, vec_db: Connection) -> Self {
        Self { db, vec_db }
    }
}