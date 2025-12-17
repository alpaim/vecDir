use anyhow::{Context, Ok, Result};
use mime_guess::mime;

use crate::{
    ai::AI,
    database::{
        self,
        chunks::{add_chunk, add_chunks_batch, AddFileChunk},
        files::{get_pending_files_for_space, mark_file_as_indexed_batch, MarkFileAsIndexed},
        models::LLMConfig,
        spaces::get_space_by_id,
        DbPool,
    },
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

pub async fn process_space(pool: &DbPool, ai_client: &AI, space_id: i32, limit: i32) -> Result<()> {
    let mut descriptions: Vec<String> = Vec::new();
    let mut processed_files: Vec<i32> = Vec::new();

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

    let mut chunks_to_add: Vec<AddFileChunk> = Vec::new();
    let mut updates_to_add: Vec<MarkFileAsIndexed> = Vec::new();

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

        for embedding_item in embeddings_response.data {
            let idx = embedding_item.index as usize;

            if idx >= file_id_chunk.len() {
                continue;
            }

            let file_id = file_id_chunk[idx];
            let content = &desc_chunk[idx];
            let is_content_empty = content.trim().is_empty();

            // TODO: make default dimension const
            let embedding = ai_client.prepare_matroshka(embedding_item.embedding.clone(), 768).context("failed to prepare matroshka to 768 dim")?;

            if !is_content_empty {
                let chunk = AddFileChunk {
                    file_id: file_id,

                    chunk_index: 0,
                    content: content.clone(),

                    start_char_idx: None,
                    end_char_idx: None,

                    embedding: embedding,
                };

                chunks_to_add.push(chunk);

                let update: MarkFileAsIndexed = MarkFileAsIndexed {
                    file_id: file_id,
                    description: Some(content.clone()),
                };

                updates_to_add.push(update);
            }
        }
    }

    add_chunks_batch(pool, chunks_to_add)
        .await
        .context("failed to add chunks in batch in process_space")?;

    mark_file_as_indexed_batch(pool, updates_to_add)
        .await
        .context("failed to update file indexing status in batch in process_space")?;

    Ok(())
}
