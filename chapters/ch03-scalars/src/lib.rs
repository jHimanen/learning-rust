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
pub fn mean_from_sum(sum: i64, count: u32) -> f64 {
    sum as f64 / count as f64
}

/// Returns the lowest 8 bits of `x`, i.e. the least significant byte.
///
/// `low_byte(0x1234_5678)` is `0x78`.
pub fn low_byte(x: u32) -> u8 {
    x as u8
}

/// Reinterprets the bits of an unsigned byte as a signed byte (two's complement).
///
/// `as_signed(0b1111_1111)` is `-1`.
pub fn as_signed(x: u8) -> i8 {
    x as i8
}

/// Integer division that rounds **down** (towards negative infinity), like Python's `//`.
///
/// Rust's `/` on integers rounds towards zero instead, so `-7 / 2 == -3` in Rust, but
/// `-7 // 2 == -4` in Python. You may assume `b > 0`.
///
/// Hint: there's a method on `i32` for exactly this. Browse the i32 docs (`K` on `i32` in
/// nvim, or `rustup doc --std` and search for "i32") and look for one with "euclid" in
/// its name.
pub fn python_floor_div(a: i32, b: i32) -> i32 {
    a.div_euclid(b)
}

// ===========================================================================
// Overflow                                                (tests/overflow.rs)
// ===========================================================================

/// Returns the average of two bytes, rounded down.
///
/// Careful: `200 + 100` doesn't fit in a `u8`. The result always fits, though.
pub fn average_u8(a: u8, b: u8) -> u8 {
    let sum = a as u32 + b as u32;
    sum.div_euclid(2) as u8
}

/// Turns the volume up by `step`, but never past the maximum (255).
///
/// `volume_up(250, 10)` is `255`.
pub fn volume_up(volume: u8, step: u8) -> u8 {
    volume.saturating_add(step)
}

/// Advances a 16-bit counter by `steps`, wrapping around to 0 after 65535, like an
/// odometer that rolls over.
///
/// `counter_advance(65535, 1)` is `0`.
pub fn counter_advance(count: u16, steps: u16) -> u16 {
    count.wrapping_add(steps)
}

/// Returns the distance between `a` and `b`, i.e. `|a - b|`, without ever overflowing.
///
/// `abs_difference(3, 10)` and `abs_difference(10, 3)` are both `7`.
pub fn abs_difference(a: u32, b: u32) -> u32 {
    a.abs_diff(b)
}

/// Negates `x`, wrapping on overflow instead of panicking.
///
/// `negate_wrapping(5)` is `-5`. What should `negate_wrapping(-128)` be, and why?
pub fn negate_wrapping(x: i8) -> i8 {
    x.wrapping_neg()
}

// ===========================================================================
// Bits                                                        (tests/bits.rs)
// ===========================================================================
// Bit positions count from 0 = the least significant (rightmost) bit.

/// Returns `true` if `n` is even. Use a bitwise operator, not `%`.
pub fn is_even(n: u32) -> bool {
    n & 1 == 0
}

/// Returns `n` with bit `i` set to 1. (`i < 8`)
///
/// `set_bit(0b0000_0001, 3)` is `0b0000_1001`.
pub fn set_bit(n: u8, i: u32) -> u8 {
    n | (1 << i)
}

/// Returns `n` with bit `i` cleared to 0. (`i < 8`)
///
/// `clear_bit(0b1111_1111, 0)` is `0b1111_1110`.
pub fn clear_bit(n: u8, i: u32) -> u8 {
    n & !(1 << i)
}

/// Returns `true` if bit `i` of `n` is 1. (`i < 8`)
pub fn is_bit_set(n: u8, i: u32) -> bool {
    n & (1 << i) != 0
}

/// Packs three color channels into one `u32` laid out as `0x00RRGGBB`.
///
/// `pack_rgb(0x12, 0x34, 0x56)` is `0x0012_3456`.
pub fn pack_rgb(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) << 16 | (g as u32) << 8 | b as u32
}

/// Extracts the red channel from a color packed as `0x00RRGGBB`.
///
/// `red(0x0012_3456)` is `0x12`.
pub fn red(color: u32) -> u8 {
    (color >> 16) as u8
}

/// ★ Returns `true` if `n` is a power of two (1, 2, 4, 8, ...). Zero is not a power of two.
///
/// Don't call `u32::is_power_of_two`. Write down a few powers of two in binary, then do the
/// same for each of them minus one. What do you notice? Then think about the edge case `0`.
pub fn is_power_of_two(n: u32) -> bool {
    n != 0 && n & (n - 1) == 0
}

// ===========================================================================
// Floats                                                    (tests/floats.rs)
// ===========================================================================

/// Converts degrees Celsius to degrees Fahrenheit: `F = C × 9/5 + 32`.
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

/// Returns `true` if `a` and `b` differ by less than `tolerance`.
///
/// This is how you compare floats: `0.1 + 0.2 == 0.3` is `false`!
pub fn approx_eq(a: f64, b: f64, tolerance: f64) -> bool {
    (a - b).abs() < tolerance
}

/// ★ Returns `true` if `x` is NaN, without calling `x.is_nan()`.
///
/// NaN has one property that no other value has. Look at the float demo.
///
/// Clippy will reject the natural solution with an error (lint `clippy::eq_op`). Read its
/// explanation. Since this exercise is about exactly that trick, you may silence the lint
/// by putting `#[allow(clippy::eq_op)]` on the line above `pub fn` (like in `demo_floats.rs`).
#[allow(clippy::eq_op)]
pub fn is_nan_manual(x: f64) -> bool {
    x != x
}

// ===========================================================================
// Chars & bytes                                              (tests/chars.rs)
// ===========================================================================

/// Returns the numeric value of an ASCII digit character: `'7'` gives `7`.
///
/// You may assume `c` is one of `'0'..='9'`. Don't use `to_digit` (that comes later). Use
/// the fact that the digits are consecutive in Unicode/ASCII.
pub fn digit_value(c: char) -> u32 {
    c as u32 - '0' as u32
}

/// Converts a lowercase ASCII letter (as a byte) to uppercase: `b'a'` gives `b'A'`.
///
/// You may assume `b` is in `b'a'..=b'z'`. Do it with a single bitwise operation. Compare
/// `b'a'` and `b'A'` in binary first.
pub fn ascii_to_upper(b: u8) -> u8 {
    clear_bit(b, 5)
}

/// Returns how many bytes `c` takes up when encoded as UTF-8 (1 to 4).
///
/// Find the right method on `char` in the docs.
pub fn utf8_len(c: char) -> usize {
    c.len_utf8()
}
