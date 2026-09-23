use ch03_scalars::*;

#[test]
fn average_u8_simple() {
    assert_eq!(average_u8(10, 20), 15);
    assert_eq!(average_u8(0, 1), 0); // rounded down
    assert_eq!(average_u8(3, 3), 3);
}

#[test]
fn average_u8_does_not_overflow() {
    assert_eq!(average_u8(200, 100), 150);
    assert_eq!(average_u8(255, 255), 255);
    assert_eq!(average_u8(255, 0), 127);
}

#[test]
fn volume_up_normal() {
    assert_eq!(volume_up(10, 5), 15);
    assert_eq!(volume_up(0, 0), 0);
}

#[test]
fn volume_up_stops_at_max() {
    assert_eq!(volume_up(250, 10), 255);
    assert_eq!(volume_up(255, 1), 255);
    assert_eq!(volume_up(255, 255), 255);
}

#[test]
fn counter_advance_normal() {
    assert_eq!(counter_advance(0, 1), 1);
    assert_eq!(counter_advance(1000, 234), 1234);
}

#[test]
fn counter_advance_wraps_around() {
    assert_eq!(counter_advance(65535, 1), 0);
    assert_eq!(counter_advance(65530, 10), 4);
    assert_eq!(counter_advance(u16::MAX, u16::MAX), 65534);
}

#[test]
fn abs_difference_both_orders() {
    assert_eq!(abs_difference(3, 10), 7);
    assert_eq!(abs_difference(10, 3), 7);
    assert_eq!(abs_difference(5, 5), 0);
}

#[test]
fn abs_difference_extremes() {
    assert_eq!(abs_difference(0, u32::MAX), u32::MAX);
    assert_eq!(abs_difference(u32::MAX, 0), u32::MAX);
}

#[test]
fn negate_wrapping_normal() {
    assert_eq!(negate_wrapping(5), -5);
    assert_eq!(negate_wrapping(-5), 5);
    assert_eq!(negate_wrapping(0), 0);
    assert_eq!(negate_wrapping(127), -127);
}

#[test]
fn negate_wrapping_most_negative() {
    // +128 doesn't fit in an i8, whose range is -128..=127.
    assert_eq!(negate_wrapping(-128), -128);
}
