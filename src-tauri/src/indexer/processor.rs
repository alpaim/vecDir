use std::fmt::format;

use anyhow::{Context, Ok, Result};
use mime_guess::mime;
use tauri::{AppHandle, Emitter};
use tauri_specta::Event;

use crate::{
    ai::{self, embedding, AI},
    database::{
        self,
        chunks::{add_chunk, add_chunks_batch, AddFileChunk},
        files::{
            get_pending_files_for_space, mark_file_as_indexed, mark_file_as_indexed_batch,
            MarkFileAsIndexed,
        },
        models::{EmbeddingConfig, LLMConfig, Space},
        spaces::get_space_by_id,
        DbPool,
    },
    status::{
        self,
        events::{StatusEvent, StatusType},
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
            &llm_config.image_processing_prompt.system_prompt,
            &llm_config.image_processing_prompt.user_prompt,
            &llm_config.model,
        )
        .await
        .context(format!(
            "failed to describe image in processor: {:?}",
            image_path
        ))?;

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
    app_handle: AppHandle,
    pool: &DbPool,
    space_id: i32,
    limit: i32,
) -> Result<()> {
    let space = get_space_by_id(pool, space_id)
        .await
        .context("failed to get space in process_space")?;
    let embedding_config = &space.embedding_config.0;

    if embedding_config.multimodal {
        process_space_multimodal_embedding(app_handle, pool, space, limit)
            .await
            .context("failed to process space using multimodal embedding")
            .unwrap();
    } else {
        process_space_llm(app_handle, pool, space, limit)
            .await
            .context("failed to process space using LLM description embedding")
            .unwrap();
    }

    Ok(())
}

pub async fn process_space_llm(
    app_handle: AppHandle,
    pool: &DbPool,
    space: Space,
    limit: i32,
) -> Result<()> {
    let mut descriptions: Vec<String> = Vec::new();
    let mut processed_files: Vec<i32> = Vec::new();

    let space_id = space.id;

    let llm_config = space.llm_config.0;
    let embedding_config = space.embedding_config.0;

    let ai_client_llm = AI::new(&llm_config.api_base_url, &llm_config.api_key)
        .context("failed to create openai client")?;

    let ai_client_emdedding = AI::new(&embedding_config.api_base_url, &embedding_config.api_key)
        .context("failed to create openai client")?;

    let pending_files = get_pending_files_for_space(pool, space_id, limit)
        .await
        .context("failed to get pending indexed files")?;

    if pending_files.is_empty() {
        return Ok(());
    }

    let total_files = pending_files.len() as i32;

    for (i, file) in pending_files.iter().enumerate() {
        let file_id = file.id;
        let file_path = file.absolute_path.clone();
        let guess = mime_guess::from_path(&file_path);
        let mime_type = guess.first_or_octet_stream();

        let description_result = match mime_type.type_() {
            mime::IMAGE => process_image(&file.absolute_path, &ai_client_llm, &llm_config)
                .await
                .context("failed to process image"),
            _ => process_default(&file.absolute_path, &ai_client_llm, &llm_config)
                .await
                .context("failed to process default file"),
        };

        if description_result.is_err() {
            println!("failed to process file: {:?}", &file_path);
            continue;
        }

        let description = description_result.unwrap();

        descriptions.push(description);
        processed_files.push(file_id);

        StatusEvent {
            status: StatusType::Processing,
            message: Some("Processing Space".to_string()),
            total: Some(total_files),
            processed: Some(i as i32),
        }
        .emit(&app_handle)?;
    }

    const BATCH_SIZE: usize = 50;

    let mut chunks_to_add: Vec<AddFileChunk> = Vec::new();
    let mut updates_to_add: Vec<MarkFileAsIndexed> = Vec::new();

    let chunks_iter = descriptions
        .chunks(BATCH_SIZE)
        .zip(processed_files.chunks(BATCH_SIZE));

    for (desc_chunk, file_id_chunk) in chunks_iter {
        let embeddings_response = ai_client_emdedding
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
            let embedding_result = ai_client_emdedding
                .prepare_matroshka(embedding_item.embedding.clone(), 768)
                .context("failed to prepare matroshka to 768 dim");

            if embedding_result.is_err() {
                println!("failed to create batch embedding {:?}", embedding_result);
                continue;
            }

            let embedding = embedding_result.unwrap();

            if !is_content_empty {
                let chunk = AddFileChunk {
                    file_id: file_id,

                    chunk_index: 0,
                    content: Some(content.clone()),

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

                StatusEvent {
                    status: StatusType::Processing,
                    message: Some("Embedding Results".to_string()),
                    total: Some(processed_files.len() as i32),
                    processed: Some(chunks_to_add.len() as i32),
                }
                .emit(&app_handle)?;
            }
        }
    }

    add_chunks_batch(pool, chunks_to_add)
        .await
        .context("failed to add chunks in batch in process_space")?;

    mark_file_as_indexed_batch(pool, updates_to_add)
        .await
        .context("failed to update file indexing status in batch in process_space")?;

    StatusEvent {
        status: StatusType::Idle,
        message: None,
        total: None,
        processed: None,
    }
    .emit(&app_handle)?;

    StatusEvent {
        status: StatusType::Notification,
        message: Some("Space Processed".to_string()),
        total: None,
        processed: None,
    }
    .emit(&app_handle)?;

    Ok(())
}

