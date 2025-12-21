use crate::database::{
    models::{EmbeddingConfig, IndexedRoot, LLMConfig, Space},
    DbPool,
};
use anyhow::{Context, Ok, Result};
use sqlx::{types::Json, Row};

pub async fn create_space(
    pool: &DbPool,
    name: &str,
    description: &str,
    llm_config: LLMConfig,
    embedding_config: EmbeddingConfig,
) -> Result<i32> {
    let llm_config_json = Json(llm_config);
    let embedding_config_json = Json(embedding_config);

    let record = sqlx::query(
        r#"
        INSERT INTO spaces (name, description, llm_config, embedding_config)
        VALUES (?, ?, ?, ?)
        RETURNING id
        "#,
    )
    .bind(name)
    .bind(description)
    .bind(llm_config_json)
    .bind(embedding_config_json)
    .fetch_one(pool)
    .await?;

    let id: i32 = record.get("id");

    Ok(id)
}

pub async fn update_space(
    pool: &DbPool,
    space_id: i32,
    name: &str,
    description: &str,
    llm_config: LLMConfig,
    embedding_config: EmbeddingConfig,
) -> Result<()> {
    let llm_config_json = Json(llm_config);
    let embedding_config_json = Json(embedding_config);

    sqlx::query(
        r#"
        UPDATE spaces 
        SET name = ?, description = ?, llm_config = ?, embedding_config = ? 
        WHERE id = ?
        "#,
    )
    .bind(name)
    .bind(description)
    .bind(llm_config_json)
    .bind(embedding_config_json)
    .bind(space_id)
    .execute(pool)
    .await
    .context("failed to update space")?;

    Ok(())
}

pub async fn get_space_by_id(pool: &DbPool, space_id: i32) -> Result<Space> {
    let res = sqlx::query_as::<_, Space>("SELECT * FROM spaces WHERE id = ?")
        .bind(space_id)
        .fetch_one(pool)
        .await?;

    Ok(res)
}

pub async fn get_all_spaces(pool: &DbPool) -> Result<Vec<Space>> {
    let res = sqlx::query_as::<_, Space>("SELECT * FROM spaces ORDER BY id DESC")
        .fetch_all(pool)
        .await?;

    Ok(res)
}

pub async fn add_root(pool: &DbPool, space_id: i32, path: &str) -> Result<i32> {
    let record =
        sqlx::query("INSERT INTO indexed_roots (space_id, path) VALUES (?, ?) RETURNING id")
            .bind(space_id)
            .bind(path)
            .fetch_one(pool)
            .await?;

    let id: i32 = record.get("id");

    Ok(id)
}

pub async fn delete_root(pool: &DbPool, pool_id: i32) -> Result<()> {
    sqlx::query("DELETE FROM indexed_roots WHERE id = ?")
        .bind(pool_id)
        .execute(pool)
        .await
        .context("failed to delete root")?;

    Ok(())
}

pub async fn get_roots_by_space_id(pool: &DbPool, space_id: i32) -> Result<Vec<IndexedRoot>> {
    let res = sqlx::query_as::<_, IndexedRoot>("SELECT * FROM indexed_roots WHERE space_id = ?")
        .bind(space_id)
        .fetch_all(pool)
        .await?;

    Ok(res)
}
