use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::sqlite::SqlitePool;

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Collection {
    #[serde(skip_deserializing)]
    id: i64,
    name: String,
    schema: sqlx::types::Json<serde_json::Value>,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct CollectionEntry {
    #[serde(skip_deserializing)]
    id: i64,
    entry: sqlx::types::Json<serde_json::Value>,
}

// collections
pub async fn list_collections(pool: &SqlitePool) -> Result<Vec<Collection>, sqlx::Error> {
    let collections = sqlx::query_as::<_, Collection>("SELECT id, name, schema FROM _collections")
        .fetch_all(pool)
        .await?;
    Ok(collections)
}

pub async fn create_collection(
    pool: &SqlitePool,
    payload: Collection,
) -> Result<Collection, sqlx::Error> {
    let id = sqlx::query("INSERT INTO _collections (name, schema) VALUES (?1, ?2)")
        .bind(&payload.name)
        .bind(&payload.schema)
        .execute(pool)
        .await?
        .last_insert_rowid();
    let collection = Collection {
        id,
        name: payload.name,
        schema: sqlx::types::Json(json!(payload.schema)),
    };
    Ok(collection)
}

pub async fn view_collection(pool: &SqlitePool, name: String) -> Result<Collection, sqlx::Error> {
    let collection = sqlx::query_as::<_, Collection>(
        "SELECT id, name, schema FROM _collections where name = (?1)",
    )
    .bind(name)
    .fetch_one(pool)
    .await?;
    Ok(collection)
}

pub async fn update_collection(
    pool: &SqlitePool,
    name: String,
    payload: Collection,
) -> Result<Collection, sqlx::Error> {
    let _ = sqlx::query("UPDATE _collections SET name = (?1), schema = (?2) WHERE name = (?3)")
        .bind(&payload.name)
        .bind(&payload.schema)
        .bind(name)
        .execute(pool)
        .await?;
    let collection = Collection {
        id: payload.id,
        name: payload.name,
        schema: sqlx::types::Json(json!(payload.schema)),
    };
    Ok(collection)
}

pub async fn delete_collection(pool: &SqlitePool, name: String) -> Result<(), sqlx::Error> {
    let _ = sqlx::query("DELETE FROM _collections WHERE name = (?1)")
        .bind(name)
        .execute(pool)
        .await?;
    Ok(())
}

// collection records
pub async fn list_collection_records(
    pool: &SqlitePool,
    collection_name: String,
) -> Result<Vec<CollectionEntry>, sqlx::Error> {
    let entries = sqlx::query_as::<_, CollectionEntry>(&format!(
        "SELECT id, entry FROM _collections_{}",
        collection_name
    ))
    .fetch_all(pool)
    .await?;

    Ok(entries)
}

pub async fn create_collection_record(
    pool: &SqlitePool,
    collection_name: String,
    payload: CollectionEntry,
) -> Result<CollectionEntry, sqlx::Error> {
    let _ = sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS _collections_{} (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            entry   TEXT NOT NULL
        )",
        collection_name
    ))
    .execute(pool)
    .await?;

    let id = sqlx::query(&format!(
        "INSERT INTO _collections_{} (entry) VALUES (?1)",
        collection_name
    ))
    .bind(&payload.entry)
    .execute(pool)
    .await?
    .last_insert_rowid();
    let entry = CollectionEntry {
        id,
        entry: sqlx::types::Json(json!(payload.entry)),
    };

    Ok(entry)
}

// get collection entry
pub async fn view_collection_record(
    pool: &SqlitePool,
    collection_name: String,
    id: i64,
) -> Result<CollectionEntry, sqlx::Error> {
    let entry = sqlx::query_as::<_, CollectionEntry>(&format!(
        "SELECT id, entry FROM _collections_{} where id = (?1)",
        collection_name
    ))
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(entry)
}

pub async fn update_collection_record(
    pool: &SqlitePool,
    collection_name: String,
    id: i64,
    payload: CollectionEntry,
) -> Result<CollectionEntry, sqlx::Error> {
    let _ = sqlx::query(&format!(
        "UPDATE _collections_{} SET entry = (?1) WHERE id = (?2)",
        collection_name
    ))
    .bind(&payload.entry)
    .bind(id)
    .execute(pool)
    .await?;
    let entry = CollectionEntry {
        id,
        entry: sqlx::types::Json(json!(payload.entry)),
    };

    Ok(entry)
}

pub async fn delete_collection_record(
    pool: &SqlitePool,
    collection_name: String,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query(&format!(
        "DELETE FROM _collections_{} WHERE id = (?1)",
        collection_name
    ))
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}
