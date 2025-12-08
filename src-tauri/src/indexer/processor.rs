use anyhow::{Context, Ok, Result};

use crate::{database::{self, DbPool, files::{get_pending_files_for_space}}, vector_store::VectorStore};

pub async fn process_space(pool: &DbPool, vector_store: &VectorStore, limit: i64) -> Result<()> {
    let pending_files = get_pending_files_for_space(pool, vector_store.space_id, limit)
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