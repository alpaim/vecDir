use crate::database::models::FileMetadata;
use crate::database::DbPool;
use anyhow::{Ok, Result};
use chrono::{DateTime, Utc};
use sqlx::{self, QueryBuilder};

pub async fn upsert_file(
    pool: &DbPool,
    root_id: i64,
    path: &str,
    filename: &str,
    file_extension: &str,
    size: i64,
    modified: DateTime<Utc>,
) -> Result<()> {
    // If the file already exists, updating its status to "pending";
    // ONLY if date of modification changes!
    sqlx::query!(
        r#"
        INSERT INTO files_metadata (
            root_id, absolute_path, filename, file_extension, file_size, modified_at_fs, indexing_status
        )
        VALUES (?, ?, ?, ?, ?, ?, 'pending')
        ON CONFLICT(root_id, absolute_path) DO UPDATE SET
            file_size = excluded.file_size,
            modified_at_fs = excluded.modified_at_fs,
            indexing_status = CASE 
                WHEN files_metadata.modified_at_fs != excluded.modified_at_fs THEN 'pending'
                ELSE files_metadata.indexing_status
            END
        "#,
        root_id, path, filename, file_extension, size, modified
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_pending_files(pool: &DbPool, limit: i64) -> Result<Vec<FileMetadata>> {
    let res = sqlx::query_as::<_, FileMetadata>(
        "SELECT * FROM files_metadata WHERE indexing_status = 'pending' LIMIT ?",
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(res)
}

pub async fn get_files_by_ids(pool: &DbPool, ids: Vec<i64>) -> Result<Vec<FileMetadata>> {
    if ids.is_empty() {
        return Ok(vec![]);
    }

    let mut query_builder = QueryBuilder::new("SELECT * FROM files_metadata WHERE id IN (");

    let mut separated = query_builder.separated(", ");
    for id in ids {
        separated.push_bind(id);
    }
    separated.push_unseparated(")");

    let query = query_builder.build_query_as::<FileMetadata>();

    let res = query.fetch_all(pool).await?;

    Ok(res)
}

pub async fn get_all_files(pool: &DbPool) -> Result<Vec<FileMetadata>> {
    let res = sqlx::query_as::<_, FileMetadata>("SELECT * FROM files_metadata")
        .fetch_all(pool)
        .await?;

    Ok(res)
}
