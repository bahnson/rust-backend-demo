use axum::{
    extract::{Path, State},
    Json,
};
use super::{User, CreateUser};
use super::super::AppState;
use super::db::{select_users, select_user_by_id, insert_user};

// Handler for GET /users
pub async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = select_users(state.db).await;
    Json(users)
}

// Handler for GET /users/{id}
pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Json<Option<User>> {
    let user = select_user_by_id(state.db, user_id).await;
    Json(user)
}

// Handler for POST /users
pub async fn post_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Json<User> {
    let user = insert_user(state.db, payload).await;
    Json(user)
}