use futures::TryStreamExt;
use lancedb::arrow::IntoArrowStream;
use lancedb::connection::Connection;
use lancedb::Table;
use arrow_array::{RecordBatch, RecordBatchIterator};
use arrow_array::{Int64Array, FixedSizeListArray, Float32Array};
use arrow_schema::{DataType, Field};
use lancedb::query::{ExecutableQuery, QueryBase};
use std::sync::Arc;
use crate::vector_database::schema::{VectorSearchResult, get_embeddings_schema};

fn get_table_name(space_id: i64) -> String {
    format!("space_{}", space_id)
}

pub async fn get_or_create_space_table(
    db: &Connection, 
    space_id: i64, 
    dimensions: i32
) -> Result<Table, lancedb::Error> {
    let table_name = get_table_name(space_id);
    
    // Checking if the table exists
    let existing_tables = db.table_names().execute().await?;
    
    if existing_tables.contains(&table_name) {
        return db.open_table(&table_name).execute().await;
    }

    // If not - creating a new one with schema
    // LanceDB requires initial data or schema for initializtion 
    // Making empty RecordBatch for initialization
    let schema = get_embeddings_schema(dimensions);
    let batches = RecordBatchIterator::new(vec![], schema.clone());
    
    db.create_table(&table_name, batches)
        .execute()
        .await
}

pub async fn add_embedding(
    table: &Table,
    file_id: i64,
    vector: Vec<f32>,
    dimensions: i32
) -> Result<(), lancedb::Error> {
    
    let schema = get_embeddings_schema(dimensions);
    
    let id_array = Int64Array::from(vec![file_id]);
    
    let value_data = Float32Array::from(vector);
    
    let vector_array = FixedSizeListArray::new(
        Arc::new(Field::new("item", DataType::Float32, true)),
        dimensions,
        Arc::new(value_data),
        None,
    );

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![Arc::new(id_array), Arc::new(vector_array)],
    ).map_err(|e| lancedb::Error::Runtime { message: e.to_string() })?;

    let batches = RecordBatchIterator::new(
        vec![Ok(batch)].into_iter(),
        schema.clone()
    );

    table.add(batches).execute().await?;
    
    Ok(())
}

pub async fn delete_emdedding(
    db: &Connection,
    space_id: i64,
    file_id_to_delete: i64,
) -> Result<(), lancedb::Error> {
    let talbe_name = get_table_name(space_id);
    let table = db.open_table(&talbe_name).execute().await?;

    let predicate = format!("file_id = {}", file_id_to_delete);

    table.delete(&predicate).await?;

    Ok(())
}

pub async fn search_embeddings(
    db: &Connection,
    space_id: i64,
    query_vector: Vec<f32>,
    limit: usize,
) -> Result<Vec<VectorSearchResult>, lancedb::Error> {
    let table_name = get_table_name(space_id);
    let table = db.open_table(&table_name).execute().await?;

    let mut result_vec: Vec<VectorSearchResult> = Vec::new();

    let mut stream = table
        .query()
        .nearest_to(query_vector)?
        .limit(limit)
        .execute()
        .await?;

    while let Some(batch) = stream.try_next().await? {
        let batch_results: Vec<VectorSearchResult> = serde_arrow::from_record_batch(&batch)
        .map_err(|e| lancedb::Error::Runtime { message: e.to_string() })?;
        
        result_vec.extend(batch_results);
    }

    Ok(result_vec)
}