use std::path::PathBuf;

use usearch::Index;

pub mod init;
pub mod manager;

pub struct VectorStore {
    pub space_id: i32,
    pub index: Index,
    pub path: PathBuf
}
