use crate::errors::ConfigError;
use crate::types::AppConfig;

/// A trait for providing application configuration.
///
/// This allows for a generic interface to load configuration from different
/// sources, such as files, environment variables, or remote services.
pub trait ConfigurationProvider {
    /// Loads the configuration.
    ///
    /// Returns an `AppConfig` instance if the provider can successfully
    /// load and parse the configuration. If the source is not found (e.g.,
    /// a file doesn't exist), it should return `Ok(None)`.
    fn load(&self) -> Result<Option<AppConfig>, ConfigError>;

    /// Validates the loaded configuration.
    ///
    /// # Parameters
    /// - `config`: A reference to the `AppConfig` to validate.
    ///
    /// # Returns
    /// An empty `Result` or a `ConfigError` if validation fails.
    fn validate(&self, config: &AppConfig) -> Result<(), ConfigError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AppConfig, LogFormat, LogLevel};

    struct MockProvider {
        config: AppConfig,
    }

    impl ConfigurationProvider for MockProvider {
        fn load(&self) -> Result<Option<AppConfig>, ConfigError> {
            Ok(Some(self.config.clone()))
        }

        fn validate(&self, config: &AppConfig) -> Result<(), ConfigError> {
            if let Some(env) = &config.environment {
                if env.is_empty() {
                    return Err(ConfigError::ValidationError {
                        field: "environment".to_string(),
                    });
                }
            }
            Ok(())
        }
    }

    #[test]
    fn test_mock_provider_load() {
        let config = AppConfig {
            environment: Some("test".to_string()),
            log_level: Some(LogLevel::Debug),
            log_format: Some(LogFormat::Json),
            feature_flags: Default::default(),
        };

        let provider = MockProvider {
            config: config.clone(),
        };
        let loaded_config = provider.load().unwrap().unwrap();
        assert_eq!(loaded_config, config);
    }

    #[test]
    fn test_mock_provider_validate() {
        let provider = MockProvider {
            config: AppConfig::default(),
        };

        let mut invalid_config = AppConfig::default();
        invalid_config.environment = Some("".to_string());
        let result = provider.validate(&invalid_config);
        assert!(matches!(
            result,
            Err(ConfigError::ValidationError { .. })
        ));
    }
} 