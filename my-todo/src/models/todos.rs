use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Todo {
    pub id: u64,
    pub text: String,
    pub completed: bool,
}
impl Todo {
    pub fn new(id: u64, text: &str) -> Self {
        Self {
            id,
            text: text.to_string(),
            completed: false,
        }
    }
    pub fn done(id: u64, text: &str) -> Self {
        Self {
            completed: true,
            ..Self::new(id, text)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct CreateTodo {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Default)]
pub struct UpdateTodo {
    pub text: Option<String>,
    pub completed: Option<bool>,
}
