use crate::{
    errors::ConfigError,
    traits::ConfigurationProvider,
    types::{AppConfig},
};
use std::{
    fs, io,
    path::{Path, PathBuf},
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
    fn load(&self) -> Result<Option<AppConfig>, ConfigError> {
        match fs::read_to_string(&self.path) {
            Ok(contents) => {
                let config: AppConfig = toml::from_str(&contents)?;
                Ok(Some(config))
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{LogFormat, LogLevel};
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_config_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", content).unwrap();
        file
    }

    #[test]
    fn test_load_valid_config() {
        let content = r#"
            environment = "production"
            log_level = "debug"
            log_format = "json"
            [feature_flags]
            new_ui = true
        "#;
        let file = create_temp_config_file(content);

        let provider = FileConfigurationProvider::new(file.path());
        let config = provider.load().unwrap().unwrap();

        assert_eq!(config.environment, Some("production".to_string()));
        assert_eq!(config.log_level, Some(LogLevel::Debug));
        assert_eq!(config.log_format, Some(LogFormat::Json));
        assert_eq!(config.feature_flags.get("new_ui"), Some(&true));
    }

    #[test]
    fn test_load_non_existent_file() {
        let provider = FileConfigurationProvider::new("non_existent_file.toml");
        let result = provider.load();
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_load_invalid_toml() {
        let content = "this is not valid toml";
        let file = create_temp_config_file(content);

        let provider = FileConfigurationProvider::new(file.path());
        let result = provider.load();
        assert!(matches!(result, Err(ConfigError::Toml(_))));
    }

    #[test]
    fn test_load_validation_error() {
        let content = r#"
            environment = ""
        "#;
        let file = create_temp_config_file(content);

        let provider = FileConfigurationProvider::new(file.path());
        let config = provider.load().unwrap().unwrap();
        let result = provider.validate(&config);
        assert!(matches!(
            result,
            Err(ConfigError::ValidationError { .. })
        ));
    }

    #[test]
    fn test_load_development_config() {
        let content = r#"
             environment = "development"
             log_level = "info"
             log_format = "text"
         "#;
        let file = create_temp_config_file(content);

        let provider = FileConfigurationProvider::new(file.path());
        let config = provider.load().unwrap().unwrap();
        assert_eq!(config.environment, Some("development".to_string()));
        assert_eq!(config.log_level, Some(LogLevel::Info));
    }
} 