pub use anyhow::Context;
use anyhow::{Ok, Result};

use crate::{
    ai::AI,
    database::{self, models::VectorSearchResult, DbPool},
};

// TODO: Sepa
pub async fn search_by_emdedding(
    db: &DbPool,
    ai_client: &AI,
    space_id: i32,
    query: String,
    limit: i32,
) -> Result<Vec<VectorSearchResult>> {
    let space = database::spaces::get_space_by_id(db, space_id).await?;

    let embedding_config = space.embedding_config.0;

    let embedding_response = ai_client
        .create_embedding(query, embedding_config.model)
        .await
        .context("failed to get embedding response")?;

    let embedding_raw = embedding_response
        .data
        .first()
        .context("failed to get embedding from embedding response")?
        .embedding
        .clone();

    let embedding = ai_client
        .prepare_matroshka(embedding_raw, 768)
        .context("failed to prepare embedding")?;

    let search_response = database::chunks::search_similar_chunks(db, embedding, limit)
        .await
        .context("failed to get files by ids in command")?;

    Ok(search_response)
}
