use crate::errors::LoggingError;
use crate::formatters::JsonFormatter;
use config::types::{AppConfig, LogFormat};
use tracing_subscriber::{
    fmt,
    layer::Layer,
    EnvFilter, Registry,
};

/// Initializes the global logging subscriber.
///
/// This function sets up the `tracing` subscriber based on the provided
/// application configuration. It supports different log formats and levels.
///
/// # Arguments
///
/// * `config` - The application configuration.
///
/// # Returns
///
/// * `Ok(())` if initialization is successful.
/// * `Err(LoggingError)` if initialization fails.
pub fn get_logging_layer(
    config: &AppConfig,
) -> Result<Box<dyn Layer<Registry> + Send + Sync>, LoggingError> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.log_level.to_string()));

    let layer: Box<dyn Layer<Registry> + Send + Sync> = match config.log_format {
        LogFormat::Json => {
            let log_layer = fmt::layer().event_format(JsonFormatter);
            Box::new(log_layer.with_filter(env_filter))
        }
        LogFormat::Text => {
            let log_layer = fmt::layer();
            Box::new(log_layer.with_filter(env_filter))
        }
    };

    Ok(layer)
} 