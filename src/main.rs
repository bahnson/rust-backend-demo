use axum::{routing::{get},Router};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use tokio::net::TcpListener;

mod users;
use users::api::{get_users, post_user, get_user_by_id};

// Database connection pool
#[derive(Clone)]
pub struct AppState {
   pub db: Arc<Pool<Postgres>>,
}

#[tokio::main]
async fn main() {
    // Set up database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let state = AppState {
        db: Arc::new(db_pool),
    };

    // Define routes
    let app = Router::new()
        .route("/users", get(get_users).post(post_user))
        .route("/users/{id}", get(get_user_by_id))
        .with_state(state);

    // Start server
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 Server running on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}
