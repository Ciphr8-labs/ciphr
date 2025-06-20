use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Debug => "debug".to_string(),
            LogLevel::Info => "info".to_string(),
            LogLevel::Warn => "warn".to_string(),
            LogLevel::Error => "error".to_string(),
        }
    }
}

/// Defines the possible log output formats.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub environment: String,
    pub log_level: LogLevel,
    pub log_format: LogFormat,
    #[serde(default)]
    pub feature_flags: HashMap<String, bool>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            environment: "development".to_string(),
            log_level: LogLevel::Info,
            log_format: LogFormat::Text,
            feature_flags: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config_default() {
        let default_config = AppConfig::default();
        assert_eq!(default_config.environment, "development");
        assert_eq!(default_config.log_level, LogLevel::Info);
        assert_eq!(default_config.log_format, LogFormat::Text);
        assert!(default_config.feature_flags.is_empty());
    }
} 