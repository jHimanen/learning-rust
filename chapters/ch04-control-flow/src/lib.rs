//! Chapter 4 exercises: functions & control flow.
//!
//! Replace every `todo!()` with an implementation. Everything here works on plain numbers:
//! no arrays, vectors or strings yet. Where a doc comment names a construct (`loop`, a
//! labeled block...), use it: that's the thing being practiced.
//!
//! Check yourself with `cargo test -p ch04-control-flow` (or `--test <group>` for one group).
//! Remember to drop the leading `_` from a parameter name once you use it.

// ===========================================================================
// `if` as a value                                      (tests/expressions.rs)
// ===========================================================================

/// Converts an exam score (0 to 100) to a letter grade:
///
/// | score | grade |
/// |---|---|
/// | 90 and above | `'A'` |
/// | 80 to 89 | `'B'` |
/// | 70 to 79 | `'C'` |
/// | 60 to 69 | `'D'` |
/// | below 60 | `'F'` |
///
/// Write the body as a **single `if` expression**: no `return`, no `let`.
pub fn grade(_score: u32) -> char {
    todo!()
}

/// Returns `true` if `year` is a leap year in the Gregorian calendar.
///
/// The rule: a year is a leap year if it's divisible by 4, **except** century years
/// (divisible by 100), which are leap years only if they're also divisible by 400. So 2024 is
/// a leap year, 1900 isn't and 2000 is.
///
/// Try it twice: once as an `if`/`else if` chain, and once as a single `bool` expression with
/// `&&` and `||` (chapter 3). Keep whichever reads better to you.
pub fn is_leap_year(_year: u32) -> bool {
    todo!()
}

/// Returns the median of three numbers: the one in the middle once they're sorted.
///
/// `median3(3, 1, 2)` is `2`, and `median3(5, 5, 1)` is `5`.
///
/// Use `if` expressions and comparisons only: no `.max()`, `.min()`, arrays or sorting.
pub fn median3(_a: i32, _b: i32, _c: i32) -> i32 {
    todo!()
}

// ===========================================================================
// Loops and ranges                                           (tests/loops.rs)
// ===========================================================================

/// Returns `n!` = 1 × 2 × ... × `n`. By definition, `0!` is 1.
///
/// Use a `for` loop over a range. You may assume `n <= 20` (21! doesn't fit in a `u64`).
pub fn factorial(_n: u32) -> u64 {
    todo!()
}

/// Returns how many decimal digits `n` has: `count_digits(4096)` is `4`.
///
/// Careful: `0` has one digit. A `while` loop is the obvious tool, and it gets `0` wrong.
/// Rust has no `do ... while`, but a `loop` with a `break` at the end of its body does the
/// same job.
pub fn count_digits(_n: u64) -> u32 {
    todo!()
}

/// Returns the greatest common divisor of `a` and `b`, with Euclid's algorithm:
///
/// > While `b` isn't zero, replace (`a`, `b`) with (`b`, `a % b`). Then `a` is the answer.
///
/// `gcd(48, 18)` is `6`. `gcd(a, 0)` is `a`, and `gcd(0, 0)` is `0`.
///
/// Use a `while` loop. (You'll need a temporary variable to do the replacement.)
pub fn gcd(_a: u64, _b: u64) -> u64 {
    todo!()
}

/// Counts the steps of the Collatz sequence from `n` down to 1.
///
/// One step: if `n` is even, halve it; otherwise, replace it with `3n + 1`. Starting at 6:
/// 6 → 3 → 10 → 5 → 16 → 8 → 4 → 2 → 1 is **8** steps. Starting at 1 takes 0 steps.
///
/// You may assume `n >= 1`. (Nobody has proved that every start reaches 1, but every number
/// anyone has tried does.)
pub fn collatz_steps(_n: u64) -> u32 {
    todo!()
}

/// Returns `true` if `n` is prime: greater than 1, and divisible only by 1 and itself.
///
/// Use a loop that **returns early** as soon as it finds a divisor.
///
/// It must also be fast: the tests include 4_294_967_291, the largest prime that fits in a
/// `u32`. Trying every divisor up to `n` means 4 billion divisions (minutes in a debug build).
/// If `n` has a divisor, then it has one that is no bigger than √n. Why? And how do you write
/// "d ≤ √n" without floats? Watch for overflow (chapter 3) in whatever you come up with.
pub fn is_prime(_n: u32) -> bool {
    todo!()
}

/// Returns the smallest power of two (1, 2, 4, 8, ...) that is `>= n`.
///
/// `smallest_power_of_two_at_least(5)` is `8`, and `smallest_power_of_two_at_least(8)` is `8`.
/// You may assume `n <= 2^63`, so the answer always fits.
///
/// Make the whole function body a single `loop` that produces the answer with `break value`.
/// (Don't call `u64::next_power_of_two`, which does this for you.)
pub fn smallest_power_of_two_at_least(_n: u64) -> u64 {
    todo!()
}

/// ★ Returns the integer square root of `n`: the largest `r` with `r * r <= n`.
///
/// `isqrt(15)` is `3`, `isqrt(16)` is `4`.
///
/// Three traps, and the tests check all of them:
/// - Counting up from 0 is correct, but for `n = u64::MAX` it takes 4 billion steps.
///   Use **binary search**: keep a range `lo..=hi` that must contain the answer, and halve
///   it on every step.
/// - `(n as f64).sqrt() as u64` is fast, but `f64` has only 53 bits of precision
///   (chapter 3), so it's wrong for large `n`.
/// - `mid * mid` can overflow a `u64`. How can you test `mid * mid <= n` without computing
///   `mid * mid`?
///
/// (Don't call `u64::isqrt`.)
pub fn isqrt(_n: u64) -> u64 {
    todo!()
}

// ===========================================================================
// Nested loops and labels                                   (tests/labels.rs)
// ===========================================================================

/// Returns the smallest positive number that is divisible by every number from 1 to `k`.
///
/// `smallest_multiple(4)` is `12` (divisible by 1, 2, 3 and 4). `smallest_multiple(1)` is `1`.
///
/// Use brute force: try every candidate `n` from 1 upwards (`1..` is a range with no end),
/// and for each candidate, loop over the divisors. As soon as one divisor fails, move on to the
/// next candidate with a **labeled `continue`**. (The tests only go up to `k = 12`: brute force
/// is too slow beyond that, and the math that isn't slow is a different exercise.)
pub fn smallest_multiple(_k: u64) -> u64 {
    todo!()
}

/// Finds a Pythagorean triple `a < b < c` (so `a² + b² = c²`) whose perimeter
/// `a + b + c` is exactly `perimeter`, and returns the product `a * b * c`.
///
/// `pythagorean_product(12)` is `60`, from (3, 4, 5).
///
/// If there's more than one such triple, return the one with the **smallest `a`**. If there's
/// none, return `0`.
///
/// Search with two nested `for` loops over `a` and `b` (then `c` is fixed by the perimeter).
/// Write the search as a **labeled block**, `'search: { ... }`, whose value is the answer:
/// `break 'search product` when you find a triple, and `0` as the block's last expression.
/// Careful with `c = perimeter - a - b`: these are unsigned numbers.
pub fn pythagorean_product(_perimeter: u64) -> u64 {
    todo!()
}
