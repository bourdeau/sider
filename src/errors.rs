use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum SiderError {
    #[error("Invalid command")]
    InvalidCommand,
    #[error("(nil)")]
    Nil,
    #[error("(empty array)")]
    EmptyArray,
    #[error("(error) WRONGTYPE Operation against a key holding the wrong kind of value")]
    WrongType,
    #[error("ERR value is not an integer")]
    NotInt,
    #[error("ERR unexpected database error")]
    DatabaseError,
    #[error("ERR regex")]
    RegexError,
    #[error("Error: TTL is required")]
    TTL,
    #[error("{0}")]
    Custom(String),
    #[error("(error) value is not an integer or out of range")]
    NotIntOrOutOfRange,
}
