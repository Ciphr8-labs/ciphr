pub mod context;
pub mod errors;
pub mod formatters;
pub mod init;

pub use init::get_logging_layer;

#[cfg(test)]
mod tests {
    use super::*;
    use config::types::{AppConfig, LogFormat, LogLevel};
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::Registry;

    #[test]
    fn test_get_logging_layer() {
        let config = AppConfig {
            log_level: Some(LogLevel::Info),
            log_format: Some(LogFormat::Text),
            ..Default::default()
        };

        let layer = get_logging_layer(&config);
        let subscriber = Registry::default().with(layer);

        // This is a basic test to ensure it doesn't panic.
        // A more thorough test would capture output.
        tracing::subscriber::with_default(subscriber, || {
            tracing::info!("test");
        });
    }
}
