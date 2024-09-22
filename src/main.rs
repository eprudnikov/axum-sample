mod order;

use axum::Router;

#[tokio::main]
async fn main() {
    let app =  setup_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// The function sequentially passes the router to the feature-specific setup functions so that each
/// feature-module does its own things.
fn setup_router() -> Router {
    let app = Router::new();
    order::setup_router(app)
}

