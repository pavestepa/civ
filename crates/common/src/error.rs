use thiserror::Error;

#[derive(Debug, Error)]
pub enum CivError {
    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("deserialization error: {0}")]
    Deserialization(String),

    #[error("invalid command: {0}")]
    InvalidCommand(String),

    #[error("scripting error: {0}")]
    Scripting(String),

    #[error("save error: {0}")]
    Save(String),

    #[error("network error: {0}")]
    Network(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("{0}")]
    Other(String),
}

pub type CivResult<T> = Result<T, CivError>;
