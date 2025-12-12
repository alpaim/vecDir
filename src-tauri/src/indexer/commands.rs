use tauri::State;
use tokio::sync::Mutex;

use crate::{indexer, state::AppState};

#[tauri::command]
#[specta::specta]
pub async fn index_space(state: State<'_, Mutex<AppState>>, space_id: i32) -> Result<bool, ()> {
    let state = state.lock().await;

    let result = indexer::indexer::index_space(&state.db, space_id).await.unwrap();

    Ok(result)
}