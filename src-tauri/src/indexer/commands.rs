use anyhow::Context;
use tauri::{AppHandle, State};

use crate::{indexer, state::AppState};

#[tauri::command]
#[specta::specta]
pub async fn index_space(
    app_handle: AppHandle,
    state: State<'_, AppState>,
    space_id: i32,
) -> Result<bool, ()> {
    let result = indexer::indexer::index_space(app_handle, &state.db, space_id)
        .await
        .unwrap();

    Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn process_space(
    app_handle: AppHandle,
    state: State<'_, AppState>,
    space_id: i32,
) -> Result<(), ()> {
    let result = indexer::processor::process_space(app_handle, &state.db, space_id, 1_000)
        .await
        .unwrap();

    Ok(result)
}
