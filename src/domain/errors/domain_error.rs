use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Infra error: {0}")]
    Infra(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}
