use sqlx::sqlite::SqlitePool;

use crate::db::init_db;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

impl AppState {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let pool = init_db().await?;

        Ok(Self { db: pool })
    }
}
