mod folders;
mod health;

use axum::Router;
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(folders::router())
        .merge(health::router())
        .with_state(state)
}
