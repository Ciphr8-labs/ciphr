use uuid::Uuid;
use std::collections::HashMap;

/// Provides context for feature flag evaluation.
///
/// This can be extended to include information about the user, request,
/// or environment, allowing for more sophisticated targeting rules.
#[derive(Debug, Default)]
pub struct EvaluationContext {
    /// A unique identifier for the user or session.
    pub user_id: Option<Uuid>,
    /// Additional properties for custom evaluation strategies.
    pub properties: HashMap<String, String>,
}

/// A trait for evaluating the state of a feature flag.
///
/// This trait allows for different strategies to be used for flag evaluation,
/// such as simple on/off, percentage-based rollouts, or user targeting.
pub trait FeatureFlagEvaluator {
    /// Determines if a feature is enabled based on the given context.
    ///
    /// # Parameters
    /// - `flag_name`: The name of the feature flag to evaluate.
    /// - `context`: The evaluation context.
    ///
    /// # Returns
    /// `true` if the feature should be considered enabled, `false` otherwise.
    fn is_enabled(&self, flag_name: &str, context: &EvaluationContext) -> bool;
} 