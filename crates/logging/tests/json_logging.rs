use logging::get_logging_layer;
use config::types::{AppConfig, LogFormat, LogLevel};
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::Registry;

#[test]
fn test_json_logging_works() {
    let config = AppConfig {
        log_level: Some(LogLevel::Info),
        log_format: Some(LogFormat::Json),
        ..Default::default()
    };
    let layer = get_logging_layer(&config);
    let subscriber = Registry::default().with(layer);

    tracing::subscriber::with_default(subscriber, || {
        info!(message = "This is a JSON test log.", key = "value");
    });
    
    // As with the unit test, a simple panic check is the most
    // straightforward approach for now.
} 