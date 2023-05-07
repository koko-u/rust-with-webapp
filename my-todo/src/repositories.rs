use crate::models::todos::{CreateTodo, Todo, UpdateTodo};
use axum::async_trait;

#[async_trait]
pub trait TodoRepository: Clone + Send + Sync + 'static {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<Todo>;
    async fn find(&self, id: u64) -> anyhow::Result<Todo>;
    async fn all(&self) -> anyhow::Result<Vec<Todo>>;
    async fn update(&self, id: u64, payload: UpdateTodo) -> anyhow::Result<Todo>;
    async fn delete(&self, id: u64) -> anyhow::Result<()>;
}

pub mod in_memory;
