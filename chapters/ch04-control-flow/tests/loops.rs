use ch04_control_flow::*;

#[test]
fn factorial_small() {
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(10), 3_628_800);
}

#[test]
fn factorial_of_zero_is_one() {
    // The range 1..=0 is empty, so the loop body never runs.
    assert_eq!(factorial(0), 1);
}

#[test]
fn factorial_largest_that_fits() {
    assert_eq!(factorial(20), 2_432_902_008_176_640_000);
}

#[test]
fn count_digits_normal() {
    assert_eq!(count_digits(7), 1);
    assert_eq!(count_digits(10), 2);
    assert_eq!(count_digits(99), 2);
    assert_eq!(count_digits(4096), 4);
    assert_eq!(count_digits(1_000_000), 7);
}

#[test]
fn count_digits_zero_has_one_digit() {
    assert_eq!(count_digits(0), 1);
}

#[test]
fn count_digits_max() {
    assert_eq!(count_digits(u64::MAX), 20); // 18446744073709551615
}

#[test]
fn gcd_normal() {
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(18, 48), 6);
    assert_eq!(gcd(17, 5), 1);
    assert_eq!(gcd(12, 12), 12);
    assert_eq!(gcd(1_071, 462), 21);
}

#[test]
fn gcd_with_zeros() {
    assert_eq!(gcd(7, 0), 7);
    assert_eq!(gcd(0, 7), 7);
    assert_eq!(gcd(0, 0), 0);
}

#[test]
fn collatz_steps_small() {
    assert_eq!(collatz_steps(1), 0);
    assert_eq!(collatz_steps(2), 1);
    assert_eq!(collatz_steps(6), 8);
    assert_eq!(collatz_steps(16), 4);
}

#[test]
fn collatz_steps_long_runs() {
    assert_eq!(collatz_steps(27), 111); // climbs to 9232 before coming down
    assert_eq!(collatz_steps(837_799), 524);
}

#[test]
fn is_prime_small_numbers() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(!is_prime(4));
    assert!(is_prime(5));
    assert!(!is_prime(9));
    assert!(is_prime(97));
    assert!(!is_prime(91)); // 7 × 13
}

#[test]
fn is_prime_squares_of_primes() {
    // The only divisor (besides 1 and n) is exactly √n. Is your loop's bound inclusive?
    assert!(!is_prime(25));
    assert!(!is_prime(49));
    assert!(!is_prime(10_201)); // 101²
}

#[test]
fn is_prime_large() {
    assert!(is_prime(1_000_000_007));
    assert!(!is_prime(4_294_967_295)); // u32::MAX = 3 × 5 × 17 × 257 × 65537
    assert!(!is_prime(4_294_836_225)); // 65535²
}

#[test]
fn is_prime_largest_u32_prime() {
    assert!(is_prime(4_294_967_291));
}

#[test]
fn smallest_power_of_two_normal() {
    assert_eq!(smallest_power_of_two_at_least(5), 8);
    assert_eq!(smallest_power_of_two_at_least(8), 8);
    assert_eq!(smallest_power_of_two_at_least(9), 16);
    assert_eq!(smallest_power_of_two_at_least(1000), 1024);
}

#[test]
fn smallest_power_of_two_edges() {
    assert_eq!(smallest_power_of_two_at_least(0), 1);
    assert_eq!(smallest_power_of_two_at_least(1), 1);
    assert_eq!(smallest_power_of_two_at_least(2), 2);
    assert_eq!(smallest_power_of_two_at_least(3), 4);
    assert_eq!(smallest_power_of_two_at_least((1 << 62) + 1), 1 << 63);
    assert_eq!(smallest_power_of_two_at_least(1 << 63), 1 << 63);
}

#[test]
fn isqrt_small() {
    assert_eq!(isqrt(0), 0);
    assert_eq!(isqrt(1), 1);
    assert_eq!(isqrt(3), 1);
    assert_eq!(isqrt(4), 2);
    assert_eq!(isqrt(15), 3);
    assert_eq!(isqrt(16), 4);
    assert_eq!(isqrt(17), 4);
}

#[test]
fn isqrt_where_floats_go_wrong() {
    // (2^32 - 1)² = 18446744065119617025. One less than that must give 2^32 - 2.
    // Its true square root is 4294967294.99999999988..., which f64 rounds up to 2^32 - 1.
    assert_eq!(isqrt(18_446_744_065_119_617_024), 4_294_967_294);
    assert_eq!(isqrt(18_446_744_065_119_617_025), 4_294_967_295);
}

#[test]
fn isqrt_max() {
    // √(2^64 - 1) is just under 2^32. Counting up to it one by one takes far too long, and
    // squaring anything above 2^32 - 1 overflows.
    assert_eq!(isqrt(u64::MAX), 4_294_967_295);
}
