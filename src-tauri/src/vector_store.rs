use std::path::PathBuf;

use usearch::Index;

pub mod init;
pub mod manager;

pub struct VectorStore {
    pub space_id: i64,
    pub index: Index,
    pub path: PathBuf
}
