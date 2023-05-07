use std::sync::Arc;

use axum::response::IntoResponse;
use axum::{http, Extension, Json};

use crate::models::todos::CreateTodo;
use crate::repositories::TodoRepository;

pub async fn create_todo<T>(
    Extension(repository): Extension<Arc<T>>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse
where
    T: TodoRepository,
{
    let todo = repository.create(payload);

    (http::StatusCode::CREATED, Json(todo))
}
