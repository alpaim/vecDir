pub use anyhow::Context;
use anyhow::{Ok, Result};
use std::collections::HashMap;

use crate::{
    ai::{vecbox::VecboxClient, AI},
    database::{self, models::EmbeddingBackendType, models::VectorSearchResult, DbPool},
};

pub async fn search_by_emdedding(
    db: &DbPool,
    space_id: i32,
    query: String,
    limit: i32,
) -> Result<Vec<VectorSearchResult>> {
    let space = database::spaces::get_space_by_id(db, space_id).await?;

    let embedding_config = space.embedding_config.0;

    let embedding = match embedding_config.backend {
        EmbeddingBackendType::VecBox => {
            let vecbox_client = VecboxClient::new(&embedding_config.api_base_url, &embedding_config.model)
                .context("failed to create vecbox client")?;
            
            let raw_embedding = vecbox_client
                .create_text_embedding(&query)
                .await
                .context("failed to get vecbox embedding")?;
            
            vecbox_client
                .prepare_matroshka(raw_embedding, 768)
                .context("failed to prepare matroshka embedding")?
        }
        EmbeddingBackendType::OpenAICompat => {
            let ai_client = AI::new(&embedding_config.api_base_url, &embedding_config.api_key)
                .context("failed to create openai client")?;

            let embedding_response = ai_client
                .create_embedding(query, embedding_config.model.clone())
                .await
                .context("failed to get embedding response")?;

            let embedding_raw = embedding_response
                .data
                .first()
                .context("failed to get embedding from embedding response")?
                .embedding
                .clone();

            ai_client
                .prepare_matroshka(embedding_raw, 768)
                .context("failed to prepare embedding")?
        }
    };

    // Temp fix to oversample for better results
    // Needs to be adjusted according to config and heuristic 
    let oversample_limit = limit * 100;
    
    let chunks = database::chunks::search_similar_chunks(db, space_id, embedding, oversample_limit)
        .await
        .context("failed to search similar chunks")?;

    let mut file_map: HashMap<i32, VectorSearchResult> = HashMap::new();
    
    for chunk in chunks {
        file_map
            .entry(chunk.file_id)
            .and_modify(|existing| {
                if chunk.distance < existing.distance {
                    *existing = chunk.clone();
                }
            })
            .or_insert(chunk);
    }

    let mut results: Vec<VectorSearchResult> = file_map.into_values().collect();
    results.sort_by(|a, b| {
        a.distance.partial_cmp(&b.distance).unwrap_or(std::cmp::Ordering::Equal)
    });
    results.truncate(limit as usize);

    Ok(results)
}
