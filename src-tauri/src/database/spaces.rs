use sqlx::{types::Json};
use crate::database::DbPool;

use super::models::{Space, EmbeddingConfig};

pub async fn create_space(
    pool: &DbPool, 
    name: &str, 
    config: EmbeddingConfig
) -> Result<i64, sqlx::Error> {
    let config_json = Json(config);
    let record = sqlx::query!(
        r#"
        INSERT INTO spaces (name, embedding_config)
        VALUES (?, ?)
        RETURNING id
        "#,
        name,
        config_json
    )
    .fetch_one(pool)
    .await?;

    Ok(record.id)
}

pub async fn get_all_spaces(pool: &DbPool) -> Result<Vec<Space>, sqlx::Error> {
    sqlx::query_as::<_, Space>("SELECT * FROM spaces ORDER BY id DESC")
        .fetch_all(pool)
        .await
}

pub async fn add_root(
    pool: &DbPool, 
    space_id: i64, 
    path: &str
) -> Result<i64, sqlx::Error> {
    let record = sqlx::query!(
        "INSERT INTO indexed_roots (space_id, path) VALUES (?, ?) RETURNING id",
        space_id,
        path
    )
    .fetch_one(pool)
    .await?;

    Ok(record.id.expect("Failed to retrieve inserted root ID"))
}