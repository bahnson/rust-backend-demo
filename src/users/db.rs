use sqlx::{Pool, Postgres};
use std::sync::Arc;
use super::{CreateUser, User};
pub async fn select_users(db: Arc<Pool<Postgres>>) -> Vec<User> {
    sqlx::query_as!(User, r#"SELECT id, name, email FROM users"#)
        .fetch_all(&*db)
        .await
        .expect("Failed to fetch users")
}

pub async fn select_user_by_id(
    db: Arc<Pool<Postgres>>,
    user_id: i32,
) -> Option<User> {
    sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(&*db)
    .await
    .expect("Failed to fetch user")
}

pub async fn insert_user(
    db: Arc<Pool<Postgres>>,
    payload: CreateUser,
) -> User {
    sqlx::query_as!(
        User,
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        payload.name,
        payload.email
    )
    .fetch_one(&*db)
    .await
    .expect("Failed to insert user")
}