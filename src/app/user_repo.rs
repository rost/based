use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct User {
    #[serde(skip_deserializing)]
    id: i64,
    name: String,
}

pub async fn create_user(pool: &SqlitePool, payload: &User) -> Result<User, sqlx::Error> {
    let id = sqlx::query("INSERT INTO users (name) VALUES (?1)")
        .bind(&payload.name)
        .execute(pool)
        .await?
        .last_insert_rowid();
    let user = User {
        id,
        name: payload.name.clone(),
    };
    Ok(user)
}

pub async fn get_user(pool: &SqlitePool, id: i32) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT id, name, data FROM users where id = (?1)")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn get_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>("SELECT id, name, data FROM users")
        .fetch_all(pool)
        .await?;
    Ok(users)
}
