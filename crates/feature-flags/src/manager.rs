use crate::evaluator::{EvaluationContext, FeatureFlagEvaluator};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Manages the state and evaluation of feature flags.
pub struct FeatureFlagManager<E: FeatureFlagEvaluator> {
    evaluator: E,
    flags: Arc<RwLock<HashMap<String, bool>>>,
}

impl<E: FeatureFlagEvaluator> FeatureFlagManager<E> {
    /// Creates a new `FeatureFlagManager`.
    ///
    /// # Parameters
    /// - `evaluator`: The evaluator to use for checking flag states.
    /// - `flags`: The initial set of feature flags.
    pub fn new(evaluator: E, flags: HashMap<String, bool>) -> Self {
        Self {
            evaluator,
            flags: Arc::new(RwLock::new(flags)),
        }
    }

    /// Checks if a feature is enabled.
    ///
    /// This method delegates the evaluation to the configured evaluator.
    pub fn is_enabled(&self, flag_name: &str, context: &EvaluationContext) -> bool {
        self.evaluator.is_enabled(flag_name, context)
    }

    /// A simple method to update a flag's state at runtime.
    /// In a real application, this would likely involve a more complex
    /// mechanism, such as a background thread polling a remote service.
    pub fn update_flag(&self, flag_name: String, enabled: bool) {
        let mut flags = self.flags.write().unwrap();
        flags.insert(flag_name, enabled);
    }
} 