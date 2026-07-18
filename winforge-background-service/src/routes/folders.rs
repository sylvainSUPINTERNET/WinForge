use axum::{
    extract::State,
    routing::get,
    Router,
};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/winforge/folders", get(folders))
}

async fn folders(State(state): State<AppState>) -> &'static str {
    let _pool = &state.pool;

    "Hello, World!"
}
