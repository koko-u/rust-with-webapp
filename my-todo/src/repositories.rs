use crate::models::todos::{CreateTodo, Todo, UpdateTodo};

pub trait TodoRepository: Clone + Send + Sync + 'static {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: u64) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: u64, payload: UpdateTodo) -> anyhow::Result<Todo>;
    fn delete(&self, id: u64) -> anyhow::Result<()>;
}

pub mod in_memory;
