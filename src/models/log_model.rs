use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Log {
    pub id: i32,
    pub schema_id: Uuid,
    pub log_data: Value,
    pub created_at: DateTime<Utc>,
}
