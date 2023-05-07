use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Not Found, id: {0}")]
    NotFound(u64),
}
