use axum::{
    extract::State,
    Json,
    routing::get,
    Router,
};
use serde::Serialize;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/winforge/health", get(health))
}

#[derive(Serialize)]
struct HealthResponse {
    port: u16,
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse { port: state.port })
}
