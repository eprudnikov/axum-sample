mod order;
mod repository;

use crate::AppState;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use std::sync::Arc;
use axum::extract::State;
use crate::orders::order::Order;
use crate::orders::repository::insert;

pub fn setup_router(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route("/orders", get(get_orders)
        .post(post_order))
}

async fn get_orders() -> Json<Vec<Order>> {
    Json(vec![Order::new(), Order::new()])
}

async fn post_order(State(state): State<Arc<AppState>>) -> Json<Order> {
    let new_order = Order::new();
    insert(&new_order, &state).await;
    Json(new_order)
}
