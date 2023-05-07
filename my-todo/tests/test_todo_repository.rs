use pretty_assertions::assert_eq;

use my_todo::models::todos::{CreateTodo, Todo, UpdateTodo};
use my_todo::repositories::in_memory::TodoRepositoryInMemory;
use my_todo::repositories::TodoRepository;

#[test]
fn todo_crud_scenario() -> anyhow::Result<()> {
    let text = "todo text".to_string();
    let id = 1;
    let expected = Todo::new(id, &text);

    let repository = TodoRepositoryInMemory::new();

    // create todoitem
    let todo = repository.create(CreateTodo { text: text.clone() });

    assert_eq!(todo, expected.clone());

    // find todoitem
    let todo = repository.find(id);
    assert_eq!(todo, Some(expected.clone()));

    // all
    let todos = repository.all();
    assert_eq!(todos, vec![expected.clone()]);

    // update
    let text = "update todo text".to_string();
    let todo = repository.update(
        id,
        UpdateTodo {
            text: Some(text.clone()),
            completed: Some(true),
        },
    );
    assert!(todo.is_ok());
    assert_eq!(todo.unwrap(), Todo::done(id, &text));

    // delete
    let res = repository.delete(id);
    assert!(res.is_ok());

    Ok(())
}
