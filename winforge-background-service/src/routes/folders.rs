use axum::{
    Json, Router,
    extract::{Query, State, rejection::QueryRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::{models::folders::Folder, state::AppState};

const MAX_FOLDERS_PER_PAGE: usize = 100;

#[derive(Debug, Deserialize)]
struct Pagination {
    last_id: Option<i64>,
    per_page: usize,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: &'static str,
    message: &'static str,
}

#[derive(Debug)]
struct ApiError {
    status: StatusCode,
    body: ErrorResponse,
}

impl ApiError {
    fn new(status: StatusCode, code: &'static str, message: &'static str) -> Self {
        Self {
            status,
            body: ErrorResponse { code, message },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}

pub fn router() -> Router<AppState> {
    Router::new().route("/winforge/folders", get(folders))
}

async fn folders(
    State(state): State<AppState>,
    pagination: Result<Query<Pagination>, QueryRejection>,
) -> Result<Json<Vec<Folder>>, ApiError> {
    let Query(pagination) = pagination.map_err(|rejection| {
        debug!(%rejection, "Invalid folders pagination parameters");
        ApiError::new(
            StatusCode::BAD_REQUEST,
            "INVALID_PAGINATION",
            "Expected a numeric per_page and an optional numeric last_id",
        )
    })?;

    if !(1..=MAX_FOLDERS_PER_PAGE).contains(&pagination.per_page) {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "INVALID_PER_PAGE",
            "per_page must be between 1 and 100",
        ));
    }

    if pagination.last_id.is_some_and(|last_id| last_id < 0) {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "INVALID_LAST_ID",
            "last_id must be zero or greater",
        ));
    }

    let per_page = pagination.per_page as i64;

    debug!(
        last_id = ?pagination.last_id,
        per_page,
        "Retrieving folders"
    );

    let conn = state.pool.get().map_err(|error| {
        error!(%error, "Failed to get a database connection");
        ApiError::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "DATABASE_UNAVAILABLE",
            "The folder service is temporarily unavailable",
        )
    })?;

    let mut statement = conn
        .prepare(
            "SELECT id, uid, resource_path, prompt, created_at
             FROM folders
             WHERE (?1 IS NULL OR id > ?1)
             ORDER BY id ASC
             LIMIT ?2",
        )
        .map_err(internal_database_error)?;

    let folders = statement
        .query_map(params![pagination.last_id, per_page], |row| {
            Ok(Folder {
                id: row.get(0)?,
                uid: row.get(1)?,
                resource_path: row.get(2)?,
                prompt: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .map_err(internal_database_error)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(internal_database_error)?;

    debug!(count = folders.len(), "Folders retrieved");
    Ok(Json(folders))
}

fn internal_database_error(error: rusqlite::Error) -> ApiError {
    error!(%error, "Failed to query folders");
    ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "DATABASE_ERROR",
        "Failed to retrieve folders",
    )
}
