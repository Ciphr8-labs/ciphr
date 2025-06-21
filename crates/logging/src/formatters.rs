use serde::Serialize;
use tracing::{Event, Subscriber};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;

/// A struct that formats log events as JSON.
pub struct JsonFormatter;

impl<S, N> FormatEvent<S, N> for JsonFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let meta = event.metadata();

        let mut visit = JsonVisitor::default();
        event.record(&mut visit);

        let log_entry = LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: meta.level().to_string(),
            target: meta.target().to_string(),
            message: visit.message,
            fields: visit.fields,
        };

        if let Ok(json) = serde_json::to_string(&log_entry) {
            writeln!(writer, "{}", json)?;
        }

        Ok(())
    }
}

/// A visitor to extract key-value data from a log event.
#[derive(Default)]
struct JsonVisitor {
    fields: std::collections::HashMap<String, serde_json::Value>,
    message: String,
}

impl tracing::field::Visit for JsonVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        let value_str = format!("{:?}", value);
        self.fields
            .insert(field.name().to_string(), serde_json::Value::String(value_str));
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_string();
        } else {
            self.fields
                .insert(field.name().to_string(), serde_json::Value::String(value.to_string()));
        }
    }
}

/// Represents a single log entry in a structured format.
#[derive(Serialize)]
struct LogEntry {
    timestamp: String,
    level: String,
    target: String,
    message: String,
    fields: std::collections::HashMap<String, serde_json::Value>,
} 