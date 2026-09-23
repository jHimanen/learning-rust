use ch03_scalars::*;

#[test]
fn celsius_to_fahrenheit_known_points() {
    assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
}

#[test]
fn celsius_to_fahrenheit_body_temperature() {
    assert!(approx_eq(celsius_to_fahrenheit(37.0), 98.6, 1e-9));
}

#[test]
fn approx_eq_the_classic() {
    assert!(approx_eq(0.1 + 0.2, 0.3, 1e-9));
    assert!(approx_eq(0.3, 0.1 + 0.2, 1e-9)); // order doesn't matter
}

#[test]
fn approx_eq_respects_tolerance() {
    assert!(!approx_eq(1.0, 1.1, 1e-9));
    assert!(approx_eq(1.0, 1.05, 0.1));
    assert!(!approx_eq(1.0, 1.2, 0.1));
    assert!(!approx_eq(-1.0, 1.0, 0.5));
}

#[test]
fn is_nan_manual_detects_nan() {
    assert!(is_nan_manual(f64::NAN));
    assert!(is_nan_manual(f64::INFINITY - f64::INFINITY));
}

#[test]
fn is_nan_manual_rejects_numbers() {
    assert!(!is_nan_manual(0.0));
    assert!(!is_nan_manual(-1.5));
    assert!(!is_nan_manual(f64::INFINITY));
    assert!(!is_nan_manual(f64::MAX));
}
