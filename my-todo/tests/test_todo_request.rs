use axum::http;
use pretty_assertions::assert_eq;
use tower::ServiceExt;

use crate::utilities::request_builders::{build_todo_request, JsonOrEmpty};
use crate::utilities::response_converters::convert_to_todo;
use my_todo::app::create_app;
use my_todo::models::todos::{CreateTodo, Todo};
use my_todo::repositories::in_memory::TodoRepositoryInMemory;
use my_todo::repositories::TodoRepository;

mod utilities;

#[tokio::test]
async fn should_find_todo() -> anyhow::Result<()> {
    let expected = Todo::new(1, "should find todo");

    let repository = TodoRepositoryInMemory::new();
    repository.create(CreateTodo {
        text: "should find todo".to_string(),
    });

    let req = build_todo_request("/todos/1", http::Method::GET, JsonOrEmpty::Empty)?;
    let res = create_app(repository).oneshot(req).await?;

    let todo: Todo = convert_to_todo(res).await?;

    assert_eq!(todo, expected);

    Ok(())
}

#[tokio::test]
async fn should_get_all_todos() -> anyhow::Result<()> {
    let expected = Todo::new(1, "should all todo");

    let repository = TodoRepositoryInMemory::new();
    repository.create(CreateTodo {
        text: "should all todo".to_string(),
    });

    let req = build_todo_request("/todos", http::Method::GET, JsonOrEmpty::Empty)?;
    let res = create_app(repository).oneshot(req).await?;

    let todos: Vec<Todo> = convert_to_todo(res).await?;

    assert_eq!(todos, vec![expected]);

    Ok(())
}

#[tokio::test]
async fn should_update_todo() -> anyhow::Result<()> {
    let expected = Todo::new(1, "should update todo");

    let repository = TodoRepositoryInMemory::new();
    repository.create(CreateTodo {
        text: "before update todo".to_string(),
    });

    let req = build_todo_request::<JsonOrEmpty>(
        "/todos/1",
        http::Method::PATCH,
        r#"{ "text": "should update todo", "completed": false }"#.into(),
    )?;
    let res = create_app(repository).oneshot(req).await?;

    let todos: Todo = convert_to_todo(res).await?;

    assert_eq!(todos, expected);

    Ok(())
}

#[tokio::test]
async fn should_delete_todo() -> anyhow::Result<()> {
    let repository = TodoRepositoryInMemory::new();
    repository.create(CreateTodo {
        text: "should delete todo".to_string(),
    });

    let req = build_todo_request("/todos/1", http::Method::DELETE, JsonOrEmpty::Empty)?;
    let res = create_app(repository).oneshot(req).await?;

    assert_eq!(res.status(), http::StatusCode::NO_CONTENT);

    Ok(())
}
