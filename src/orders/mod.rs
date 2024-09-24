mod order;
mod repository;

use crate::AppState;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use std::sync::Arc;
use axum::extract::State;
use crate::orders::order::Order;
use crate::orders::repository::{create, fetch_all};

pub fn setup_router(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route("/orders", get(get_orders)
        .post(post_order))
}

async fn get_orders(State(state): State<Arc<AppState>>) -> Json<Vec<Order>> {
    match fetch_all(&state).await {
        Ok(orders) => {
            println!("Fetch {} orders", orders.len());
            Json(orders)
        },
        Err(err) => {
            eprintln!("Cannot fetch orders: {}", err);
            Json(vec![])
        }
    }
}

async fn post_order(State(state): State<Arc<AppState>>) -> Json<Order> {
    let new_order = Order::new();
    create(&new_order, &state).await;
    Json(new_order)
}
