use std::path::{Path, PathBuf};
use crate::{
    errors::ConfigError,
    traits::ConfigurationProvider,
    types::AppConfig,
};

/// A configuration provider that loads settings from a TOML file.
pub struct FileConfigurationProvider {
    path: PathBuf,
}

impl FileConfigurationProvider {
    /// Creates a new `FileConfigurationProvider`.
    ///
    /// # Parameters
    /// - `path`: The path to the configuration file.
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl ConfigurationProvider for FileConfigurationProvider {
    fn load(&self) -> Result<AppConfig, ConfigError> {
        let contents = std::fs::read_to_string(&self.path).map_err(|e| {
            ConfigError::ReadError(format!(
                "Failed to read file at {}: {}",
                self.path.display(),
                e
            ))
        })?;

        let config: AppConfig = toml::from_str(&contents)
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;
        
        Ok(config)
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_load_valid_config() {
        let content = r#"
            environment = "production"
            log_level = "debug"
            log_format = "json"
            [feature_flags]
            new_ui = true
        "#;
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", content).unwrap();

        let provider = FileConfigurationProvider::new(file.path());
        let config = provider.load().unwrap();

        assert_eq!(config.environment, "production");
        assert_eq!(config.log_level, crate::types::LogLevel::Debug);
        assert_eq!(config.log_format, crate::types::LogFormat::Json);
        assert_eq!(config.feature_flags.get("new_ui"), Some(&true));
    }

    #[test]
    fn test_load_non_existent_file() {
        let provider = FileConfigurationProvider::new("non_existent_file.toml");
        let result = provider.load();
        assert!(matches!(result, Err(ConfigError::ReadError(_))));
    }

    #[test]
    fn test_load_invalid_toml() {
        let content = "this is not valid toml";
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", content).unwrap();

        let provider = FileConfigurationProvider::new(file.path());
        let result = provider.load();
        assert!(matches!(result, Err(ConfigError::ParseError(_))));
    }

    #[test]
    fn test_load_validation_error() {
        let content = r#"
            environment = ""
            log_level = "info"
            log_format = "text"
        "#;
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", content).unwrap();

        let provider = FileConfigurationProvider::new(file.path());
        let config = provider.load().unwrap();
        let result = provider.validate(&config);
        assert!(matches!(result, Err(ConfigError::ValidationError { .. })));
    }
} 