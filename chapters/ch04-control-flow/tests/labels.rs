use ch04_control_flow::*;

#[test]
fn smallest_multiple_small() {
    assert_eq!(smallest_multiple(1), 1);
    assert_eq!(smallest_multiple(2), 2);
    assert_eq!(smallest_multiple(3), 6);
    assert_eq!(smallest_multiple(4), 12);
    assert_eq!(smallest_multiple(5), 60);
}

#[test]
fn smallest_multiple_larger() {
    assert_eq!(smallest_multiple(7), 420);
    assert_eq!(smallest_multiple(10), 2_520);
    assert_eq!(smallest_multiple(12), 27_720);
}

#[test]
fn pythagorean_product_classic_triples() {
    assert_eq!(pythagorean_product(12), 60); // 3, 4, 5
    assert_eq!(pythagorean_product(24), 480); // 6, 8, 10
    assert_eq!(pythagorean_product(30), 780); // 5, 12, 13
}

#[test]
fn pythagorean_product_picks_smallest_a() {
    // Both (10, 24, 26) and (15, 20, 25) have perimeter 60.
    assert_eq!(pythagorean_product(60), 6_240);
}

#[test]
fn pythagorean_product_none() {
    assert_eq!(pythagorean_product(0), 0);
    assert_eq!(pythagorean_product(5), 0);
    assert_eq!(pythagorean_product(13), 0);
    assert_eq!(pythagorean_product(31), 0);
}

#[test]
fn pythagorean_product_thousand() {
    // The only triple with perimeter 1000 is (200, 375, 425).
    assert_eq!(pythagorean_product(1_000), 31_875_000);
}
