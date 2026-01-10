use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub version: String,
    pub environment: String,
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/health", get(health))
        .route("/status", get(status))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

async fn status() -> Json<StatusResponse> {
    Json(StatusResponse {
        version: "0.1.0".to_string(),
        environment: "development".to_string(),
    })
}
