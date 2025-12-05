use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}};
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::database::DbPool;

pub async fn perform_migrations(pool: &DbPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;

    Ok(())
}

pub async fn initialize_database(app_dir: &Path) -> Result<DbPool, sqlx::Error> {
    if !app_dir.exists() {
        fs::create_dir_all(app_dir)?;
    }

    let db_path = app_dir.join("vecDir.db");
    let db_url = format!("sqlite:{}", db_path.to_str().unwrap());

    let options = SqliteConnectOptions::from_str(&db_url)?
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;
    
    perform_migrations(&pool).await?;

    Ok(pool)
}