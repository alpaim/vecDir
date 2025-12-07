use std::iter::zip;
use std::path::{Path, PathBuf};
use std::{error::Error, fs};
use anyhow::{Ok, Result, bail};
use usearch::{Index, IndexOptions};

use crate::vector_store::VectorStore;

impl VectorStore {
    pub fn init(app_dir: &Path, space_id: i64, options: IndexOptions) -> Result<Self> {
        let spaces_dir = app_dir.join("spaces");
        if !spaces_dir.exists() {
            fs::create_dir_all(&spaces_dir).expect("failed to create space vector dir");
        }

        let index_path = spaces_dir.join(format!("space_{}.usearch", space_id));
        let index_path_str = index_path.to_str().expect("invalid path encoding");

        let index = Index::new(&options).expect("failed to create usearch index");

        // add size check to use index.view if needed: https://docs.rs/usearch/latest/usearch/struct.Index.html#method.view
        if index_path.exists() {
            index.load(index_path_str).expect("failed to load usearch index from disk");
        } else {
            index.save(index_path_str).expect("failed to save usearch index to disk")
        }

        Ok(Self { space_id, index, path: index_path })
    }

    pub fn save(&self) -> Result<()> {
        let path_str = self.path.to_str().expect("invalid path");
        self.index.save(path_str).expect("failed to save usearch index to disk");

        Ok(())
    }

    pub fn add(&self, key: u64, vector: &[f32]) -> Result<()> {
        self.index.add(key, vector).expect("failed to add vector to usearch index");

        Ok(())
    }

    pub fn search(&self, vector: &[f32], count: usize) -> Result<Vec<(u64, f32)>> {
        if vector.len() != self.index.dimensions() {
            bail!("dimensions missmatch: expected: {}, got {}", self.index.dimensions(), vector.len());
        }

        let results = self.index.search(vector, count).expect("search failed");

        let matches: Vec<(u64, f32)> = results.keys.iter() 
            .zip(results.distances.iter())
            .map(|(&key, &dist)| (key, dist))
            .collect();
        
        Ok(matches)
    }

    pub fn remove(&self, key: u64) -> Result<()> {
        self.index.remove(key).expect("failed to remove key from usearch index");

        Ok(())
    }

    pub fn count(&self) -> usize {
        self.index.size()
    }

    pub fn contains(&self, key: u64) -> bool {
        self.index.contains(key)
    }
}