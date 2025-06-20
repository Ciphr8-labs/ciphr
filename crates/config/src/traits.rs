use crate::errors::ConfigError;
use crate::types::AppConfig;

/// A trait for providing application configuration.
///
/// This allows for a generic interface to load configuration from different
/// sources, such as files, environment variables, or remote services.
pub trait ConfigurationProvider {
    /// Loads the configuration.
    ///
    /// # Returns
    /// A `Result` containing the `AppConfig` or a `ConfigError`.
    fn load(&self) -> Result<AppConfig, ConfigError>;

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
    use crate::types::LogLevel;
    use std::collections::HashMap;

    struct MockProvider {
        config: AppConfig,
    }

    impl ConfigurationProvider for MockProvider {
        fn load(&self) -> Result<AppConfig, ConfigError> {
            Ok(self.config.clone())
        }

        fn validate(&self, config: &AppConfig) -> Result<(), ConfigError> {
            if config.environment.is_empty() {
                return Err(ConfigError::ValidationError {
                    field: "environment".to_string(),
                    message: "cannot be empty".to_string(),
                });
            }
            Ok(())
        }
    }

    #[test]
    fn test_mock_provider_load() {
        let mut flags = HashMap::new();
        flags.insert("new_feature".to_string(), true);

        let config = AppConfig {
            environment: "test".to_string(),
            log_level: LogLevel::Debug,
            feature_flags: flags,
        };

        let provider = MockProvider { config: config.clone() };
        let loaded_config = provider.load().unwrap();
        assert_eq!(loaded_config, config);
    }

    #[test]
    fn test_mock_provider_validate() {
        let provider = MockProvider { config: AppConfig::default() };
        let valid_config = AppConfig::default();
        assert!(provider.validate(&valid_config).is_ok());

        let invalid_config = AppConfig {
            environment: "".to_string(),
            ..Default::default()
        };
        assert!(provider.validate(&invalid_config).is_err());
    }
} 