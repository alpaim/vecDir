use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::{Ok, Result};
use usearch::IndexOptions;

use crate::vector_store::VectorStore;

pub type VectorStoreArcMutex = Arc<Mutex<VectorStore>>;
pub type IndiciesMapMutex = Mutex<HashMap<i64, VectorStoreArcMutex>>;

pub struct VectorIndexManager {
    indices: IndiciesMapMutex,
    app_dir: PathBuf,
}

impl VectorIndexManager {
    pub fn new(app_dir: PathBuf) -> Self {
        Self {
            indices: Mutex::new(HashMap::new()),
            app_dir,
        }
    }

    pub fn get_index(&self, space_id: i64) -> Result<VectorStoreArcMutex> {
        let mut map = self.indices.lock().unwrap();

        if let Some(store) = map.get(&space_id) {
            return Ok(store.clone());
        }

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

        map.insert(space_id, store_arc.clone());

        Ok(store_arc)
    }
}
