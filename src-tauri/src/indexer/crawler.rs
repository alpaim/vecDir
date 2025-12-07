use std::path::PathBuf;

use crate::database::{files::upsert_file, DbPool};
use anyhow::{Context, Result};
use chrono::DateTime;
use ignore::WalkBuilder;

pub async fn scan_root(pool: &DbPool, root_id: i64, root_path: &PathBuf) -> Result<()> {
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
        let file_size = metadata.len() as i64;

        let modification_time = metadata
            .modified()
            .map(DateTime::from)
            .unwrap_or_else(|_| DateTime::from_timestamp(0, 0).unwrap());

        let filename = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let file_extension = path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        upsert_file(
            pool,
            root_id,
            path_string,
            filename,
            file_extension,
            file_size,
            modification_time,
        )
        .await
        .context("database upsert failed")?;
    }

    Ok(())
}
