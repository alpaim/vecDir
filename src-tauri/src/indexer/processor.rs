use std::fmt::format;

use anyhow::{Context, Result};
use mime_guess::mime;
use tauri::{AppHandle, Emitter};
use tauri_specta::Event;

use crate::{
    ai::{vecbox::VecboxClient, AI},
    database::{
        self,
        chunks::{add_chunk, add_chunks_batch, AddFileChunk},
        files::{
            get_pending_files_for_space, mark_file_as_indexed, mark_file_as_indexed_batch,
            MarkFileAsIndexed,
        },
        models::{EmbeddingBackendType, LLMConfig},
        spaces::get_space_by_id,
        DbPool,
    },
    status::{
        self,
        events::{StatusEvent, StatusType},
    },
};

async fn process_image_vecbox(
    image_path: &str,
    vecbox_client: &VecboxClient,
) -> Result<(Vec<f32>, String)> {
    let ai = AI::new("", "").unwrap();
    let image_data_url = ai.image_to_base64(image_path).await?;

    let embedding = vecbox_client
        .create_image_embedding(&image_data_url)
        .await
        .context("failed to create vecbox image embedding")?;

    let filename = std::path::Path::new(image_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("image")
        .to_string();

    Ok((embedding, format!("image: {}", filename)))
}

async fn process_text_vecbox(
    file_path: &str,
    vecbox_client: &VecboxClient,
) -> Result<(Vec<f32>, String)> {
    let content = tokio::fs::read_to_string(file_path).await.unwrap_or_default();

    let filename = std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file")
        .to_string();

    let text_to_embed = if content.trim().is_empty() {
        format!("file: {}", filename)
    } else {
        content.chars().take(1000).collect::<String>()
    };

    let embedding = vecbox_client
        .create_text_embedding(&text_to_embed)
        .await
        .context("failed to create vecbox text embedding")?;

    Ok((embedding, text_to_embed))
}

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

    let llm_config = space.llm_config.0;
    let embedding_config = space.embedding_config.0;

    match embedding_config.backend {
        EmbeddingBackendType::VecBox => {
            process_space_vecbox(app_handle, pool, space_id, limit, &embedding_config).await
        }
        EmbeddingBackendType::OpenAICompat => {
            process_space_standard(app_handle, pool, space_id, limit, &llm_config, &embedding_config).await
        }
    }
}

async fn process_space_vecbox(
    app_handle: AppHandle,
    pool: &DbPool,
    space_id: i32,
    limit: i32,
    embedding_config: &crate::database::models::EmbeddingConfig,
) -> Result<()> {
    let vecbox_client = VecboxClient::new(&embedding_config.api_base_url, &embedding_config.model)
        .context("failed to create vecbox client")?;

    let pending_files = get_pending_files_for_space(pool, space_id, limit)
        .await
        .context("failed to get pending indexed files")?;

    if pending_files.is_empty() {
        return Ok(());
    }

    let total_files = pending_files.len() as i32;
    let mut chunks_to_add: Vec<AddFileChunk> = Vec::new();
    let mut updates_to_add: Vec<MarkFileAsIndexed> = Vec::new();

    for (i, file) in pending_files.iter().enumerate() {
        let file_id = file.id;
        let file_path = &file.absolute_path;
        let guess = mime_guess::from_path(file_path);
        let mime_type = guess.first_or_octet_stream();

        let result = match mime_type.type_() {
            mime::IMAGE => process_image_vecbox(file_path, &vecbox_client).await,
            _ => process_text_vecbox(file_path, &vecbox_client).await,
        };

        match result {
            Ok((embedding, content)) => {
                let truncated_embedding = vecbox_client
                    .prepare_matroshka(embedding, 768)
                    .context("failed to prepare matroshka to 768 dim")?;

                if !content.trim().is_empty() {
                    let chunk = AddFileChunk {
                        file_id,
                        chunk_index: 0,
                        content: content.clone(),
                        start_char_idx: None,
                        end_char_idx: None,
                        embedding: truncated_embedding,
                    };
                    chunks_to_add.push(chunk);

                    let update = MarkFileAsIndexed {
                        file_id,
                        description: Some(content),
                    };
                    updates_to_add.push(update);
                }

                StatusEvent {
                    status: StatusType::Processing,
                    message: Some("Processing Space".to_string()),
                    total: Some(total_files),
                    processed: Some(i as i32),
                }
                .emit(&app_handle)?;
            }
            Err(e) => {
                println!("failed to process file {:?}: {}", file_path, e);
                continue;
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

async fn process_space_standard(
    app_handle: AppHandle,
    pool: &DbPool,
    space_id: i32,
    limit: i32,
    llm_config: &LLMConfig,
    embedding_config: &crate::database::models::EmbeddingConfig,
) -> Result<()> {
    let mut descriptions: Vec<String> = Vec::new();
    let mut processed_files: Vec<i32> = Vec::new();

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
            mime::IMAGE => process_image(&file.absolute_path, &ai_client_llm, llm_config)
                .await
                .context("failed to process image"),
            _ => process_default(&file.absolute_path, &ai_client_llm, llm_config)
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
