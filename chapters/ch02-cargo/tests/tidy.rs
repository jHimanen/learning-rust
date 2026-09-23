// Tests for Part 2. These already pass: they're a safety net, so you can tidy the code
// without changing what it does.
use ch02_cargo::*;

#[test]
fn double_works() {
    assert_eq!(double(21), 42);
    assert_eq!(double(-4), -8);
    assert_eq!(double(0), 0);
}

#[test]
fn larger_works() {
    assert_eq!(larger(1, 2), 2);
    assert_eq!(larger(5, -5), 5);
    assert_eq!(larger(3, 3), 3);
}

#[test]
fn is_positive_works() {
    assert!(is_positive(1));
    assert!(!is_positive(0));
    assert!(!is_positive(-1));
}
