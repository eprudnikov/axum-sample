use crate::AppState;
use crate::orders::order::Order;

pub async fn insert(order: &Order, app_state: &AppState) -> bool {
    match sqlx::query(
        "INSERT INTO orders (id, created_at, updated_at)
        VALUES ($1, $2, $3)")
        .bind(order.id)
        .bind(order.created_at)
        .bind(order.updated_at)
        .execute(&app_state.db).await {
        // TODO map to Result model or similar one
        Ok(result ) => {
            println!("Order {} is created", order.id);
            true
        },
        Err(err) => {
            eprintln!("{}", err);
            false
        }
    }
}