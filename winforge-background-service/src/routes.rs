mod folders;
mod health;

use crate::state::AppState;
use axum::Router;
use tower_http::cors::CorsLayer;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(folders::router())
        .merge(health::router())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