pub async fn process_space_multimodal_embedding(
    app_handle: AppHandle,
    pool: &DbPool,
    space: Space,
    limit: i32,
) -> Result<()> {
    let space_id = space.id;

    let embedding_config = space.embedding_config.0;

    let ai_client_emdedding = AI::new(&embedding_config.api_base_url, &embedding_config.api_key)
        .context("failed to create openai client")?;

    let pending_files = get_pending_files_for_space(pool, space_id, limit)
        .await
        .context("failed to get pending indexed files")?;

    if pending_files.is_empty() {
        return Ok(());
    }

    const BATCH_SIZE: usize = 50;
    let total_files = pending_files.len() as i32;

    for (i, chunk) in pending_files.chunks(BATCH_SIZE).enumerate() {
        let mut inputs: Vec<ai::embedding::multimodal_llamacpp::MultimodalEmbeddingInput> =
            Vec::new();

        for file in chunk.iter() {
            let file_path = file.absolute_path.clone();
            let guess = mime_guess::from_path(&file_path);
            let mime_type = guess.first_or_octet_stream();

            match mime_type.type_() {
                mime::IMAGE => {
                    let image_url = ai_client_emdedding
                        .image_to_base64(&file_path)
                        .await
                        .context("failed to get image base64 url")?;
                    let input = ai::embedding::multimodal_llamacpp::MultimodalEmbeddingInput {
                        text: Some(
                            embedding_config
                                .image_processing_prompt
                                .system_prompt
                                .clone(),
                        ),
                        image_url: Some(image_url),
                    };

                    inputs.push(input);
                }
                _ => {}
            };
        }

        let embeddings_response = ai_client_emdedding
            .create_embeddings_batch_qwen3vl_llamacpp(inputs, embedding_config.model.clone())
            .await
            .context("failed to create multimodal embeddings in batch during processing space")?;

        if embeddings_response.data.is_empty() {
            continue;
        }

        let mut chunks_to_add: Vec<AddFileChunk> = Vec::new();
        let mut updates_to_add: Vec<MarkFileAsIndexed> = Vec::new();

        for embedding_item in embeddings_response.data {
            let idx = embedding_item.index as usize;

            if idx >= chunk.len() {
                continue;
            }

            let file = &chunk[idx];
            let file_id = file.id;

            // TODO: make default dimension const
            let embedding_result = ai_client_emdedding
                .prepare_matroshka(embedding_item.embedding.clone(), 768)
                .context("failed to prepare matroshka to 768 dim");

            if embedding_result.is_err() {
                println!("failed to create batch embedding {:?}", embedding_result);
                continue;
            }

            let embedding = embedding_result.unwrap();

            let file_chunk = AddFileChunk {
                file_id: file_id,

                chunk_index: 0,
                content: None,

                start_char_idx: None,
                end_char_idx: None,

                embedding: embedding,
            };

            chunks_to_add.push(file_chunk);

            let update: MarkFileAsIndexed = MarkFileAsIndexed {
                file_id: file_id,
                description: None,
            };

            updates_to_add.push(update);
        }

        add_chunks_batch(pool, chunks_to_add)
            .await
            .context("failed to add chunks in batch in process_space")?;

        mark_file_as_indexed_batch(pool, updates_to_add)
            .await
            .context("failed to update file indexing status in batch in process_space")?;

        StatusEvent {
            status: StatusType::Processing,
            message: Some("Processing Space".to_string()),
            total: Some(total_files),
            processed: Some(i as i32),
        }
        .emit(&app_handle)?;
    }

    StatusEvent {
        status: StatusType::Idle,
        message: None,
        total: None,
        processed: None,
    }
    .emit(&app_handle)?;

    StatusEvent {
        status: StatusType::Notification,
        message: Some("Space Processed".to_string()),
        total: None,
        processed: None,
    }
    .emit(&app_handle)?;

    Ok(())
}
