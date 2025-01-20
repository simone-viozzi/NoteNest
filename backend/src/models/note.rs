use serde::{Deserialize, Serialize};
use sqlx::types::time::PrimitiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: uuid::Uuid,
    pub title: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}
