// Tests for Part 1. Each `#[test]` function is one test. `assert_eq!(left, right)` panics
// (fails the test) if the two values differ, and prints both of them.
use ch02_cargo::*;

#[test]
fn answer_is_42() {
    assert_eq!(answer(), 42);
}

#[test]
fn add_small_numbers() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn add_negative_numbers() {
    assert_eq!(add(-7, 3), -4);
    assert_eq!(add(-7, -3), -10);
}

#[test]
fn add_zero() {
    assert_eq!(add(0, 0), 0);
    assert_eq!(add(12, 0), 12);
}
