use serde::{Deserialize, Serialize};
use validator::Validate;

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

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, message = "TODOs without content cannot be created."))]
    #[validate(length(
        max = 100,
        message = "TODOs that are too long (over 100 characters) cannot be created."
    ))]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Default, Validate)]
pub struct UpdateTodo {
    #[validate(length(
        min = 1,
        message = "It is not possible to update to a TODO without content."
    ))]
    #[validate(length(
        max = 100,
        message = "It is not possible to update a TODO whose content is too long (more than 100 characters)."
    ))]
    pub text: Option<String>,
    pub completed: Option<bool>,
}
