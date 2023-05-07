use axum::response::{IntoResponse, Response};
use axum::{extract, http};

use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Not Found, id: {0}")]
    NotFound(u64),
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Server Error: {0}")]
    StatusError(http::StatusCode),
    #[error("Json Parse Error {0}")]
    JsonParseError(#[from] extract::rejection::JsonRejection),
    #[error("Validation Error {0}")]
    ValidationError(#[from] ValidationErrors),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::StatusError(status) => {
                let mut res = ().into_response();
                *res.status_mut() = status;
                res
            }
            e => {
                let mut res = e.to_string().into_response();
                *res.status_mut() = http::StatusCode::BAD_REQUEST;
                res
            }
        }
    }
}
