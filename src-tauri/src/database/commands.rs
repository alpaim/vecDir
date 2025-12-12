use anyhow::Context;
use tauri::State;
use tokio::sync::Mutex;

use crate::{
    database::{
        self,
        models::{AppConfig, EmbeddingConfig, FileMetadata, IndexedRoot, LLMConfig, Space},
    },
    state::AppState,
};

// CONFIG

#[tauri::command]
#[specta::specta]
pub async fn get_config(state: State<'_, Mutex<AppState>>) -> Result<AppConfig, ()> {
    let state = state.lock().await;

    let config = database::config::get_config(&state.db)
        .await
        .context("failed to get config in command")
        .unwrap();

    Ok(config)
}

#[tauri::command]
#[specta::specta]
pub async fn update_config(state: State<'_, Mutex<AppState>>, config: AppConfig) -> Result<(), ()> {
    let state = state.lock().await;

    let result = database::config::update_config(&state.db, config)
        .await
        .context("failed to update config in command")
        .unwrap();

    Ok(result)
}

// SPACES

#[tauri::command]
#[specta::specta]
pub async fn create_space(
    state: State<'_, Mutex<AppState>>,
    name: &str,
    llm_config: LLMConfig,
    embedding_config: EmbeddingConfig,
) -> Result<i32, ()> {
    let state: tokio::sync::MutexGuard<'_, AppState> = state.lock().await;

    let space = database::spaces::create_space(&state.db, name, llm_config, embedding_config)
        .await
        .context("failed to create space in command")
        .unwrap();

    Ok(space as i32)
}

#[tauri::command]
#[specta::specta]
pub async fn get_space_by_id(state: State<'_, Mutex<AppState>>, space_id: i32) -> Result<Space, ()> {
    let state: tokio::sync::MutexGuard<'_, AppState> = state.lock().await;

    let spaces = database::spaces::get_space_by_id(&state.db, space_id)
        .await
        .context("failed to get space by id in command")
        .unwrap();

    Ok(spaces)
}

#[tauri::command]
#[specta::specta]
pub async fn get_all_spaces(state: State<'_, Mutex<AppState>>) -> Result<Vec<Space>, ()> {
    let state: tokio::sync::MutexGuard<'_, AppState> = state.lock().await;

    let spaces = database::spaces::get_all_spaces(&state.db)
        .await
        .context("failed to get all spaces in command")
        .unwrap();

    Ok(spaces)
}

#[tauri::command]
#[specta::specta]
pub async fn add_root(
    state: State<'_, Mutex<AppState>>,
    space_id: i32,
    path: &str,
) -> Result<i32, ()> {
    let state: tokio::sync::MutexGuard<'_, AppState> = state.lock().await;

    let root = database::spaces::add_root(&state.db, space_id, path)
        .await
        .context("failed to add root to space in command")
        .unwrap();

    Ok(root)
}

#[tauri::command]
#[specta::specta]
pub async fn get_roots_by_space_id(state: State<'_, Mutex<AppState>>, space_id: i32) -> Result<Vec<IndexedRoot>, ()> {
    let state: tokio::sync::MutexGuard<'_, AppState> = state.lock().await;

    let roots = database::spaces::get_roots_by_space_id(&state.db, space_id)
        .await
        .context("failed to get roots by space_id in command")
        .unwrap();

    Ok(roots)
}

// FILES

#[tauri::command]
#[specta::specta]
pub async fn get_files_by_ids(
    state: State<'_, Mutex<AppState>>,
    ids: Vec<i32>,
) -> Result<Vec<FileMetadata>, ()> {
    let state: tokio::sync::MutexGuard<'_, AppState> = state.lock().await;

    let files = database::files::get_files_by_ids(&state.db, ids)
        .await
        .context("failed to get files by ids in command")
        .unwrap();

    Ok(files)
}
