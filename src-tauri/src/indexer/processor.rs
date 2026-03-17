use anyhow::{Context, Result};
use mime_guess::mime;
use tauri::AppHandle;
use tauri_specta::Event;

use crate::{
    ai::{vecbox::VecboxClient, AI},
    database::{
        chunks::{add_chunks_batch, AddFileChunk},
        files::{
            get_pending_files_for_space, mark_file_as_indexed_batch,
            MarkFileAsIndexed,
        },
        models::{EmbeddingBackendType, LLMConfig},
        spaces::get_space_by_id,
        DbPool,
    },
    indexer::chunker::{chunk_text, create_empty_chunk, TextChunk},
    status::events::{StatusEvent, StatusType},
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
) -> Result<Vec<(TextChunk, Vec<f32>)>> {
    let content = tokio::fs::read_to_string(file_path)
        .await
        .unwrap_or_default();

    let filename = std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file")
        .to_string();

    if content.trim().is_empty() {
        let chunk = create_empty_chunk(&filename);
        let embedding = vecbox_client
            .create_text_embedding(&chunk.content)
            .await
            .context("failed to create vecbox text embedding")?;
        let truncated = vecbox_client
            .prepare_matroshka(embedding, 768)
            .context("failed to prepare matroshka")?;
        return Ok(vec![(chunk, truncated)]);
    }

    let chunks = chunk_text(&content);
    let mut results: Vec<(TextChunk, Vec<f32>)> = Vec::new();

    for chunk in chunks {
        let embedding = vecbox_client
            .create_text_embedding(&chunk.content)
            .await
            .context("failed to create vecbox text embedding")?;
        let truncated = vecbox_client
            .prepare_matroshka(embedding, 768)
            .context("failed to prepare matroshka")?;
        results.push((chunk, truncated));
    }

    Ok(results)
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
            mime::IMAGE => process_image_vecbox(file_path, &vecbox_client).await.map(|(emb, desc)| vec![(TextChunk { content: desc, start_char_idx: 0, end_char_idx: 0 }, emb)]),
            _ => process_text_vecbox(file_path, &vecbox_client).await,
        };

        match result {
            Ok(processed) => {
                if mime_type.type_() == mime::IMAGE {
                    let (chunk, embedding) = processed.into_iter().next().unwrap();
                    chunks_to_add.push(AddFileChunk {
                        file_id,
                        chunk_index: 0,
                        content: chunk.content.clone(),
                        start_char_idx: None,
                        end_char_idx: None,
                        embedding,
                    });
                    updates_to_add.push(MarkFileAsIndexed {
                        file_id,
                        description: Some(chunk.content),
                    });
                } else {
                    for (chunk_idx, (chunk, embedding)) in processed.into_iter().enumerate() {
                        chunks_to_add.push(AddFileChunk {
                            file_id,
                            chunk_index: chunk_idx as i32,
                            content: chunk.content,
                            start_char_idx: Some(chunk.start_char_idx),
                            end_char_idx: Some(chunk.end_char_idx),
                            embedding,
                        });
                    }
                    updates_to_add.push(MarkFileAsIndexed {
                        file_id,
                        description: None,
                    });
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
        .context("failed to add chunks in batch")?;

    mark_file_as_indexed_batch(pool, updates_to_add)
        .await
        .context("failed to update file indexing status")?;

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
    let ai_client_llm = AI::new(&llm_config.api_base_url, &llm_config.api_key)
        .context("failed to create openai client")?;

    let ai_client_embedding = AI::new(&embedding_config.api_base_url, &embedding_config.api_key)
        .context("failed to create openai client")?;

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

        if mime_type.type_() == mime::IMAGE {
            let description_result = process_image(&file.absolute_path, &ai_client_llm, llm_config)
                .await
                .context("failed to process image");

            match description_result {
                Ok(description) => {
                    let embedding_response = ai_client_embedding
                        .create_embedding(description.clone(), embedding_config.model.clone())
                        .await
                        .context("failed to create embedding for image description")?;

                    let embedding = embedding_response
                        .data
                        .first()
                        .context("failed to get embedding from response")?;

                    let truncated = ai_client_embedding
                        .prepare_matroshka(embedding.embedding.clone(), 768)
                        .context("failed to prepare matroshka")?;

                    chunks_to_add.push(AddFileChunk {
                        file_id,
                        chunk_index: 0,
                        content: description.clone(),
                        start_char_idx: None,
                        end_char_idx: None,
                        embedding: truncated,
                    });

                    updates_to_add.push(MarkFileAsIndexed {
                        file_id,
                        description: Some(description),
                    });
                }
                Err(e) => {
                    println!("failed to process image {:?}: {}", file_path, e);
                    continue;
                }
            }
        } else {
            let content = tokio::fs::read_to_string(file_path)
                .await
                .unwrap_or_default();

            let filename = std::path::Path::new(file_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("file")
                .to_string();

            let chunks = if content.trim().is_empty() {
                vec![create_empty_chunk(&filename)]
            } else {
                chunk_text(&content)
            };

            if chunks.is_empty() {
                continue;
            }

            let chunk_contents: Vec<String> = chunks.iter().map(|c| c.content.clone()).collect();

            let embeddings_response = ai_client_embedding
                .create_embeddings_batch(chunk_contents, embedding_config.model.clone())
                .await
                .context("failed to create embeddings batch")?;

            for (chunk_idx, chunk) in chunks.into_iter().enumerate() {
                let embedding_item = embeddings_response
                    .data
                    .iter()
                    .find(|e| e.index == chunk_idx as u32);

                if let Some(embedding_item) = embedding_item {
                    let truncated = ai_client_embedding
                        .prepare_matroshka(embedding_item.embedding.clone(), 768)
                        .context("failed to prepare matroshka")?;

                    chunks_to_add.push(AddFileChunk {
                        file_id,
                        chunk_index: chunk_idx as i32,
                        content: chunk.content,
                        start_char_idx: Some(chunk.start_char_idx),
                        end_char_idx: Some(chunk.end_char_idx),
                        embedding: truncated,
                    });
                }
            }

            updates_to_add.push(MarkFileAsIndexed {
                file_id,
                description: None,
            });
        }

        StatusEvent {
            status: StatusType::Processing,
            message: Some("Processing Space".to_string()),
            total: Some(total_files),
            processed: Some(i as i32),
        }
        .emit(&app_handle)?;
    }

    add_chunks_batch(pool, chunks_to_add)
        .await
        .context("failed to add chunks in batch")?;

    mark_file_as_indexed_batch(pool, updates_to_add)
        .await
        .context("failed to update file indexing status")?;

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
