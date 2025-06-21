use crate::types::{AppConfig, LogFormat, LogLevel};
use std::collections::HashMap;

/// A builder for creating `AppConfig` instances.
///
/// This builder provides a fluent API for constructing an `AppConfig`
/// piece by piece, with validation occurring at build time.
#[derive(Default)]
pub struct AppConfigBuilder {
    environment: Option<String>,
    log_level: Option<LogLevel>,
    log_format: Option<LogFormat>,
    feature_flags: HashMap<String, bool>,
}

impl AppConfigBuilder {
    /// Creates a new `AppConfigBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the environment for the configuration.
    pub fn environment(mut self, environment: impl Into<String>) -> Self {
        self.environment = Some(environment.into());
        self
    }

    /// Sets the log level for the configuration.
    pub fn log_level(mut self, log_level: LogLevel) -> Self {
        self.log_level = Some(log_level);
        self
    }

    /// Sets the log format for the configuration.
    pub fn log_format(mut self, log_format: LogFormat) -> Self {
        self.log_format = Some(log_format);
        self
    }

    /// Adds a feature flag to the configuration.
    pub fn feature_flag(mut self, key: impl Into<String>, value: bool) -> Self {
        self.feature_flags.insert(key.into(), value);
        self
    }

    /// Builds the `AppConfig`.
    ///
    /// This method will use default values for any fields that have not
    /// been explicitly set.
    pub fn build(self) -> AppConfig {
        AppConfig {
            environment: self.environment.unwrap_or_else(|| "development".to_string()),
            log_level: self.log_level.unwrap_or(LogLevel::Info),
            log_format: self.log_format.unwrap_or(LogFormat::Text),
            feature_flags: self.feature_flags,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_defaults() {
        let config = AppConfigBuilder::new().build();
        assert_eq!(config.environment, "development");
        assert_eq!(config.log_level, LogLevel::Info);
        assert_eq!(config.log_format, LogFormat::Text);
        assert!(config.feature_flags.is_empty());
    }

    #[test]
    fn test_builder_custom_values() {
        let config = AppConfigBuilder::new()
            .environment("production")
            .log_level(LogLevel::Debug)
            .log_format(LogFormat::Json)
            .feature_flag("new_feature", true)
            .build();

        assert_eq!(config.environment, "production");
        assert_eq!(config.log_level, LogLevel::Debug);
        assert_eq!(config.log_format, LogFormat::Json);
        assert_eq!(config.feature_flags.get("new_feature"), Some(&true));
    }
} 