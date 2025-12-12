use anyhow::{anyhow, bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use usearch::{Index, IndexOptions};

use crate::vector_store::VectorStore;

// TODO: IMPORTANT! need to handle mutable nature of self in CPP usearch calls!
impl VectorStore {
    pub fn init(app_dir: &Path, space_id: i32, options: IndexOptions) -> Result<Self> {
        let spaces_dir = app_dir.join("spaces");

        if !spaces_dir.exists() {
            fs::create_dir_all(&spaces_dir).context("failed to create space vector directory")?;
        }

        let index_path = spaces_dir.join(format!("space_{}.usearch", space_id));

        let index_path_str = index_path
            .to_str()
            .ok_or_else(|| anyhow!("invalid path encoding for index path: {:?}", index_path))?;

        let index =
            Index::new(&options).map_err(|e| anyhow!("failed to create usearch index: {}", e))?;

        // add size check to use index.view if needed: https://docs.rs/usearch/latest/usearch/struct.Index.html#method.view
        if index_path.exists() {
            index
                .load(index_path_str)
                .map_err(|e| anyhow!("failed to load usearch index from disk: {}", e))?;
        } else {
            index
                .save(index_path_str)
                .map_err(|e| anyhow!("failed to save new usearch index to disk: {}", e))?;
        }

        Ok(Self {
            space_id,
            index,
            path: index_path,
        })
    }

    pub fn save(&self) -> Result<()> {
        let path_str = self
            .path
            .to_str()
            .ok_or_else(|| anyhow!("invalid path encoding for save: {:?}", self.path))?;

        self.index
            .save(path_str)
            .map_err(|e| anyhow!("failed to save usearch index: {}", e))?;

        Ok(())
    }

    pub fn add(&self, key: u64, vector: &[f32]) -> Result<()> {
        self.index.add(key, vector).map_err(|e| {
            anyhow!(
                "failed to add vector to usearch index (key: {}): {}",
                key,
                e
            )
        })?;

        Ok(())
    }

    pub fn search(&self, vector: &[f32], count: usize) -> Result<Vec<(u64, f32)>> {
        if vector.len() != self.index.dimensions() {
            bail!(
                "dimensions mismatch: expected {}, got {}",
                self.index.dimensions(),
                vector.len()
            );
        }

        let results = self
            .index
            .search(vector, count)
            .map_err(|e| anyhow!("search failed: {}", e))?;

        let matches: Vec<(u64, f32)> = results
            .keys
            .iter()
            .zip(results.distances.iter())
            .map(|(&key, &dist)| (key, dist))
            .collect();

        Ok(matches)
    }

    pub fn remove(&self, key: u64) -> Result<()> {
        self.index
            .remove(key)
            .map_err(|e| anyhow!("failed to remove key {} from usearch index: {}", key, e))?;

        Ok(())
    }

    pub fn count(&self) -> usize {
        self.index.size()
    }

    pub fn contains(&self, key: u64) -> bool {
        self.index.contains(key)
    }
}
