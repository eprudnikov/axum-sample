use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Order {
    pub id: Uuid,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<Utc>
}

impl Order {
    pub fn new() -> Self {
        Order {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}