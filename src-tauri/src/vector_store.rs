use std::path::PathBuf;

use usearch::Index;

pub mod init;
pub mod manager;

pub struct VectorStore {
    space_id: i64,
    index: Index,
    path: PathBuf
}
