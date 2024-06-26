use std::str::FromStr;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let db_uri = if cfg!(test) {
        "sqlite::memory:".to_string()
    } else {
        "sqlite:dev.db".to_string()
    };

    let opts = SqliteConnectOptions::from_str(&db_uri)?
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(opts).await?;
    init_tables(&pool).await?;
    Ok(pool)
}

async fn init_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id    INTEGER PRIMARY KEY AUTOINCREMENT,
            name  TEXT NOT NULL,
            data  BLOB
        )",
    )
    .execute(pool)
    .await?;

    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS _collections (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    TEXT NOT NULL,
            schema  TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}
