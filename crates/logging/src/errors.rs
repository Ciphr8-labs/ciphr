use thiserror::Error;

/// Defines errors that can occur within the logging infrastructure.
#[derive(Error, Debug)]
pub enum LoggingError {
    /// Error returned when the logging system fails to initialize.
    #[error("Failed to initialize logging subscriber: {0}")]
    Initialization(String),

    /// Error returned for an invalid log level configuration.
    #[error("Invalid log level specified: {0}")]
    InvalidLogLevel(String),

    /// Error returned when failing to set up a file appender.
    #[error("Failed to set up file logger: {0}")]
    FileAppender(String),
} 