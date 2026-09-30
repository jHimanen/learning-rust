//! Milestone m04: the first statistics functions, in minipolars' library crate.
//!
//! Copy this file to `project/minipolars/tests/` and run `cargo test -p minipolars`.
//! Don't edit it: it's the spec.

use minipolars::{max, mean, min, std_dev, sum, variance};

/// Floats are compared with a tolerance (chapter 3).
fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-9,
        "expected {expected}, got {actual}"
    );
}

// The classic example from statistics textbooks: mean 5, sample variance 32/7.
const DATA: &[f64] = &[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

#[test]
fn sum_values() {
    assert_close(sum(DATA), 40.0);
    assert_close(sum(&[1.5, 2.5, -3.0]), 1.0);
    assert_close(sum(&[42.0]), 42.0);
}

#[test]
fn sum_of_nothing_is_zero() {
    assert_eq!(sum(&[]), 0.0);
}

#[test]
fn mean_values() {
    assert_close(mean(DATA), 5.0);
    assert_close(mean(&[1.0, 2.0, 3.0, 4.0]), 2.5);
    assert_close(mean(&[-1.0, 1.0]), 0.0);
}

#[test]
fn mean_of_nothing_is_nan() {
    assert!(mean(&[]).is_nan());
}

#[test]
fn min_and_max_values() {
    assert_eq!(min(DATA), 2.0);
    assert_eq!(max(DATA), 9.0);
    assert_eq!(min(&[3.0, -1.0, 7.0, 2.0]), -1.0);
    assert_eq!(max(&[3.0, -1.0, 7.0, 2.0]), 7.0);
    assert_eq!(min(&[42.0]), 42.0);
    assert_eq!(max(&[42.0]), 42.0);
}

#[test]
fn min_and_max_all_positive_or_all_negative() {
    // A common bug: starting the search at 0.0.
    assert_eq!(max(&[-5.0, -3.0, -9.0]), -3.0);
    assert_eq!(min(&[5.0, 3.0, 9.0]), 3.0);
}

#[test]
fn min_and_max_of_nothing() {
    assert_eq!(min(&[]), f64::INFINITY);
    assert_eq!(max(&[]), f64::NEG_INFINITY);
}

#[test]
fn variance_is_the_sample_variance() {
    // Divided by n - 1 = 7, not by n = 8.
    assert_close(variance(DATA), 32.0 / 7.0);
    assert_close(variance(&[1.0, 2.0]), 0.5);
    assert_close(variance(&[3.0, 3.0, 3.0]), 0.0);
}

#[test]
fn variance_needs_two_values() {
    assert!(variance(&[42.0]).is_nan());
    assert!(variance(&[]).is_nan());
}

#[test]
fn std_dev_values() {
    assert_close(std_dev(DATA), (32.0_f64 / 7.0).sqrt());
    assert_close(std_dev(&[1.0, 3.0]), 2.0_f64.sqrt());
    assert!(std_dev(&[42.0]).is_nan());
    assert!(std_dev(&[]).is_nan());
}
