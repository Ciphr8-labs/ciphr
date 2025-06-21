use config::types::{AppConfig, LogFormat};
use tracing_subscriber::{
    fmt::{format::FmtSpan, Layer as FmtLayer},
    registry::Registry,
    EnvFilter, Layer,
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
pub fn get_logging_layer(config: &AppConfig) -> Box<dyn Layer<Registry> + Send + Sync> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        let level = config.log_level.clone().unwrap_or_default().to_string();
        EnvFilter::new(level)
    });

    let layer = match config.log_format.as_ref().unwrap_or(&LogFormat::Text) {
        LogFormat::Json => {
            let layer = FmtLayer::new()
                .json()
                .with_span_list(true)
                .with_span_events(FmtSpan::FULL);
            Box::new(layer) as Box<dyn Layer<Registry> + Send + Sync>
        }
        LogFormat::Text => {
            let layer = FmtLayer::new().with_span_events(FmtSpan::FULL);
            Box::new(layer) as Box<dyn Layer<Registry> + Send + Sync>
        }
    };

    Box::new(layer.with_filter(env_filter))
} 