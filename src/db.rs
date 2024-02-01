use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let dbname = std::env::var("DBNAME").unwrap_or_else(|_| "test.db".to_string());
    let opts = SqliteConnectOptions::new()
        .filename(&dbname)
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(opts).await?;
    init_tables(&pool).await?;
    Ok(pool)
}

async fn init_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let _ = sqlx::query(
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
