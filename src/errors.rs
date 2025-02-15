use std::fmt;

#[derive(Debug)]
pub enum SiderErrorKind {
    InvalidCommand,
    Nil,
    EmptyArray,
    WrongType,
    NotInt,
    DatabaseError,
    RegexError,
    TTL,
    Custom,
}

#[derive(Debug)]
pub struct SiderError {
    error_type: SiderErrorKind,
    message: Option<String>,
}

impl SiderError {
    pub fn new(error_type: SiderErrorKind) -> Self {
        Self {
            error_type,
            message: None,
        }
    }
    pub fn with_message(error_type: SiderErrorKind, message: impl Into<String>) -> Self {
        Self {
            error_type,
            message: Some(message.into()),
        }
    }
}

impl fmt::Display for SiderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(msg) = &self.message {
            return write!(f, "{}", msg);
        }

        let err_msg = match self.error_type {
            SiderErrorKind::InvalidCommand => "Invalid command",
            SiderErrorKind::Nil => "(nil)",
            SiderErrorKind::EmptyArray => "(empty array)",
            SiderErrorKind::WrongType => {
                "(error) WRONGTYPE Operation against a key holding the wrong kind of value"
            }
            SiderErrorKind::NotInt => "ERR value is not an integer",
            SiderErrorKind::DatabaseError => "ERR unexpected database error",
            SiderErrorKind::RegexError => "ERR regex",
            SiderErrorKind::TTL => "Error: TTL is required",
            SiderErrorKind::Custom => "Custom error",
        };

        write!(f, "{}", err_msg)
    }
}
