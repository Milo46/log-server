use std::sync::Arc;
use axum::{
    routing::{get, post, delete, put},
    Router,
    response::Json,
    http::StatusCode,
};
use tower::ServiceBuilder;
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer,
};
use serde_json::json;

pub mod models;
pub mod repositories;
pub mod services;
pub mod handlers;

pub use models::{Log, Schema};
pub use services::{SchemaService, LogService};
pub use handlers::{
    get_schemas, get_schema_by_id, create_schema, update_schema, delete_schema,
    get_logs_default, get_logs, get_log_by_id, create_log, delete_log, ErrorResponse
};
pub use repositories::{SchemaRepository, LogRepository};

#[derive(Clone)]
pub struct AppState {
    pub schema_service: Arc<SchemaService>,
    pub log_service: Arc<LogService>,
}

async fn health_check() -> Result<Json<serde_json::Value>, StatusCode> {
    tracing::info!("Health check endpoint called");
    Ok(Json(json!({
        "status": "healthy",
        "service": "log-server",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

pub fn create_app(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/schemas", get(get_schemas))
        .route("/schemas", post(create_schema))
        .route("/schemas/{id}", get(get_schema_by_id))
        .route("/schemas/{id}", put(update_schema))
        .route("/schemas/{id}", delete(delete_schema))
        .route("/logs", post(create_log))
        .route("/logs/schema/{schema_name}", get(get_logs_default))
        .route("/logs/schema/{schema_name}/{schema_version}", get(get_logs))
        .route("/logs/{id}", get(get_log_by_id))
        .route("/logs/{id}", delete(delete_log))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        )
        .with_state(app_state)
}
