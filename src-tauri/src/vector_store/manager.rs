use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::{Ok, Result};
use dashmap::{DashMap, Entry};
use usearch::IndexOptions;

use crate::vector_store::VectorStore;

pub type VectorStoreArcMutex = Arc<Mutex<VectorStore>>;
pub type IndiciesMapMutex = DashMap<i64, VectorStoreArcMutex>;

pub struct VectorIndexManager {
    indices: IndiciesMapMutex,
    app_dir: PathBuf,
}

impl VectorIndexManager {
    pub fn new(app_dir: PathBuf) -> Self {
        Self {
            indices: DashMap::new(),
            app_dir,
        }
    }

    pub fn get_index(&self, space_id: i64) -> Result<VectorStoreArcMutex> {
        match self.indices.entry(space_id) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                // Need to make a function to generate index options from space db info
                let options = IndexOptions {
                    dimensions: 1536,
                    metric: usearch::MetricKind::Cos,
                    quantization: usearch::ScalarKind::I8,
                    connectivity: 16,
                    expansion_add: 128,
                    expansion_search: 64,
                    multi: false,
                };

                let store = VectorStore::init(&self.app_dir, space_id, options)?;
                let store_arc: VectorStoreArcMutex = Arc::new(Mutex::new(store));

                entry.insert(store_arc.clone());

                Ok(store_arc)
            }
        }
    }
}
