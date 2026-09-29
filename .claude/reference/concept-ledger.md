# Concept ledger

What each released chapter *introduces* (use it freely from then on) and *previews* (shown but
not yet explained, so exercises must not rely on it).

- **Tutor mode:** before hinting, check how far the learner has got. Stay within the chapters
  they've done. If a hint needs a later concept, say which chapter covers it.
- **Authoring mode:** don't use a concept before the chapter that introduces it. If an exercise
  really needs one, preview it briefly and say which chapter covers it properly. Add the new
  chapter's entry here when it's written.

- **ch01 (compiling):** `rustc`, source → binary, `fn main`, `println!` with `{}`, `let`,
  running a binary, debug vs `-O`, `file`/`otool -L`/`nm`, `--emit asm`, compile errors and
  `rustc --explain`, rustup/toolchains. *Previews:* `for` loops and ranges, `u64`, `mut`.
- **ch02 (cargo):** `cargo new/build/run/check/test/clean/doc/fmt/clippy/add`, `Cargo.toml`
  fields, `Cargo.lock` and why this repo ignores it, `target/{debug,release}`, profiles,
  dependencies from crates.io, workspaces and `-p`, the exercise loop (`todo!()`, reading test
  output), `#[test]`/`assert_eq!` (reading level), rust-analyzer in nvim. ★ `minipolars` binary
  printing its version. *Previews:* `env!`, `cfg!`, `use`, `pub fn`, `if`/`else` (in the
  clippy exercise), `&str`.
- **ch03 (scalars):** integer types and sizes, `usize`, literals (`1_000`, `0xff`, `0b1010`,
  `b'a'`), two's complement, overflow behavior (debug panic / release wrap), `checked_*` (as a
  concept), `wrapping_*`, `saturating_*`, `as` casts (truncation, sign), `f32`/`f64`
  (IEEE 754, NaN, precision), `bool`, `char` (4 bytes, Unicode scalar value), bitwise
  operators (`& | ^ ! << >>`), `{:b} {:x} {:#x} {:08b} {:e} {:?}` formatting,
  `std::mem::size_of`, `to_bits`/`to_le_bytes`, endianness, shadowing, `const`, reading and
  writing function bodies (last expression = return value), `&&`/`||` short-circuit, method
  calls on primitives (`x.abs()`), finding methods in the docs. *Previews:* `if` (as an
  expression, not yet used in exercises), `Option` (the returned value from `checked_*`,
  not used), `std::hint::black_box`, tuples (`overflowing_add`), `for` loops over array
  literals and `{:?}` Debug formatting (in demos), lint attributes (`#[allow(clippy::...)]`).
- **ch04 (control flow):** expressions vs statements, blocks as values, `;` and `()` (unit,
  zero-sized), `if`/`else if`/`else` as an expression (same type in every branch, `else`
  required when used as a value, no truthiness), functions: typed params, `-> T`, `return`
  for early exit, no return type = `()`, `mut` parameters, calling other functions,
  recursion (mentioned). `loop` with `break value`, `while`, `for` over ranges, `continue`,
  ranges `a..b`, `a..=b`, `a..` (and empty ranges), `.rev()`/`.step_by()` on ranges,
  labeled `break`/`continue` (`'outer:`), labeled blocks with `break 'label value`, the
  never type `!` (why `todo!()`/`panic!` fit any type), `is_multiple_of` (clippy's
  `manual_is_multiple_of`). Under the hood: ARM64 calling convention (args in `x0`, `x1`...,
  return in `x0`, `w` = low 32 bits), `csel` for `if`, loops as backward branches, debug vs
  optimized assembly. ★ minipolars gets a library crate (`src/lib.rs` next to `main.rs`).
  *Previews:* slices `&[f64]` (`.len()`, indexing, `for &x in values`, reference vs value,
  properly in ch05/ch08/ch09), `&'static str` return type (ch10), `for` desugaring to
  `loop` + `next()` (ch20), `const` slices and `assert!` with a message (in the milestone
  test), `#[inline(never)]` (in `demo_asm`), library + binary in one package (ch15).
- **ch05 (tuples & arrays):** tuple types `(T, U)`, `.0` field access (compile-time, E0609),
  destructuring `let (a, b, _) = t;`, returning several values as a tuple, destructuring
  assignment `(a, b) = (b, a + b);`, one-element tuples `(x,)`, `()` as the empty tuple, tuple
  comparison (lexicographic). Arrays `[T; N]` (length is part of the type), `[v; N]`,
  indexing with `usize` only (E0277), runtime bounds checks (panic) and compile-time ones
  (`unconditional_panic`), `.len()`, `let mut` arrays and element assignment (E0594),
  `for x in array` by value, array destructuring `let [r, g, b] = rgb;` (also in `for`
  patterns), nested arrays `[[T; C]; R]` (row-major), `==` on arrays and tuples, `const`
  lookup tables of arrays/tuples. Arrays and tuples of plain numbers are **copied** on
  assignment and when passed to functions (contrast: Python aliasing; moves are ch07,
  references ch08). `{:?}`/`{:#?}` Debug formatting, `dbg!` (stderr, returns its value),
  running a subset of tests by name filter, `-- --nocapture`. Under the hood: `size_of`,
  `align_of`, `offset_of!` on tuples, alignment, padding, size is a multiple of alignment,
  Rust reorders tuple fields (vs C order, `#[repr(C)]` mentioned for ch11/ch36), contiguous
  element addresses, index = start + i × size, Python list vs NumPy vs Rust array sizes,
  locals live in the stack frame, 8 MB main-thread stack and stack overflow (frame reserved
  at function entry), rust-analyzer's memory-layout hover. *Previews:*
  `.into_iter().enumerate()` in `for (i, x) in ...` (ch20), `&x` and `{:p}` to print addresses
  (ch06/ch08), `Vec` (ch14), slices `&[T]` accepting any length (ch09),
  `#[allow(clippy::needless_range_loop)]`.
