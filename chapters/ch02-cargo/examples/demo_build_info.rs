// Run it in both profiles and compare:
//   cargo run -p ch02-cargo --example demo_build_info
//   cargo run -p ch02-cargo --example demo_build_info --release

fn main() {
    // `env!("NAME")` reads an environment variable *at compile time* and bakes its value into
    // the binary as a string. Cargo sets a bunch of these when it calls rustc, e.g. the
    // package name from Cargo.toml.
    println!("package name:  {}", env!("CARGO_PKG_NAME"));
    println!("manifest dir:  {}", env!("CARGO_MANIFEST_DIR"));

    // `cfg!(debug_assertions)` is `true` in the dev profile and `false` in release.
    // Debug assertions include the integer overflow checks you'll meet in chapter 3.
    println!("debug build?   {}", cfg!(debug_assertions));
}
