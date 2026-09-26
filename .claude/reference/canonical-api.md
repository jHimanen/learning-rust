# Canonical minipolars API

Every milestone test in every chapter uses these names. Extend this list when a chapter
introduces new public API. Never rename anything. When a later chapter supersedes an older
milestone test, that chapter explicitly tells the learner which old test file to delete.

In tutor mode, this is the spec the learner's `project/minipolars` is built against.

- **ch02:** package `minipolars` at `project/minipolars` (created with
  `cargo new project/minipolars`), a binary crate. Running it with no arguments prints exactly
  `minipolars v<version>` (the version from Cargo.toml) followed by a newline. Test:
  `milestone/m02_version.rs`. *(ch26's CLI will supersede this test.)*
- **ch04:** a **library crate** at `project/minipolars/src/lib.rs` (same package as the
  binary, so the crate is `minipolars`). Free functions at the crate root, all over plain
  float slices, all with the input assumed NaN-free:
  - `pub fn sum(values: &[f64]) -> f64`: `0.0` for empty input.
  - `pub fn mean(values: &[f64]) -> f64`: NaN for empty input.
  - `pub fn min(values: &[f64]) -> f64`: `f64::INFINITY` for empty input.
  - `pub fn max(values: &[f64]) -> f64`: `f64::NEG_INFINITY` for empty input.
  - `pub fn variance(values: &[f64]) -> f64`: the **sample** variance (divides by `n - 1`,
    like pandas/polars), NaN for fewer than 2 values.
  - `pub fn std_dev(values: &[f64]) -> f64`: `variance(values).sqrt()`, NaN for fewer than 2
    values.

  Test: `milestone/m04_stats.rs`, imported as `use minipolars::{max, mean, min, std_dev, sum,
  variance};`. When ch15 splits minipolars into modules, these must stay reachable at the
  crate root (e.g. via `pub use`). Null-aware versions (ch13) get new names or live on
  `Column`: don't change these signatures.
