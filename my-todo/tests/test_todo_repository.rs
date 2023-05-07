use pretty_assertions::assert_eq;

use my_todo::models::todos::{CreateTodo, Todo, UpdateTodo};
use my_todo::repositories::in_memory::TodoRepositoryInMemory;
use my_todo::repositories::TodoRepository;

#[tokio::test]
async fn todo_crud_scenario() -> anyhow::Result<()> {
    let text = "todo text".to_string();
    let id = 1;
    let expected = Todo::new(id, &text);

    let repository = TodoRepositoryInMemory::new();

    // create todoitem
    let todo = repository.create(CreateTodo { text: text.clone() }).await;
    assert!(todo.is_ok());
    assert_eq!(todo.unwrap(), expected.clone());

    // find todoitem
    let todo = repository.find(id).await;
    assert!(todo.is_ok());
    assert_eq!(todo.unwrap(), expected.clone());

    // all
    let todos = repository.all().await;
    assert!(todos.is_ok());
    assert_eq!(todos.unwrap(), vec![expected.clone()]);

    // update
    let text = "update todo text".to_string();
    let todo = repository
        .update(
            id,
            UpdateTodo {
                text: Some(text.clone()),
                completed: Some(true),
            },
        )
        .await;
    assert!(todo.is_ok());
    assert_eq!(todo.unwrap(), Todo::done(id, &text));

    // delete
    let res = repository.delete(id).await;
    assert!(res.is_ok());

    Ok(())
}
