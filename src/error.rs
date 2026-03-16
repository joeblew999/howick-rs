use thiserror::Error;

#[derive(Debug, Error)]
pub enum HowickError {
    #[error("Parse error on line {line}: {message}")]
    ParseError { line: usize, message: String },

    #[error("Unknown operation '{0}'")]
    UnknownOperation(String),

    #[error("Unknown unit '{0}'")]
    UnknownUnit(String),

    #[error("Unknown label orientation '{0}'")]
    UnknownLabel(String),

    #[error("Missing field: {0}")]
    MissingField(String),

    #[error("Invalid number '{value}': {source}")]
    InvalidNumber {
        value: String,
        source: std::num::ParseFloatError,
    },

    #[error("Invalid integer '{value}': {source}")]
    InvalidInteger {
        value: String,
        source: std::num::ParseIntError,
    },

    #[error("IO error: {0}")]
    Io(#[from] std::fmt::Error),
}
