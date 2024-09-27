use sqlx::Error;
use tracing::{error, info};
use crate::orders::order::Order;
use crate::AppState;

/// Return an error message if any.
pub async fn create(order: &Order, app_state: &AppState) -> Option<String> {
    match sqlx::query(
        "INSERT INTO orders (id, created_at, updated_at)
        VALUES ($1, $2, $3)")
        .bind(order.id)
        .bind(order.created_at)
        .bind(order.updated_at)
        .execute(&app_state.db).await {
        Ok(_) => {
            info!("Order {} is created", order.id);
            None
        },
        Err(err) => {
            error!("{}", err);
            Some(err.to_string())
        }
    }
}

pub async fn fetch_all(app_state: &AppState) -> Result<Vec<Order>, Error> {
    sqlx::query_as!(Order, "SELECT * FROM orders")
        .fetch_all(&app_state.db).await
}