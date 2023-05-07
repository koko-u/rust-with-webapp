use std::sync::Arc;

use axum::{routing, Extension, Router};

use crate::handlers::root;
use crate::handlers::todos::*;
use crate::repositories::TodoRepository;

pub fn create_app<T>(repository: T) -> Router
where
    T: TodoRepository,
{
    Router::new()
        .route("/", routing::get(root))
        .route("/todos", routing::get(all_todos::<T>))
        .route("/todos", routing::post(create_todo::<T>))
        .route("/todos/:id", routing::get(find_todo::<T>))
        .route("/todos/:id", routing::patch(update_todo::<T>))
        .route("/todos/:id", routing::delete(delete_todo::<T>))
        .layer(Extension(Arc::new(repository)))
}
