use crate::database::{models::VectorSearchResult, DbPool};
use anyhow::{Context, Result};
use sqlx::Row;
use sqlx::{types::Json, QueryBuilder, Sqlite};

pub struct AddFileChunk {
    pub file_id: i32,

    pub chunk_index: i32,
    pub content: String,

    pub start_char_idx: Option<i32>,
    pub end_char_idx: Option<i32>,

    pub embedding: Vec<f32>,
}

fn f32_vec_to_bytes(data: &[f32]) -> Result<Vec<u8>> {
    let bytes = data.iter().flat_map(|&f| f.to_le_bytes()).collect();

    Ok(bytes)
}

pub async fn add_chunk(pool: &DbPool, chunk: AddFileChunk) -> Result<i32> {
    let mut tx = pool
        .begin()
        .await
        .context("failed to begin database transaction in add_chunk")?;

    let chunk_id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO file_chunks (
            file_id, chunk_index, content, start_char_idx, end_char_idx
        )
        VALUES (?, ?, ?, ?, ?)
        RETURNING id
        "#,
    )
    .bind(chunk.file_id)
    .bind(chunk.chunk_index)
    .bind(chunk.content)
    .bind(chunk.start_char_idx)
    .bind(chunk.end_char_idx)
    .fetch_one(&mut *tx)
    .await
    .context("failed to insert chunk in add_chunk")?;

    let vec_bytes = f32_vec_to_bytes(&chunk.embedding)
        .context("failed to convert embedding vector to bytes add_chunk")?;

    sqlx::query(
        r#"
        INSERT INTO vec_chunks (rowid, embedding)
        VALUES (?, ?)
        "#,
    )
    .bind(chunk_id)
    .bind(vec_bytes)
    .execute(&mut *tx)
    .await
    .context("failed to insert embedding to vec_chunks")?;

    tx.commit()
        .await
        .context("failed to commit transaction in add_chunk")?;

    Ok(chunk_id)
}

pub async fn add_chunks_batch(pool: &DbPool, chunks: Vec<AddFileChunk>) -> Result<Vec<i32>> {
    if chunks.is_empty() {
        return Ok(vec![]);
    }

    // generally it should be safe up to 5000
    const BATCH_SIZE: usize = 4000;

    let mut added_ids = Vec::new();
    let mut tx = pool.begin().await.context("failed to begin transaction")?;

    for batch in chunks.chunks(BATCH_SIZE) {
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "INSERT INTO file_chunks (file_id, chunk_index, content, start_char_idx, end_char_idx) "
        );
        query_builder.push_values(batch, |mut b, chunk| {
            b.push_bind(chunk.file_id)
                .push_bind(chunk.chunk_index)
                .push_bind(&chunk.content)
                .push_bind(chunk.start_char_idx)
                .push_bind(chunk.end_char_idx);
        });

        query_builder.push(" RETURNING id");

        let batch_ids: Vec<i32> = query_builder
            .build()
            .fetch_all(&mut *tx)
            .await
            .context("failed to batch insert file_chunks")?
            .iter()
            .map(|row| row.get("id"))
            .collect();

        let mut vec_builder: QueryBuilder<Sqlite> =
            QueryBuilder::new("INSERT INTO vec_chunks (rowid, embedding) ");

        let zip_iter = batch_ids.iter().zip(batch.iter());

        vec_builder.push_values(zip_iter, |mut b, (id, chunk)| {
            let vec_bytes = f32_vec_to_bytes(&chunk.embedding)
                .context("failed to convert embedding vector to bytes")
                .unwrap();

            b.push_bind(id) // rowid
                .push_bind(vec_bytes); // embedding
        });

        vec_builder
            .build()
            .execute(&mut *tx)
            .await
            .context("failed to batch insert vec_chunks")?;

        added_ids.extend(batch_ids);
    }

    tx.commit()
        .await
        .context("failed to commit batch transaction")?;

    Ok(added_ids)
}

pub async fn search_similar_chunks(
    pool: &DbPool,
    query_vector: Vec<f32>,
    limit: i32,
) -> Result<Vec<VectorSearchResult>> {
    let query_bytes =
        f32_vec_to_bytes(&query_vector).context("failed to convert embedding vector to bytes")?;

    let results = sqlx::query_as::<_, VectorSearchResult>(
        r#"
        SELECT 
            fc.id as chunk_id,
            fc.content,
            fm.id as file_id,
            fm.absolute_path,
            fm.filename,
            v.distance
        FROM vec_chunks v
        JOIN file_chunks fc ON fc.id = v.rowid
        JOIN files_metadata fm ON fm.id = fc.file_id
        WHERE v.embedding MATCH ? 
          AND k = ?
        ORDER BY v.distance ASC
        "#,
    )
    .bind(query_bytes)
    .bind(limit)
    .fetch_all(pool)
    .await
    .context("failed to find similar vectors in database in search_similar_vectors")?;

    Ok(results)
}
