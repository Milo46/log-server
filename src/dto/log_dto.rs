use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::Log;

#[derive(Debug, Deserialize)]
pub struct CreateLogRequest {
    pub schema_id: Uuid,
    pub log_data: Value,
}

#[derive(Debug, Serialize)]
pub struct LogResponse {
    pub id: i32,
    pub schema_id: Uuid,
    pub log_data: Value,
    pub created_at: String,
}

impl From<Log> for LogResponse {
    fn from(log: Log) -> Self {
        LogResponse {
            id: log.id,
            schema_id: log.schema_id,
            log_data: log.log_data,
            created_at: log.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "lowercase")]
pub enum LogEvent {
    Created {
        id: i32,
        schema_id: Uuid,
        log_data: Value,
        created_at: String,
    },
    Deleted {
        id: i32,
        schema_id: Uuid,
    },
}

impl LogEvent {
    pub fn created_from(log: Log) -> Self {
        LogEvent::Created {
            id: log.id,
            schema_id: log.schema_id,
            log_data: log.log_data,
            created_at: log.created_at.to_rfc3339(),
        }
    }

    pub fn deleted_from(log: Log) -> Self {
        LogEvent::Deleted {
            id: log.id,
            schema_id: log.schema_id,
        }
    }

    pub fn schema_id(&self) -> Uuid {
        match self {
            LogEvent::Created { schema_id, .. } => *schema_id,
            LogEvent::Deleted { schema_id, .. } => *schema_id,
        }
    }
}
