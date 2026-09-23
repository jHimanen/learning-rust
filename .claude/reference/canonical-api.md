# Canonical minipolars API

Every milestone test in every chapter uses these names. Extend this list when a chapter
introduces new public API. Never rename anything. When a later chapter supersedes an older
milestone test, that chapter explicitly tells the learner which old test file to delete.

In tutor mode, this is the spec the learner's `project/minipolars` is built against.

- **ch02:** package `minipolars` at `project/minipolars` (created with
  `cargo new project/minipolars`), a binary crate. Running it with no arguments prints exactly
  `minipolars v<version>` (the version from Cargo.toml) followed by a newline. Test:
  `milestone/m02_version.rs`. *(ch26's CLI will supersede this test.)*
