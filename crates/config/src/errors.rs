use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read configuration: {0}")]
    Io(#[from] io::Error),

    #[error("Failed to parse TOML configuration: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Validation failed: {field}")]
    ValidationError { field: String },

    #[error("Configuration source not found: {0}")]
    NotFound(String),

    #[error("An unexpected error occurred: {0}")]
    Unexpected(#[from] anyhow::Error),
} 