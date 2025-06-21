#![cfg(test)]
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_addition_is_commutative(a in 0..1000i32, b in 0..1000i32) {
        prop_assert_eq!(a + b, b + a);
    }
} 