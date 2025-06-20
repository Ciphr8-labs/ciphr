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