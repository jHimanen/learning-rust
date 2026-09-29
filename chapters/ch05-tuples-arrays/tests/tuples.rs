use ch05_tuples_arrays::*;

#[test]
fn to_hms_normal() {
    assert_eq!(to_hms(3_661), (1, 1, 1));
    assert_eq!(to_hms(59), (0, 0, 59));
    assert_eq!(to_hms(60), (0, 1, 0));
    assert_eq!(to_hms(86_399), (23, 59, 59));
}

#[test]
fn to_hms_zero() {
    assert_eq!(to_hms(0), (0, 0, 0));
}

#[test]
fn to_hms_hours_dont_wrap() {
    assert_eq!(to_hms(90_000), (25, 0, 0));
}

#[test]
fn to_hms_destructures() {
    // A returned tuple can be unpacked straight into variables, like in Python.
    let (h, m, s) = to_hms(45_296);
    assert_eq!(h, 12);
    assert_eq!(m, 34);
    assert_eq!(s, 56);
}

#[test]
fn fib_pair_start() {
    assert_eq!(fib_pair(0), (0, 1));
    assert_eq!(fib_pair(1), (1, 1));
    assert_eq!(fib_pair(2), (1, 2));
}

#[test]
fn fib_pair_normal() {
    assert_eq!(fib_pair(6), (8, 13));
    assert_eq!(fib_pair(10), (55, 89));
    assert_eq!(fib_pair(50), (12_586_269_025, 20_365_011_074));
}

#[test]
fn fib_pair_largest_that_fits() {
    assert_eq!(
        fib_pair(92),
        (7_540_113_804_746_346_429, 12_200_160_415_121_876_738)
    );
}

#[test]
fn divmod_positive() {
    assert_eq!(divmod(7, 2), (3, 1));
    assert_eq!(divmod(6, 3), (2, 0));
    assert_eq!(divmod(0, 5), (0, 0));
    assert_eq!(divmod(2, 7), (0, 2));
}

#[test]
fn divmod_negative_dividend() {
    // Python: divmod(-7, 2) == (-4, 1). Rust's -7 / 2 is -3 and -7 % 2 is -1.
    assert_eq!(divmod(-7, 2), (-4, 1));
    assert_eq!(divmod(-6, 3), (-2, 0));
    assert_eq!(divmod(-1, 10), (-1, 9));
}

#[test]
fn divmod_negative_divisor() {
    // Python: divmod(7, -2) == (-4, -1). The remainder takes the sign of the divisor.
    assert_eq!(divmod(7, -2), (-4, -1));
    assert_eq!(divmod(-7, -2), (3, -1));
    assert_eq!(divmod(1, -10), (-1, -9));
}

#[test]
fn divmod_matches_pythons_rules() {
    // Python guarantees, for every a and every b != 0:
    //   q * b + r == a,  and r is 0 or has the sign of b, with |r| < |b|.
    for a in -30..=30 {
        for b in [-7, -3, -2, -1, 1, 2, 3, 7] {
            let (q, r) = divmod(a, b);
            assert_eq!(q * b + r, a, "divmod({a}, {b}) returned ({q}, {r})");
            let sign_ok = if b > 0 {
                0 <= r && r < b
            } else {
                b < r && r <= 0
            };
            assert!(sign_ok, "divmod({a}, {b}) returned ({q}, {r})");
        }
    }
}

#[test]
fn divmod_large() {
    assert_eq!(divmod(i64::MAX, 2), (i64::MAX / 2, 1));
    assert_eq!(divmod(i64::MIN, 2), (i64::MIN / 2, 0));
    assert_eq!(divmod(i64::MIN + 1, 2), (i64::MIN / 2, 1));
}
