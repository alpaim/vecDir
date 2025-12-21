use super::models::AppConfig;
use anyhow::{Ok, Result};
use sqlx::{prelude::FromRow, types::Json, Pool, Sqlite};

const CONFIG_KEY: &str = "main";

// TODO: make it query! again; fix sqlite-vec issues
// temp fix-hack until query! will not be fixed
#[derive(FromRow)]
struct ConfigRow {
    config: Json<AppConfig>,
}

pub async fn get_config(pool: &Pool<Sqlite>) -> Result<AppConfig> {
    let record = sqlx::query_as::<_, ConfigRow>(
        "SELECT config as \"config: Json<AppConfig>\" FROM app_config WHERE key = ?",
    )
    .bind(CONFIG_KEY)
    .fetch_optional(pool) // returnbs Option<Record>
    .await?;

    // If there are no record — returning default config (but dont save to db yet)
    match record {
        Some(r) => Ok(r.config.0), // .0 extracts structure from Json<>
        None => Ok(AppConfig::default()),
    }
}

pub async fn update_config(pool: &Pool<Sqlite>, new_config: AppConfig) -> Result<()> {
    let config_json = Json(new_config);

    // UPSERT: If it exists - update, if not - create
    sqlx::query(
        r#"
        INSERT INTO app_config (key, config)
        VALUES (?, ?)
        ON CONFLICT(key) DO UPDATE SET
            config = excluded.config
        "#,
    )
    .bind(CONFIG_KEY)
    .bind(config_json)
    .execute(pool)
    .await?;

    Ok(())
}
