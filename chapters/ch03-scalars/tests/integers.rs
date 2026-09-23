use ch03_scalars::*;

#[test]
fn mean_from_sum_is_not_integer_division() {
    assert_eq!(mean_from_sum(7, 2), 3.5);
    assert_eq!(mean_from_sum(10, 4), 2.5);
}

#[test]
fn mean_from_sum_handles_negative_sums() {
    assert_eq!(mean_from_sum(-3, 2), -1.5);
    assert_eq!(mean_from_sum(0, 5), 0.0);
}

#[test]
fn low_byte_keeps_only_the_lowest_8_bits() {
    assert_eq!(low_byte(0x1234_5678), 0x78);
    assert_eq!(low_byte(0xFF), 0xFF);
    assert_eq!(low_byte(0x100), 0x00);
    assert_eq!(low_byte(300), 44);
}

#[test]
fn as_signed_small_values_are_unchanged() {
    assert_eq!(as_signed(0), 0);
    assert_eq!(as_signed(1), 1);
    assert_eq!(as_signed(127), 127);
}

#[test]
fn as_signed_high_bit_means_negative() {
    assert_eq!(as_signed(0b1111_1111), -1);
    assert_eq!(as_signed(0b1000_0000), -128);
    assert_eq!(as_signed(200), -56);
}

#[test]
fn python_floor_div_matches_rust_for_positive_numbers() {
    assert_eq!(python_floor_div(7, 2), 3);
    assert_eq!(python_floor_div(6, 3), 2);
    assert_eq!(python_floor_div(0, 5), 0);
}

#[test]
fn python_floor_div_rounds_down_for_negative_numbers() {
    assert_eq!(python_floor_div(-7, 2), -4); // Rust's -7 / 2 would give -3
    assert_eq!(python_floor_div(-1, 3), -1);
    assert_eq!(python_floor_div(-6, 3), -2);
}
