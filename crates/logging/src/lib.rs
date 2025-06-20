pub mod context;
pub mod errors;
pub mod formatters;
pub mod init;

pub use init::get_logging_layer;

#[cfg(test)]
mod tests {
    use super::*;
    use config::types::{AppConfig, LogFormat, LogLevel};
    use tracing::info;
    use tracing_subscriber::layer::SubscriberExt;

    #[test]
    fn test_logging_init_text() {
        let config = AppConfig {
            log_level: LogLevel::Info,
            log_format: LogFormat::Text,
            ..Default::default()
        };
        let layer = get_logging_layer(&config).unwrap();
        let subscriber = tracing_subscriber::registry().with(layer);

        // The test needs to run within the context of the subscriber
        tracing::subscriber::with_default(subscriber, || {
            info!("This is a test log message.");
        });

        // We can't easily assert on the output here without more complex setup,
        // so we just ensure that creating and using the layer doesn't panic.
    }
}
