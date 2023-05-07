use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use anyhow::{bail, Context};

use crate::errors::RepositoryError;
use crate::models::todos::{CreateTodo, Todo, UpdateTodo};
use crate::repositories::TodoRepository;

type TodoData = HashMap<u64, Todo>;

#[derive(Debug, Clone, Default)]
pub struct TodoRepositoryInMemory {
    store: Arc<RwLock<TodoData>>,
}

impl TodoRepositoryInMemory {
    pub fn new() -> Self {
        Self::default()
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<TodoData> {
        self.store.write().unwrap()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<TodoData> {
        self.store.read().unwrap()
    }

    fn max_id(&self) -> u64 {
        match self.read_store_ref().keys().max() {
            Some(m) => *m,
            None => 0,
        }
    }
}

impl TodoRepository for TodoRepositoryInMemory {
    fn create(&self, payload: CreateTodo) -> Todo {
        let id = self.max_id() + 1;
        let mut store = self.write_store_ref();
        let todo = Todo::new(id, &payload.text);
        store.insert(id, todo.clone());
        todo
    }

    fn find(&self, id: u64) -> Option<Todo> {
        let store = self.read_store_ref();
        store.get(&id).cloned()
    }

    fn all(&self) -> Vec<Todo> {
        let store = self.read_store_ref();
        store.values().cloned().collect()
    }

    fn update(&self, id: u64, payload: UpdateTodo) -> anyhow::Result<Todo> {
        let mut store = self.write_store_ref();
        let todo = store.get(&id).context(RepositoryError::NotFound(id))?;

        let text = payload.text.as_ref().unwrap_or(&todo.text);
        let completed = payload.completed.unwrap_or(todo.completed);

        let todo = if completed {
            Todo::done(id, text)
        } else {
            Todo::new(id, text)
        };

        store.insert(id, todo.clone());

        Ok(todo)
    }

    fn delete(&self, id: u64) -> anyhow::Result<()> {
        let mut store = self.write_store_ref();
        let Some(_) = store.remove(&id) else {
            bail!(RepositoryError::NotFound(id));
        };
        Ok(())
    }
}
