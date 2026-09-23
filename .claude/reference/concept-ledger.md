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
