use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct Order {
    pub id: Uuid,
}

pub fn setup_router(router: Router) -> Router {
    router.route("/orders", get(get_orders)
        .post(post_order))
}

async fn get_orders() -> Json<Vec<Order>> {
    Json(vec![Order { id: Uuid::new_v4() },
              Order { id: Uuid::new_v4() }])
}

async fn post_order() {

}
