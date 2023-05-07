use std::sync::Arc;

use axum::{routing, Extension, Router};

use crate::handlers::root;
use crate::handlers::todos::create_todo;
use crate::repositories::TodoRepository;

pub fn create_app<T>(repository: T) -> Router
where
    T: TodoRepository,
{
    Router::new()
        .route("/", routing::get(root))
        .route("/todos", routing::post(create_todo::<T>))
        .layer(Extension(Arc::new(repository)))
}
