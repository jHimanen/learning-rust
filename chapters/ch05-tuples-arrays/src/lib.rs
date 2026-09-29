//! Chapter 5 exercises: tuples and arrays.
//!
//! Replace every `todo!()` with an implementation. Tuples and fixed-size arrays only: no
//! `Vec`, no slices, no strings. Where a doc comment names a construct (destructuring, a lookup
//! table...), use it: that's the thing being practiced.
//!
//! Check yourself with `cargo test -p ch05-tuples-arrays` (or `--test <group>` for one group).
//! Stuck on a failing test? Put a `dbg!(...)` around the value you're unsure about and rerun
//! just that test (README section 5).

// ===========================================================================
// Tuples                                                    (tests/tuples.rs)
// ===========================================================================

/// Splits a number of seconds into `(hours, minutes, seconds)`.
///
/// `to_hms(3_661)` is `(1, 1, 1)`. Hours don't wrap around at 24: `to_hms(90_000)` is
/// `(25, 0, 0)`.
pub fn to_hms(_seconds: u32) -> (u32, u32, u32) {
    todo!()
}

/// Returns the pair `(F(n), F(n + 1))` of consecutive Fibonacci numbers, where `F(0) = 0`,
/// `F(1) = 1` and `F(k) = F(k - 1) + F(k - 2)`.
///
/// `fib_pair(0)` is `(0, 1)`, and `fib_pair(6)` is `(8, 13)`.
///
/// Keep the current pair in two variables and step it forward `n` times with a **single
/// destructuring assignment**, `(a, b) = (..., ...);`, without a temporary variable (compare
/// with `gcd` in chapter 4). You may assume `n <= 92`, so that `F(n + 1)` fits in a `u64`.
pub fn fib_pair(_n: u32) -> (u64, u64) {
    todo!()
}

/// Python's `divmod(a, b)`: returns `(q, r)` where `q` is `a / b` rounded **down** (towards
/// minus infinity), and `r = a - q * b`.
///
/// `divmod(7, 2)` is `(3, 1)`, like in Python, and `divmod(-7, 2)` is `(-4, 1)`, also like in
/// Python. But Rust's `-7 / 2` is `-3`, and `-7 % 2` is `-1`: Rust's `/` rounds towards zero.
/// It follows that `r` always has the same sign as `b` (or is 0).
///
/// You may assume `b != 0`, and that the division doesn't overflow (`a` isn't `i64::MIN`
/// when `b` is `-1`).
pub fn divmod(_a: i64, _b: i64) -> (i64, i64) {
    todo!()
}

// ===========================================================================
// Arrays                                                    (tests/arrays.rs)
// ===========================================================================

/// Packs a color given as `[red, green, blue]` into one number, `0xRRGGBB`: the hex code you
/// know from CSS.
///
/// `rgb_to_hex([255, 128, 0])` is `0xFF8000`.
///
/// Start with array destructuring, `let [r, g, b] = rgb;`, then use shifts and `|` (chapter 3).
pub fn rgb_to_hex(_rgb: [u8; 3]) -> u32 {
    todo!()
}

/// The reverse of `rgb_to_hex`: unpacks `0xRRGGBB` into `[red, green, blue]`.
///
/// `hex_to_rgb(0xFF8000)` is `[255, 128, 0]`. Anything above the lowest 24 bits is ignored:
/// `hex_to_rgb(0xAB12_3456)` is `[0x12, 0x34, 0x56]`.
///
/// Build the result as an array literal, `[..., ..., ...]`.
pub fn hex_to_rgb(_hex: u32) -> [u8; 3] {
    todo!()
}

/// Counts how many times each decimal digit appears in `n`: element `d` of the result is the
/// number of `d` digits.
///
/// `digit_histogram(1_000_000)` is `[6, 1, 0, 0, 0, 0, 0, 0, 0, 0]`: six zeros and a one.
/// And `0` has one digit, a zero.
///
/// Start from an array of ten zero counters and add one per digit. (`count_digits` from
/// chapter 4 walks the digits the same way.)
pub fn digit_histogram(_n: u64) -> [u32; 10] {
    todo!()
}

/// Given a week of daily temperatures (index 0 is Monday), returns
/// `(index of the warmest day, its temperature)`.
///
/// `warmest_day([3.0, 7.5, 7.5, 1.0, 0.0, -2.0, 4.0])` is `(1, 7.5)`: on a tie, the
/// **first** warmest day wins. You may assume there's no NaN.
///
/// Loop over the array with `for (i, t) in temps.into_iter().enumerate()` (README section 3).
pub fn warmest_day(_temps: [f64; 7]) -> (usize, f64) {
    todo!()
}

// ===========================================================================
// Grids: arrays of arrays                                    (tests/grids.rs)
// ===========================================================================

/// Returns the transpose of a 3×3 matrix: row `i` of the result is column `i` of `m`.
///
/// ```text
/// 1 2 3        1 4 7
/// 4 5 6   →    2 5 8
/// 7 8 9        3 6 9
/// ```
///
/// Start from a new matrix of zeros, `[[0; 3]; 3]`, and fill it in with two nested `for`
/// loops.
pub fn transpose(_m: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    todo!()
}

/// Multiplies two 3×3 matrices: element `[i][j]` of the result is the sum over `k` of
/// `a[i][k] * b[k][j]` (row `i` of `a` times column `j` of `b`).
///
/// This is NumPy's `a @ b`, written out by hand.
pub fn mat_mul(_a: [[i64; 3]; 3], _b: [[i64; 3]; 3]) -> [[i64; 3]; 3] {
    todo!()
}

/// ★ Returns the winner of a finished tic-tac-toe game: `'X'` or `'O'` if that player has
/// three in a row (a row, a column or a diagonal), and `'.'` if nobody has. Empty squares are
/// `'.'`.
///
/// ```text
/// X O .
/// O X .      → 'X' (the diagonal)
/// O . X
/// ```
///
/// Don't write eight `if`s. Write the eight lines as **data**, a lookup table: a `const`
/// array with one element per line, each element holding the three `(row, column)`
/// coordinates of that line. Then loop over it. What's the type of that `const`?
/// Careful: three empty squares in a row are not a win.
///
/// You may assume the board comes from a real game, so at most one player has a line.
pub fn tic_tac_toe_winner(_board: [[char; 3]; 3]) -> char {
    todo!()
}
