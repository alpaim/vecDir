use anyhow::{Context, Ok, Result};
use mime_guess::mime;

use crate::{
    ai::AI,
    database::{
        self, files::get_pending_files_for_space, models::LLMConfig, spaces::get_space_by_id,
        DbPool,
    },
    vector_store::{manager::VectorStoreArcMutex, VectorStore},
};

async fn process_image(
    image_path: &String,
    ai_client: &AI,
    llm_config: &LLMConfig,
) -> Result<String> {
    let description = ai_client
        .describe_image_from_file(
            &image_path,
            &llm_config.system_prompt,
            &llm_config.user_prompt,
            &llm_config.model,
        )
        .await
        .context("failed to describe image in processor")?;

    Ok(description)
}

async fn process_default(
    file_path: &String,
    ai_client: &AI,
    llm_config: &LLMConfig,
) -> Result<String> {
    Ok("".to_string())
}

pub async fn process_space(
    pool: &DbPool,
    vector_store: &VectorStoreArcMutex,
    ai_client: &AI,
    limit: i32,
) -> Result<()> {
    let mut descriptions: Vec<String> = Vec::new();
    let mut processed_files: Vec<i32> = Vec::new();

    // get space_id from mutex; lock is temp only within this closure
    let space_id = {
        let guard = vector_store.lock().await;
        guard.space_id
    };

    let space = get_space_by_id(pool, space_id)
        .await
        .context("failed to get space in process_space")?;

    let llm_config = space.llm_config.0;
    let embedding_config = space.embedding_config.0;

    let pending_files = get_pending_files_for_space(pool, space_id, limit)
        .await
        .context("failed to get pending indexed files")?;

    if pending_files.is_empty() {
        return Ok(());
    }

    for file in pending_files {
        let file_id = file.id;
        let file_path = file.absolute_path.clone();
        let guess = mime_guess::from_path(file_path);
        let mime_type = guess.first_or_octet_stream();

        let description = match mime_type.type_() {
            mime::IMAGE => process_image(&file.absolute_path, ai_client, &llm_config)
                .await
                .context("failed to process image")?,
            _ => process_default(&file.absolute_path, ai_client, &llm_config)
                .await
                .context("failed to process default file")?,
        };

        descriptions.push(description);
        processed_files.push(file_id);
    }

    const BATCH_SIZE: usize = 50;

    let chunks_iter = descriptions
        .chunks(BATCH_SIZE)
        .zip(processed_files.chunks(BATCH_SIZE));

    for (desc_chunk, file_id_chunk) in chunks_iter {
        let embeddings_response = ai_client
            .create_embeddings_batch(desc_chunk.to_vec(), embedding_config.model.clone())
            .await
            .context("failed to create embeddings batch during processing space")?;

        if embeddings_response.data.is_empty() {
            continue;
        }

        {
            let guard = vector_store.lock().await;

            for embedding_item in embeddings_response.data {
                if let Some(&file_id) = file_id_chunk.get(embedding_item.index as usize) {
                    guard
                        .index
                        .add(file_id as u64, &embedding_item.embedding)
                        .context(
                            "failed to add embedding to vector store during processing space",
                        )?;
                }
            }
        }

        for &file_id in file_id_chunk {
            database::files::mark_file_as_indexed(pool, file_id)
                .await
                .context("failed to mark file as indexed")?;
            // TODO: add descrtion to database
        }
    }

    Ok(())
}
