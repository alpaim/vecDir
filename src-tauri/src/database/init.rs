use anyhow::{Result, Ok};
use libsqlite3_sys::sqlite3_auto_extension;
use sqlite_vec::sqlite3_vec_init;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}};
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::database::DbPool;

pub async fn perform_migrations(pool: &DbPool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;

    Ok(())
}

pub async fn initialize_database(app_dir: &Path) -> Result<DbPool> {
    if !app_dir.exists() {
        fs::create_dir_all(app_dir)?;
    }

    let db_path = app_dir.join("vecDir.db");
    let db_url = format!("sqlite:{}", db_path.to_str().unwrap());

    // registering sqlite3_vec ext
    unsafe {
        let init_fn = std::mem::transmute(sqlite3_vec_init as *const ());
        sqlite3_auto_extension(Some(init_fn));
    }

    let options = SqliteConnectOptions::from_str(&db_url)?
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;
    
    perform_migrations(&pool).await?;

    let ver: String = sqlx::query_scalar("select vec_version()").fetch_one(&pool).await?;
    println!("sqlite-vec loaded: {}", ver);

    Ok(pool)
}