use anyhow::Context;
use tauri::State;

use crate::{indexer, state::AppState};

#[tauri::command]
#[specta::specta]
pub async fn index_space(state: State<'_, AppState>, space_id: i32) -> Result<bool, ()> {
    let result = indexer::indexer::index_space(&state.db, space_id).await.unwrap();

    Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn process_space(state: State<'_, AppState>, space_id: i32) -> Result<(), ()> {
    let vector_index = state.vector_index_manager.get_index(space_id).unwrap();
    let ai_client = state.openai_client.clone();

    let result = indexer::processor::process_space(&state.db, &vector_index, &ai_client, 1_000).await.unwrap();

    Ok(())
}