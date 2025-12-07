use crate::database::{
    models::{EmbeddingConfig, LLMConfig, Space},
    DbPool,
};
use anyhow::{anyhow, Context, Ok, Result};
use sqlx::types::Json;

pub async fn create_space(
    pool: &DbPool,
    name: &str,
    llm_config: LLMConfig,
    embedding_config: EmbeddingConfig,
) -> Result<i64> {
    let llm_config_json = Json(llm_config);
    let embedding_config_json = Json(embedding_config);
    let record = sqlx::query!(
        r#"
        INSERT INTO spaces (name, llm_config, embedding_config)
        VALUES (?, ?, ?)
        RETURNING id
        "#,
        name,
        llm_config_json,
        embedding_config_json
    )
    .fetch_one(pool)
    .await?;

    Ok(record.id)
}

pub async fn get_all_spaces(pool: &DbPool) -> Result<Vec<Space>> {
    let res = sqlx::query_as::<_, Space>("SELECT * FROM spaces ORDER BY id DESC")
        .fetch_all(pool)
        .await?;

    Ok(res)
}

pub async fn add_root(pool: &DbPool, space_id: i64, path: &str) -> Result<i64> {
    let record = sqlx::query!(
        "INSERT INTO indexed_roots (space_id, path) VALUES (?, ?) RETURNING id",
        space_id,
        path
    )
    .fetch_one(pool)
    .await?;

    Ok(record.id.context("failed to retrieve inserted root ID")?)
}
