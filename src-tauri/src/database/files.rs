use sqlx;
use crate::database::DbPool;

use super::models::FileMetadata;
use chrono::NaiveDateTime;

pub async fn upsert_file(
    pool: &DbPool,
    root_id: i64,
    path: &str,
    filename: &str,
    file_extension: Option<&str>,
    size: i64,
    modified: NaiveDateTime,
) -> Result<(), sqlx::Error> {
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

pub async fn get_pending_files(pool: &DbPool, limit: i64) -> Result<Vec<FileMetadata>, sqlx::Error> {
    sqlx::query_as::<_, FileMetadata>(
        "SELECT * FROM files_metadata WHERE indexing_status = 'pending' LIMIT ?"
    )
    .bind(limit)
    .fetch_all(pool)
    .await
}