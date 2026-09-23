//! Milestone m02: running `minipolars` prints its name and version.
//!
//! Copy this file to `project/minipolars/tests/` and run `cargo test -p minipolars`.
//! Don't edit it: it's the spec.

use std::process::Command;

#[test]
fn prints_name_and_version() {
    // Cargo builds the binary before running integration tests, and tells the test where it
    // is through the CARGO_BIN_EXE_<name> variable.
    let output = Command::new(env!("CARGO_BIN_EXE_minipolars"))
        .output()
        .expect("failed to run the minipolars binary");
    let stdout = String::from_utf8(output.stdout).expect("stdout was not valid UTF-8");

    // CARGO_PKG_VERSION is the `version` field of project/minipolars/Cargo.toml.
    let expected = format!("minipolars v{}\n", env!("CARGO_PKG_VERSION"));
    assert_eq!(stdout, expected);
}
