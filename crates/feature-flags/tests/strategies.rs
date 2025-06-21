use feature_flags::evaluator::{EvaluationContext, FeatureFlagEvaluator};
use feature_flags::strategies::{PercentageRolloutEvaluator, UserSegmentEvaluator};
use std::collections::HashMap;

#[test]
fn test_user_segment_evaluator_enabled_for_segment() {
    let mut segments = HashMap::new();
    segments.insert("new_feature".to_string(), vec!["beta_testers".to_string()]);
    let evaluator = UserSegmentEvaluator::new(segments);

    let mut context = EvaluationContext::default();
    context.user_segment = Some("beta_testers".to_string());

    assert!(evaluator.is_enabled("new_feature", &context));
}

#[test]
fn test_user_segment_evaluator_not_enabled_for_different_segment() {
    let mut segments = HashMap::new();
    segments.insert("new_feature".to_string(), vec!["beta_testers".to_string()]);
    let evaluator = UserSegmentEvaluator::new(segments);

    let mut context = EvaluationContext::default();
    context.user_segment = Some("internal_users".to_string());

    assert!(!evaluator.is_enabled("new_feature", &context));
}

#[test]
fn test_user_segment_evaluator_no_segment_in_context() {
    let mut segments = HashMap::new();
    segments.insert("new_feature".to_string(), vec!["beta_testers".to_string()]);
    let evaluator = UserSegmentEvaluator::new(segments);

    let context = EvaluationContext::default();

    assert!(!evaluator.is_enabled("new_feature", &context));
}

#[test]
fn test_percentage_rollout_100_percent() {
    let mut percentages = HashMap::new();
    percentages.insert("new_feature".to_string(), 1.0);
    let evaluator = PercentageRolloutEvaluator::new(percentages);
    let context = EvaluationContext::default();
    assert!(evaluator.is_enabled("new_feature", &context));
}

#[test]
fn test_percentage_rollout_0_percent() {
    let mut percentages = HashMap::new();
    percentages.insert("new_feature".to_string(), 0.0);
    let evaluator = PercentageRolloutEvaluator::new(percentages);
    let context = EvaluationContext::default();
    assert!(!evaluator.is_enabled("new_feature", &context));
} 