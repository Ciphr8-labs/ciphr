use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub environment: String,
    pub log_level: LogLevel,
    #[serde(default)]
    pub feature_flags: HashMap<String, bool>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            environment: "development".to_string(),
            log_level: LogLevel::Info,
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
        assert!(default_config.feature_flags.is_empty());
    }
} 