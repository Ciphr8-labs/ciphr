use uuid::Uuid;

/// A struct to hold contextual information for a set of related log entries.
///
/// This can be used to correlate all logs generated during a single
/// request or operation.
#[derive(Debug, Clone)]
pub struct RequestContext {
    /// A unique identifier for the request.
    pub request_id: Uuid,
}

impl RequestContext {
    /// Creates a new `RequestContext` with a new, unique request ID.
    pub fn new() -> Self {
        Self {
            request_id: Uuid::new_v4(),
        }
    }
}

impl Default for RequestContext {
    fn default() -> Self {
        Self::new()
    }
} 