use ch05_tuples_arrays::*;

#[test]
fn rgb_to_hex_normal() {
    assert_eq!(rgb_to_hex([255, 128, 0]), 0xFF8000);
    assert_eq!(rgb_to_hex([0x12, 0x34, 0x56]), 0x123456);
}

#[test]
fn rgb_to_hex_extremes() {
    assert_eq!(rgb_to_hex([0, 0, 0]), 0x000000);
    assert_eq!(rgb_to_hex([255, 255, 255]), 0xFFFFFF);
    assert_eq!(rgb_to_hex([255, 0, 0]), 0xFF0000);
    assert_eq!(rgb_to_hex([0, 0, 255]), 0x0000FF);
}

#[test]
fn hex_to_rgb_normal() {
    assert_eq!(hex_to_rgb(0xFF8000), [255, 128, 0]);
    assert_eq!(hex_to_rgb(0x123456), [0x12, 0x34, 0x56]);
    assert_eq!(hex_to_rgb(0x000000), [0, 0, 0]);
    assert_eq!(hex_to_rgb(0xFFFFFF), [255, 255, 255]);
}

#[test]
fn hex_to_rgb_ignores_high_bits() {
    assert_eq!(hex_to_rgb(0xAB12_3456), [0x12, 0x34, 0x56]);
    assert_eq!(hex_to_rgb(u32::MAX), [255, 255, 255]);
}

#[test]
fn rgb_round_trip() {
    for color in [[1, 2, 3], [200, 100, 50], [0, 255, 0], [17, 0, 254]] {
        assert_eq!(hex_to_rgb(rgb_to_hex(color)), color);
    }
}

#[test]
fn digit_histogram_normal() {
    assert_eq!(digit_histogram(1_000_000), [6, 1, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(digit_histogram(9_876_543_210), [1; 10]);
    assert_eq!(digit_histogram(7_777), [0, 0, 0, 0, 0, 0, 0, 4, 0, 0]);
}

#[test]
fn digit_histogram_single_digits() {
    assert_eq!(digit_histogram(5), [0, 0, 0, 0, 0, 1, 0, 0, 0, 0]);
    assert_eq!(digit_histogram(9), [0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
}

#[test]
fn digit_histogram_zero_has_one_zero_digit() {
    assert_eq!(digit_histogram(0), [1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn digit_histogram_max() {
    // u64::MAX = 18446744073709551615
    assert_eq!(digit_histogram(u64::MAX), [2, 3, 0, 1, 4, 3, 2, 3, 1, 1]);
}

#[test]
fn warmest_day_normal() {
    assert_eq!(warmest_day([3.0, 5.5, 9.0, 1.0, 0.0, -2.0, 4.0]), (2, 9.0));
}

#[test]
fn warmest_day_first_and_last() {
    assert_eq!(warmest_day([30.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]), (0, 30.0));
    assert_eq!(warmest_day([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 30.0]), (6, 30.0));
}

#[test]
fn warmest_day_tie_goes_to_the_first() {
    assert_eq!(warmest_day([3.0, 7.5, 7.5, 1.0, 0.0, -2.0, 7.5]), (1, 7.5));
    assert_eq!(warmest_day([2.0; 7]), (0, 2.0));
}

#[test]
fn warmest_day_all_below_zero() {
    // A Finnish January week. What's a safe starting value for "the warmest so far"?
    assert_eq!(
        warmest_day([-12.0, -8.5, -15.0, -20.0, -9.0, -8.6, -30.0]),
        (1, -8.5)
    );
}
