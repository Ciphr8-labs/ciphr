use crate::{
    errors::ConfigError,
    traits::ConfigurationProvider,
    types::AppConfig,
};

/// A configuration provider that layers multiple providers.
///
/// It iterates through its list of providers, loading and merging
/// the configuration from each one. Later providers in the list
/// override earlier ones.
pub struct LayeredConfigurationProvider {
    providers: Vec<Box<dyn ConfigurationProvider>>,
}

impl LayeredConfigurationProvider {
    /// Creates a new, empty layered provider.
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    /// Adds a configuration provider to the layer stack.
    pub fn with_provider(mut self, provider: Box<dyn ConfigurationProvider>) -> Self {
        self.providers.push(provider);
        self
    }
}

impl Default for LayeredConfigurationProvider {
    fn default() -> Self {
        Self::new()
    }
}

// Helper macro to merge optional fields. If the new value is Some, it overwrites the existing value.
macro_rules! merge_option {
    ($existing:expr, $new:expr) => {
        if $new.is_some() {
            $existing = $new;
        }
    };
}

impl ConfigurationProvider for LayeredConfigurationProvider {
    fn load(&self) -> Result<Option<AppConfig>, ConfigError> {
        let mut merged_config = AppConfig::default();
        let mut at_least_one_config_loaded = false;

        for provider in &self.providers {
            if let Some(loaded_config) = provider.load()? {
                at_least_one_config_loaded = true;
                merge_option!(merged_config.environment, loaded_config.environment);
                merge_option!(merged_config.log_level, loaded_config.log_level);
                merge_option!(merged_config.log_format, loaded_config.log_format);
                merged_config.feature_flags.extend(loaded_config.feature_flags);
            }
        }

        if at_least_one_config_loaded {
            Ok(Some(merged_config))
        } else {
            Ok(None)
        }
    }

    fn validate(&self, config: &AppConfig) -> Result<(), ConfigError> {
        // Validation can be complex in a layered setup. For now, we'll
        // delegate to the first provider's validation logic, assuming
        // it contains the base validation rules. A more robust strategy
        // could be implemented later.
        if let Some(provider) = self.providers.first() {
            provider.validate(config)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        loader::FileConfigurationProvider,
        types::{LogFormat, LogLevel},
    };
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_config_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", content).unwrap();
        file
    }

    #[test]
    fn test_layered_override() {
        let base_config_content = r#"
            environment = "development"
            log_level = "info"
        "#;
        let base_file = create_temp_config_file(base_config_content);
        let base_provider = FileConfigurationProvider::new(base_file.path());

        let override_config_content = r#"
            environment = "production"
            log_format = "json"
        "#;
        let override_file = create_temp_config_file(override_config_content);
        let override_provider = FileConfigurationProvider::new(override_file.path());

        let layered_provider = LayeredConfigurationProvider::new()
            .with_provider(Box::new(base_provider))
            .with_provider(Box::new(override_provider));

        let config = layered_provider.load().unwrap().unwrap();

        assert_eq!(config.environment, Some("production".to_string())); // Overridden
        assert_eq!(config.log_level, Some(LogLevel::Info)); // From base
        assert_eq!(config.log_format, Some(LogFormat::Json)); // From override
    }

    #[test]
    fn test_feature_flag_merging() {
        let base_config_content = r#"
            [feature_flags]
            one = true
            two = false
        "#;
        let base_file = create_temp_config_file(base_config_content);
        let base_provider = FileConfigurationProvider::new(base_file.path());

        let override_config_content = r#"
            [feature_flags]
            two = true
            three = true
        "#;
        let override_file = create_temp_config_file(override_config_content);
        let override_provider = FileConfigurationProvider::new(override_file.path());

        let layered_provider = LayeredConfigurationProvider::new()
            .with_provider(Box::new(base_provider))
            .with_provider(Box::new(override_provider));
        
        let config = layered_provider.load().unwrap().unwrap();

        assert_eq!(config.feature_flags.get("one"), Some(&true));
        assert_eq!(config.feature_flags.get("two"), Some(&true));
        assert_eq!(config.feature_flags.get("three"), Some(&true));
        assert_eq!(config.feature_flags.len(), 3);
    }
} 