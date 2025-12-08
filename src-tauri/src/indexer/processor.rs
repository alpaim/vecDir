use anyhow::{Context, Ok, Result};

use crate::database::{self, files::get_pending_files, DbPool};

pub async fn processor(pool: &DbPool) -> Result<()> {
    let pending_files = get_pending_files(pool, 10)
        .await
        .context("failed to get pending indexed files")?;

    if pending_files.is_empty() {
        return Ok(());
    }

    for file in pending_files {
        // A. reading content
        // B. generating description by llm; saving it to database
        // C. generating embedding from Result of B
        // D. saving embedding to vector store with specific space_id | how to obtain this space_id?

        database::files::mark_file_as_indexed(pool, file.id)
            .await
            .context("failed to mark file as indexed")?;
    }

    Ok(())
}