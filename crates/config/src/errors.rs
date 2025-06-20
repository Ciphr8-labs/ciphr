use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read configuration from path: {0}")]
    ReadError(String),

    #[error("Failed to parse configuration: {0}")]
    ParseError(String),

    #[error("Validation failed: {field}: {message}")]
    ValidationError { field: String, message: String },

    #[error("Configuration source not found: {0}")]
    NotFound(String),

    #[error("An unexpected error occurred: {0}")]
    Unexpected(#[from] anyhow::Error),
} 