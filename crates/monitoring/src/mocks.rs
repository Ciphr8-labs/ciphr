// crates/monitoring/src/mocks.rs
use crate::traits::MonitoringService;
use std::collections::HashMap;

/// A mock monitoring service that prints events to the console.
pub struct MockMonitoringService;

impl MonitoringService for MockMonitoringService {
    fn report_error(&self, error: &dyn std::error::Error) {
        println!("[MONITORING] Error reported: {}", error);
    }

    fn track_event(&self, name: &str, properties: HashMap<String, String>) {
        println!("[MONITORING] Event tracked: {} | Properties: {:?}", name, properties);
    }
} 