use crate::evaluator::{EvaluationContext, FeatureFlagEvaluator};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use siphasher::sip::SipHasher;

/// An evaluator that enables features based on a percentage rollout.
///
/// This implementation uses a hashing algorithm on the user ID to provide
/// consistent user targeting.
pub struct PercentageRolloutEvaluator {
    percentages: HashMap<String, f32>,
}

impl PercentageRolloutEvaluator {
    pub fn new(percentages: HashMap<String, f32>) -> Self {
        Self { percentages }
    }
}

impl FeatureFlagEvaluator for PercentageRolloutEvaluator {
    fn is_enabled(&self, flag_name: &str, context: &EvaluationContext) -> bool {
        let percentage = self.percentages.get(flag_name).copied().unwrap_or(0.0);
        if percentage >= 1.0 {
            return true;
        }
        if percentage <= 0.0 {
            return false;
        }

        let user_id = match &context.user_id {
            Some(id) => id,
            None => return false, // No user ID, no percentage rollout
        };

        let mut hasher = SipHasher::new();
        flag_name.hash(&mut hasher);
        user_id.hash(&mut hasher);
        let hash_result = hasher.finish();

        // Use the hash to determine if the user is in the percentage.
        // We can take the hash modulo 100 to get a value from 0-99.
        let rollout_value = (hash_result % 100) as f32;
        rollout_value < (percentage * 100.0)
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