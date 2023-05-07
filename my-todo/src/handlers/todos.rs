use std::sync::Arc;

use crate::errors::ServerError;
use axum::response::IntoResponse;
use axum::{extract, http, Extension, Json};

use crate::models::todos::{CreateTodo, UpdateTodo};
use crate::payloads::ValidatedJson;
use crate::repositories::TodoRepository;

pub async fn create_todo<T>(
    Extension(repository): Extension<Arc<T>>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
) -> Result<impl IntoResponse, ServerError>
where
    T: TodoRepository,
{
    let Ok(todo) = repository.create(payload).await else {
        return Err(ServerError::StatusError(http::StatusCode::BAD_REQUEST));
    };

    Ok((http::StatusCode::CREATED, Json(todo)))
}

pub async fn find_todo<T>(
    Extension(repository): Extension<Arc<T>>,
    extract::Path(id): extract::Path<u64>,
) -> Result<impl IntoResponse, ServerError>
where
    T: TodoRepository,
{
    let Ok(todo) = repository.find(id).await else {
        return Err(ServerError::StatusError(http::StatusCode::NOT_FOUND));
    };

    Ok((http::StatusCode::OK, Json(todo)))
}

pub async fn all_todos<T>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, ServerError>
where
    T: TodoRepository,
{
    let Ok(todos) = repository.all().await else {
        return Err(ServerError::StatusError(http::StatusCode::NOT_FOUND));
    };
    Ok((http::StatusCode::OK, Json(todos)))
}

pub async fn update_todo<T>(
    Extension(repository): Extension<Arc<T>>,
    extract::Path(id): extract::Path<u64>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<impl IntoResponse, ServerError>
where
    T: TodoRepository,
{
    let Ok(todo) = repository.update(id, payload).await else {
        return Err(ServerError::StatusError(http::StatusCode::NOT_FOUND));
    };
    Ok((http::StatusCode::OK, Json(todo)))
}

pub async fn delete_todo<T>(
    Extension(repository): Extension<Arc<T>>,
    extract::Path(id): extract::Path<u64>,
) -> http::StatusCode
where
    T: TodoRepository,
{
    match repository.delete(id).await {
        Ok(_) => http::StatusCode::NO_CONTENT,
        Err(_) => http::StatusCode::NOT_FOUND,
    }
}
