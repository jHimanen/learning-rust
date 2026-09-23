//! Chapter 3 exercises: scalars & bits.
//!
//! Replace every `todo!()` with an implementation. None of these need `if`, loops or
//! anything beyond this chapter. Most are one line. The thinking is in *which* line.
//!
//! Check yourself with `cargo test -p ch03-scalars` (or `--test <group>` for one group).
//! Remember to drop the leading `_` from a parameter name once you use it.

// ===========================================================================
// Integers & casting                                     (tests/integers.rs)
// ===========================================================================

/// Returns the mean of `count` values whose sum is `sum`, as a float.
///
/// `mean_from_sum(7, 2)` is `3.5`, not `3`. You may assume `count > 0`.
pub fn mean_from_sum(_sum: i64, _count: u32) -> f64 {
    todo!()
}

/// Returns the lowest 8 bits of `x`, i.e. the least significant byte.
///
/// `low_byte(0x1234_5678)` is `0x78`.
pub fn low_byte(_x: u32) -> u8 {
    todo!()
}

/// Reinterprets the bits of an unsigned byte as a signed byte (two's complement).
///
/// `as_signed(0b1111_1111)` is `-1`.
pub fn as_signed(_x: u8) -> i8 {
    todo!()
}

/// Integer division that rounds **down** (towards negative infinity), like Python's `//`.
///
/// Rust's `/` on integers rounds towards zero instead, so `-7 / 2 == -3` in Rust, but
/// `-7 // 2 == -4` in Python. You may assume `b > 0`.
///
/// Hint: there's a method on `i32` for exactly this. Browse the i32 docs (`K` on `i32` in
/// nvim, or `rustup doc --std` and search for "i32") and look for one with "euclid" in
/// its name.
pub fn python_floor_div(_a: i32, _b: i32) -> i32 {
    todo!()
}

// ===========================================================================
// Overflow                                                (tests/overflow.rs)
// ===========================================================================

/// Returns the average of two bytes, rounded down.
///
/// Careful: `200 + 100` doesn't fit in a `u8`. The result always fits, though.
pub fn average_u8(_a: u8, _b: u8) -> u8 {
    todo!()
}

/// Turns the volume up by `step`, but never past the maximum (255).
///
/// `volume_up(250, 10)` is `255`.
pub fn volume_up(_volume: u8, _step: u8) -> u8 {
    todo!()
}

/// Advances a 16-bit counter by `steps`, wrapping around to 0 after 65535, like an
/// odometer that rolls over.
///
/// `counter_advance(65535, 1)` is `0`.
pub fn counter_advance(_count: u16, _steps: u16) -> u16 {
    todo!()
}

/// Returns the distance between `a` and `b`, i.e. `|a - b|`, without ever overflowing.
///
/// `abs_difference(3, 10)` and `abs_difference(10, 3)` are both `7`.
pub fn abs_difference(_a: u32, _b: u32) -> u32 {
    todo!()
}

/// Negates `x`, wrapping on overflow instead of panicking.
///
/// `negate_wrapping(5)` is `-5`. What should `negate_wrapping(-128)` be, and why?
pub fn negate_wrapping(_x: i8) -> i8 {
    todo!()
}

// ===========================================================================
// Bits                                                        (tests/bits.rs)
// ===========================================================================
// Bit positions count from 0 = the least significant (rightmost) bit.

/// Returns `true` if `n` is even. Use a bitwise operator, not `%`.
pub fn is_even(_n: u32) -> bool {
    todo!()
}

/// Returns `n` with bit `i` set to 1. (`i < 8`)
///
/// `set_bit(0b0000_0001, 3)` is `0b0000_1001`.
pub fn set_bit(_n: u8, _i: u32) -> u8 {
    todo!()
}

/// Returns `n` with bit `i` cleared to 0. (`i < 8`)
///
/// `clear_bit(0b1111_1111, 0)` is `0b1111_1110`.
pub fn clear_bit(_n: u8, _i: u32) -> u8 {
    todo!()
}

/// Returns `true` if bit `i` of `n` is 1. (`i < 8`)
pub fn is_bit_set(_n: u8, _i: u32) -> bool {
    todo!()
}

/// Packs three color channels into one `u32` laid out as `0x00RRGGBB`.
///
/// `pack_rgb(0x12, 0x34, 0x56)` is `0x0012_3456`.
pub fn pack_rgb(_r: u8, _g: u8, _b: u8) -> u32 {
    todo!()
}

/// Extracts the red channel from a color packed as `0x00RRGGBB`.
///
/// `red(0x0012_3456)` is `0x12`.
pub fn red(_color: u32) -> u8 {
    todo!()
}

/// ★ Returns `true` if `n` is a power of two (1, 2, 4, 8, ...). Zero is not a power of two.
///
/// Don't call `u32::is_power_of_two`. Write down a few powers of two in binary, then do the
/// same for each of them minus one. What do you notice? Then think about the edge case `0`.
pub fn is_power_of_two(_n: u32) -> bool {
    todo!()
}

// ===========================================================================
// Floats                                                    (tests/floats.rs)
// ===========================================================================

/// Converts degrees Celsius to degrees Fahrenheit: `F = C × 9/5 + 32`.
pub fn celsius_to_fahrenheit(_c: f64) -> f64 {
    todo!()
}

/// Returns `true` if `a` and `b` differ by less than `tolerance`.
///
/// This is how you compare floats: `0.1 + 0.2 == 0.3` is `false`!
pub fn approx_eq(_a: f64, _b: f64, _tolerance: f64) -> bool {
    todo!()
}

/// ★ Returns `true` if `x` is NaN, without calling `x.is_nan()`.
///
/// NaN has one property that no other value has. Look at the float demo.
///
/// Clippy will reject the natural solution with an error (lint `clippy::eq_op`). Read its
/// explanation. Since this exercise is about exactly that trick, you may silence the lint
/// by putting `#[allow(clippy::eq_op)]` on the line above `pub fn` (like in `demo_floats.rs`).
pub fn is_nan_manual(_x: f64) -> bool {
    todo!()
}

// ===========================================================================
// Chars & bytes                                              (tests/chars.rs)
// ===========================================================================

/// Returns the numeric value of an ASCII digit character: `'7'` gives `7`.
///
/// You may assume `c` is one of `'0'..='9'`. Don't use `to_digit` (that comes later). Use
/// the fact that the digits are consecutive in Unicode/ASCII.
pub fn digit_value(_c: char) -> u32 {
    todo!()
}

/// Converts a lowercase ASCII letter (as a byte) to uppercase: `b'a'` gives `b'A'`.
///
/// You may assume `b` is in `b'a'..=b'z'`. Do it with a single bitwise operation. Compare
/// `b'a'` and `b'A'` in binary first.
pub fn ascii_to_upper(_b: u8) -> u8 {
    todo!()
}

/// Returns how many bytes `c` takes up when encoded as UTF-8 (1 to 4).
///
/// Find the right method on `char` in the docs.
pub fn utf8_len(_c: char) -> usize {
    todo!()
}
