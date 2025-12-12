use anyhow::{Context, Result};

use crate::{database::{DbPool, spaces::{get_roots_by_space_id, get_space_by_id}}, indexer::crawler::scan_root};

pub async fn index_space(pool: &DbPool, space_id: i32) -> Result<bool> {
    let roots = get_roots_by_space_id(pool, space_id).await.context("failed to get roots by id in indexer")?;

    for root in roots {
        scan_root(pool, root.id, &root.path).await.context("failed to scan root in indexer")?
    }
    
    Ok(true)
}