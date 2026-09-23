use ch03_scalars::*;

#[test]
fn is_even_works() {
    assert!(is_even(0));
    assert!(is_even(2));
    assert!(is_even(1_000_000));
    assert!(!is_even(1));
    assert!(!is_even(7));
    assert!(!is_even(u32::MAX));
}

#[test]
fn set_bit_works() {
    assert_eq!(set_bit(0b0000_0001, 3), 0b0000_1001);
    assert_eq!(set_bit(0b0000_0000, 7), 0b1000_0000);
    assert_eq!(set_bit(0b0000_1000, 3), 0b0000_1000); // already set
}

#[test]
fn clear_bit_works() {
    assert_eq!(clear_bit(0b1111_1111, 0), 0b1111_1110);
    assert_eq!(clear_bit(0b1000_0001, 7), 0b0000_0001);
    assert_eq!(clear_bit(0b0000_0000, 4), 0b0000_0000); // already clear
}

#[test]
fn is_bit_set_works() {
    assert!(is_bit_set(0b0000_0100, 2));
    assert!(!is_bit_set(0b0000_0100, 1));
    assert!(is_bit_set(0b1000_0000, 7));
    assert!(!is_bit_set(0b0111_1111, 7));
}

#[test]
fn pack_rgb_works() {
    assert_eq!(pack_rgb(0x12, 0x34, 0x56), 0x0012_3456);
    assert_eq!(pack_rgb(0xFF, 0x00, 0x00), 0x00FF_0000);
    assert_eq!(pack_rgb(0x00, 0x00, 0xFF), 0x0000_00FF);
    assert_eq!(pack_rgb(0xFF, 0xFF, 0xFF), 0x00FF_FFFF);
}

#[test]
fn red_works() {
    assert_eq!(red(0x0012_3456), 0x12);
    assert_eq!(red(0x00FF_0000), 0xFF);
    assert_eq!(red(0x0000_FFFF), 0x00);
}

#[test]
fn red_roundtrips_with_pack_rgb() {
    assert_eq!(red(pack_rgb(0xAB, 0xCD, 0xEF)), 0xAB);
}

#[test]
fn is_power_of_two_true_cases() {
    assert!(is_power_of_two(1));
    assert!(is_power_of_two(2));
    assert!(is_power_of_two(64));
    assert!(is_power_of_two(1 << 31));
}

#[test]
fn is_power_of_two_false_cases() {
    assert!(!is_power_of_two(0));
    assert!(!is_power_of_two(3));
    assert!(!is_power_of_two(12));
    assert!(!is_power_of_two(u32::MAX));
}
