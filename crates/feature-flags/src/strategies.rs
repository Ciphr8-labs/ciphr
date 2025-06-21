use crate::evaluator::{EvaluationContext, FeatureFlagEvaluator};
use std::collections::HashMap;
use rand::Rng;

/// An evaluator that enables features based on a percentage rollout.
///
/// This is a simple example and does not provide consistent user targeting.
/// A more robust implementation would use a hashing algorithm on the user ID.
pub struct PercentageRolloutEvaluator {
    percentages: HashMap<String, f32>,
}

impl PercentageRolloutEvaluator {
    pub fn new(percentages: HashMap<String, f32>) -> Self {
        Self { percentages }
    }
}

impl FeatureFlagEvaluator for PercentageRolloutEvaluator {
    fn is_enabled(&self, flag_name: &str, _context: &EvaluationContext) -> bool {
        let percentage = self.percentages.get(flag_name).copied().unwrap_or(0.0);
        if percentage >= 1.0 {
            return true;
        }
        if percentage <= 0.0 {
            return false;
        }
        
        // In a real-world scenario, you would use a stable hash of the
        // user_id from the context to ensure a user consistently sees
        // the same variation. For this example, we use random assignment.
        let mut rng = rand::thread_rng();
        rng.gen_bool(percentage as f64)
    }
}

/// An evaluator that enables features for specific user segments.
pub struct UserSegmentEvaluator {
    /// A map of feature flags to the set of user segments they are enabled for.
    segments: HashMap<String, Vec<String>>,
}

impl UserSegmentEvaluator {
    pub fn new(segments: HashMap<String, Vec<String>>) -> Self {
        Self { segments }
    }
}

impl FeatureFlagEvaluator for UserSegmentEvaluator {
    fn is_enabled(&self, flag_name: &str, context: &EvaluationContext) -> bool {
        if let Some(enabled_segments) = self.segments.get(flag_name) {
            if let Some(user_segment) = &context.user_segment {
                return enabled_segments.contains(user_segment);
            }
        }
        false
    }
} 