use sqlx::{Pool, Sqlite, types::Json};
use super::models::AppConfig;

const CONFIG_KEY: &str = "main";

pub async fn get_config(pool: &Pool<Sqlite>) -> Result<AppConfig, sqlx::Error> {
    let record = sqlx::query!(
        "SELECT config as \"config: Json<AppConfig>\" FROM app_config WHERE key = ?",
        CONFIG_KEY
    )
    .fetch_optional(pool) // returnbs Option<Record>
    .await?;

    // If there are no record — returning default config (but dont save to db yet)
    match record {
        Some(r) => Ok(r.config.0), // .0 extracts structure from Json<>
        None => Ok(AppConfig::default()),
    }
}

pub async fn update_config(pool: &Pool<Sqlite>, new_config: AppConfig) -> Result<(), sqlx::Error> {
    let config_json = Json(new_config);

    // UPSERT: If it exists - update, if not - create 
    sqlx::query!(
        r#"
        INSERT INTO app_config (key, config)
        VALUES (?, ?)
        ON CONFLICT(key) DO UPDATE SET
            config = excluded.config
        "#,
        CONFIG_KEY,
        config_json
    )
    .execute(pool)
    .await?;

    Ok(())
}