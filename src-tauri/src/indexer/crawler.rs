use crate::database::{
    files::{
        delete_files_by_paths, get_all_files_in_root, upsert_files_batch, UpsertFile,
        UpsertFilesBatch,
    },
    DbPool,
};
use anyhow::{Context, Result};
use chrono::DateTime;
use ignore::WalkBuilder;

// TODO: optimize this function. consider using tokio channels https://docs.rs/tokio/latest/tokio/sync/mpsc/fn.channel.html

pub async fn scan_root(pool: &DbPool, root_id: i32, root_path: &String) -> Result<()> {
    let mut existing_db_paths = get_all_files_in_root(pool, root_id)
        .await
        .context("failed to get existing paths for deletion check")?;

    let mut files: UpsertFilesBatch = Vec::new();

    let walker = WalkBuilder::new(root_path)
        .hidden(false)
        .git_ignore(true)
        .ignore(true)
        .build();

    for result in walker {
        let entry = match result {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("error reading file: {:?}", err);
                continue;
            }
        };

        let path = entry.path();

        // currently scanner skips directories
        if !path.is_file() {
            continue;
        }

        let metadata = entry.metadata().context("failed to read file metadata")?;

        let path_string = path.to_string_lossy().to_string();

        // remove file path from hashset if path found
        existing_db_paths.remove(&path_string);

        let file_size = metadata.len() as u32;

        let modification_time = metadata
            .modified()
            .map(DateTime::from)
            .unwrap_or_else(|_| DateTime::from_timestamp(0, 0).unwrap());

        let filename = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();
        let file_extension = path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();

        files.push(UpsertFile {
            root_id: root_id,
            path: path_string,
            filename: filename,
            file_extension: file_extension,
            size: file_size,
            modified: modification_time,
        });
    }

    upsert_files_batch(pool, files)
        .await
        .context("failed to upsert batch of indexed files")?;

    // deleting removed files

    let paths_to_delete: Vec<String> = existing_db_paths.into_iter().collect();

    if !paths_to_delete.is_empty() {
        delete_files_by_paths(pool, root_id, paths_to_delete)
            .await
            .context("failed to delete missing files")?;
    }

    Ok(())
}
