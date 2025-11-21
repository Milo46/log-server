use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Schema {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub schema_definition: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
