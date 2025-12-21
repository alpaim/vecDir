use anyhow::{Context, Result};
use tauri::AppHandle;
use tauri_specta::Event;

use crate::{
    database::{spaces::get_roots_by_space_id, DbPool},
    indexer::crawler::scan_root,
    status::events::{StatusEvent, StatusType},
};

pub async fn index_space(app_handle: AppHandle, pool: &DbPool, space_id: i32) -> Result<bool> {
    let roots = get_roots_by_space_id(pool, space_id)
        .await
        .context("failed to get roots by id in indexer")?;

    StatusEvent {
        status: StatusType::Indexing,
        message: Some("Indexing Space".to_string()),
        total: Some(roots.len() as i32),
        processed: Some(0),
    }
    .emit(&app_handle)?;

    for root in roots {
        scan_root(pool, root.id, &root.path)
            .await
            .context("failed to scan root in indexer")?
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
        message: Some("Space Indexed".to_string()),
        total: None,
        processed: None,
    }.emit(&app_handle)?;

    Ok(true)
}
