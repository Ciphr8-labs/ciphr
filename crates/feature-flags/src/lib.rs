pub mod evaluator;
pub mod manager;
pub mod strategies;

#[cfg(test)]
mod tests {
    use super::evaluator::{EvaluationContext, FeatureFlagEvaluator};
    use super::strategies::PercentageRolloutEvaluator;
    use std::collections::HashMap;

    #[test]
    fn percentage_rollout_evaluator_handles_all_or_nothing() {
        let mut percentages = HashMap::new();
        percentages.insert("always_on".to_string(), 1.0);
        percentages.insert("always_off".to_string(), 0.0);

        let evaluator = PercentageRolloutEvaluator::new(percentages);
        let context = EvaluationContext::default();

        assert!(evaluator.is_enabled("always_on", &context));
        assert!(!evaluator.is_enabled("always_off", &context));
        assert!(!evaluator.is_enabled("not_configured", &context));
    }
}
