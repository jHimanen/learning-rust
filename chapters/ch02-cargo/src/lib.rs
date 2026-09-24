//! Chapter 2 warm-up: practice the exercise loop.
//!
//! Part 1: replace each `todo!()` with a real implementation, then run
//! `cargo test -p ch02-cargo` until everything passes.
//!
//! Part 2: the functions at the bottom already work, but `cargo fmt` and
//! `cargo clippy` both have opinions about them.

// ---------------------------------------------------------------------------
// Part 1: make the tests pass
// ---------------------------------------------------------------------------

/// Returns the answer to life, the universe and everything: `42`.
pub fn answer() -> i32 {
    42
}

/// Returns the sum of `a` and `b`.
///
/// (The leading underscore in `_a` and `_b` tells the compiler "unused on purpose", which
/// keeps it quiet while the body is still `todo!()`. Rename them when you use them.)
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// ---------------------------------------------------------------------------
// Part 2: these work, but they're messy. Tidy them up with cargo fmt and cargo clippy.
// The tests in tests/tidy.rs already pass, and must still pass after your changes.
// ---------------------------------------------------------------------------

/// Returns `x` doubled.
pub fn double(x: i32) -> i32 {
    x * 2
}

/// Returns the larger of `a` and `b`.
pub fn larger(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

/// Returns `true` if `x` is greater than zero.
pub fn is_positive(x: i32) -> bool {
    x > 0
}

