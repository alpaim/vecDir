use crate::database::DbPool;

pub struct AppState {
    pub db: DbPool,
}

impl AppState {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }
}
