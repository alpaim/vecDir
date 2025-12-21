use anyhow::Context;
use tauri::State;

use crate::{
    database::models::VectorSearchResult,
    search::{self},
    state::AppState,
};

#[tauri::command]
#[specta::specta]
pub async fn search_by_emdedding(
    state: State<'_, AppState>,
    space_id: i32,
    query: String,
    limit: i32,
) -> Result<Vec<VectorSearchResult>, ()> {
    let result = search::embedding::search_by_emdedding(
        &state.db,
        space_id,
        query,
        limit,
    )
    .await
    .context("failed to get search result by emdedding")
    .unwrap();

    Ok(result)
}
