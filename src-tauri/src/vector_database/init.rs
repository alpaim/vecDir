use std::path::Path;
use lancedb::connect;
use lancedb::connection::Connection;

pub async fn initialize_vector_database(app_dir: &Path) -> Result<Connection, lancedb::Error> {
    let db_path = app_dir.join("vecDir.lancedb");

    let uri = db_path.to_str().unwrap();
    let db = connect(uri).execute().await?;
    
    Ok(db)
}