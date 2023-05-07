use axum::http;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Not Found, id: {0}")]
    NotFound(u64),
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Server Error: {0}")]
    StatusError(http::StatusCode),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let ServerError::StatusError(status) = self;
        let mut res = ().into_response();
        *res.status_mut() = status;
        res
    }
}
