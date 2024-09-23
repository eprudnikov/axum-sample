mod orders;
mod configuration;

use crate::configuration::database::setup_db_pool;
use axum::Router;
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = setup_db_pool();
    // TODO what does Arc mean?
    let app_state = Arc::new(AppState { db: pool.await.clone() });
    let router = setup_router()
        .with_state(app_state);

    println!("🚀 Server started successfully on port 3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

/// The function sequentially passes the router to the feature-specific setup functions so that each
/// feature-module does its own things.
fn setup_router() -> Router<Arc<AppState>> {
    let app = Router::new();
    orders::setup_router(app)
}

