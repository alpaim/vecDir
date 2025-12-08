use crate::database::models::FileMetadata;
use crate::database::DbPool;
use anyhow::{Ok, Result};
use chrono::{DateTime, Utc};
use sqlx::{self, QueryBuilder};

pub struct UpsertFile {
    pub root_id: i64,
    pub path: String,
    pub filename: String,
    pub file_extension: String,
    pub size: i64,
    pub modified: DateTime<Utc>,
}

pub type UpsertFilesBatch = Vec<UpsertFile>;

pub async fn upsert_file(pool: &DbPool, file: UpsertFile) -> Result<()> {
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
        file.root_id, file.path, file.filename, file.file_extension, file.size, file.modified
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn upsert_files_batch(pool: &DbPool, files: UpsertFilesBatch) -> Result<()> {
    const BATCH_SIZE: usize = 500;

    for chunk in files.chunks(BATCH_SIZE) {
        let mut query_builder = QueryBuilder::new(
            "INSERT INTO files_metadata (
                root_id, 
                absolute_path, 
                filename, 
                file_extension, 
                file_size, 
                modified_at_fs, 
                indexing_status
            ) "
        );

        // adds VALUES to query by default 
        query_builder.push_values(chunk, |mut b, file| {
            b.push_bind(&file.root_id);
            b.push_bind(&file.path);
            b.push_bind(&file.filename);
            b.push_bind(&file.file_extension);
            b.push_bind(&file.size);
            b.push_bind(&file.modified);
            b.push_bind("pending"); 
        });

        // upsert logic on conflict in raw SQL
        query_builder.push(
            " ON CONFLICT(root_id, absolute_path) DO UPDATE SET 
                file_size = excluded.file_size,
                modified_at_fs = excluded.modified_at_fs,
                indexing_status = CASE 
                    WHEN files_metadata.modified_at_fs != excluded.modified_at_fs THEN 'pending'
                    ELSE files_metadata.indexing_status
                END"
        );
        
        let query = query_builder.build();
        query.execute(pool).await?;
    }


    Ok(())
}

pub async fn mark_file_as_indexed(pool: &DbPool, file_id: i64) -> Result<()> {
    sqlx::query!(
        "UPDATE files_metadata SET indexing_status = 'indexed', last_indexed_at = CURRENT_TIMESTAMP WHERE id = ?",
        file_id
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
