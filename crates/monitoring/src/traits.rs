use std::collections::HashMap;

/// A trait for services that provide error tracking and monitoring.
pub trait MonitoringService: Send + Sync {
    /// Reports an error to the monitoring service.
    fn report_error(&self, error: &dyn std::error::Error);

    /// Tracks a custom event.
    fn track_event(&self, name: &str, properties: HashMap<String, String>);
} 