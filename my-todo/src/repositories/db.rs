use crate::errors::RepositoryError;
use crate::models::todos::{CreateTodo, Todo, UpdateTodo};
use crate::repositories::TodoRepository;
use anyhow::Context;
use axum::async_trait;
use sqlx::MySqlPool;

#[derive(Debug, Clone)]
pub struct TodoRepositoryDb {
    pool: MySqlPool,
}
impl TodoRepositoryDb {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryDb {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<Todo> {
        // START TRANSACTION ;
        //     INSERT INTO todos (text, completed) VALUE ($1, false);
        //     SELECT * FROM todos WHERE id = LAST_INSERT_ID();
        // COMMIT ;
        let mut conn = self.pool.acquire().await?;
        sqlx::query!(
            "INSERT INTO todos (text, completed) VALUE (?, false)",
            payload.text
        )
        .execute(&mut conn)
        .await?;
        let todo = sqlx::query_as!(
            Todo,
            r#"SELECT
                 id
               , text
               , completed as `completed: bool`
               FROM todos
               WHERE id = LAST_INSERT_ID()"#
        )
        .fetch_one(&mut conn)
        .await?;

        Ok(todo)
    }

    async fn find(&self, id: u64) -> anyhow::Result<Todo> {
        let todo = sqlx::query_as!(
            Todo,
            r#"
            SELECT
                id
              , text
              , completed as `completed: bool`
            FROM todos
            WHERE id = ?
        "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        todo.context(RepositoryError::NotFound(id))
    }

    async fn all(&self) -> anyhow::Result<Vec<Todo>> {
        let todos = sqlx::query_as!(
            Todo,
            r#"
        SELECT
            id
          , text
          , completed as `completed: bool`
        FROM todos
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(todos)
    }

    async fn update(&self, id: u64, payload: UpdateTodo) -> anyhow::Result<Todo> {
        let mut conn = self.pool.acquire().await?;
        sqlx::query!(
            r#"
            UPDATE todos
            SET text = CASE
                           WHEN ? IS NULL THEN todos.text
                           ELSE ?
                       END
              , completed = CASE
                                WHEN ? IS NULL THEN todos.completed
                                ELSE ?
                            END
            WHERE
              id = ?
        "#,
            payload.text,
            payload.text,
            payload.completed,
            payload.completed,
            id
        )
        .execute(&mut conn)
        .await?;

        let todo = sqlx::query_as!(
            Todo,
            r#"SELECT id, text, completed as `completed: bool` FROM todos WHERE id = ?"#,
            id
        )
        .fetch_one(&mut conn)
        .await?;

        Ok(todo)
    }

    async fn delete(&self, id: u64) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM todos WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
