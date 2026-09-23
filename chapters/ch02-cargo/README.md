# Chapter 2: Cargo & tooling

> **Goal:** get fluent with Cargo, Rust's build tool and package manager, and the tools around
> it (formatter, linter, docs, rust-analyzer). Learn the exercise loop you'll use for the rest
> of the tutorial, and create the project you'll be building: **minipolars**.
>
> **Time:** ~1 h. **Prerequisites:** chapter 1.

---

## 1. Coming from Python

In chapter 1 you called `rustc` by hand. That's fine for one file, but real projects have
many files, dependencies, tests, and debug vs release builds. In Python, those jobs are spread
over many tools. In Rust, **Cargo** does all of them:

| Python world | Rust |
|---|---|
| `pyproject.toml` | `Cargo.toml` (the **manifest**) |
| `uv.lock` / `poetry.lock` | `Cargo.lock` |
| PyPI | [crates.io](https://crates.io) |
| `uv add requests` | `cargo add rand` |
| `python main.py` | `cargo run` |
| `pytest` | `cargo test` |
| `ruff check` | `cargo clippy` |
| `ruff format` / `black` | `cargo fmt` |
| Sphinx / pdoc | `cargo doc` |
| venv / `site-packages` | nothing: dependencies are compiled into your binary |

Vocabulary:
- A **package** is a directory with a `Cargo.toml`. It's what you create with `cargo new`.
- A **crate** is one compilation unit, the thing `rustc` compiles in one go. It's either a
  **binary crate** (has a `main`, produces an executable) or a **library crate** (produces a
  `.rlib` for other crates to use, like `std` in chapter 1). A package contains one or more
  crates.
- People say "crate" loosely for packages on crates.io too ("the rand crate").

---

## 2. A throwaway project

Do this **outside** the repo, since it's just for exploring:

```sh
cd /tmp
cargo new hello-cargo
cd hello-cargo
ls -a            # .git  .gitignore  Cargo.toml  src/
```

`cargo new` created a package *and* a git repository (because `/tmp` isn't inside one).

```toml
# Cargo.toml
[package]
name = "hello-cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

- **`edition`**: Rust's opt-in language versions (2015, 2018, 2021, 2024). An edition can
  change syntax details without breaking old code, since each crate declares its own edition
  and crates of different editions work together. Everything in this tutorial uses 2024.
- `src/main.rs` contains the same hello world you typed in chapter 1. `src/main.rs` means
  "this package has a binary crate". (`src/lib.rs` would mean a library crate.)

### Build, run, check

```sh
cargo run
```
```
   Compiling hello-cargo v0.1.0 (/private/tmp/hello-cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/hello-cargo`
Hello, world!
```

`cargo run` = `cargo build` + run the result. The binary is at `target/debug/hello-cargo`, and
you can run it directly. What did Cargo actually do? Ask it:

```sh
touch src/main.rs       # pretend we changed something
cargo build -v          # -v = verbose
```

You'll see a long `Running rustc --crate-name hello_cargo --edition=2024 src/main.rs ...
-C debuginfo=2 ...` line. **Cargo is a planner that calls `rustc` for you**, with the right
flags, in the right order, for every crate in the dependency graph.

```sh
cargo check
```

`check` runs the compiler's analysis (parsing, type checking, borrow checking) but **skips
generating machine code**. It's much faster, and it's what you run constantly while writing
code (rust-analyzer runs it on every save too).

### Profiles: dev vs release

```sh
cargo build --release
ls target/            # debug/  release/
```

Remember `rustc` vs `rustc -O` from chapter 1? Cargo calls these **profiles**:

| | `dev` (default) | `release` (`--release`) |
|---|---|---|
| Optimization | none (`opt-level = 0`) | full (`opt-level = 3`) |
| Debug info | yes | no |
| Integer overflow checks | **yes** (panics) | no (wraps around; chapter 3) |
| Output | `target/debug/` | `target/release/` |
| Use it for | writing and testing code | benchmarks, shipping |

You can tweak profiles in `Cargo.toml` (`[profile.release] ...`), but the defaults are good.
**Always benchmark with `--release`.** Timing a debug build tells you very little.

`target/` is Cargo's build cache: compiled dependencies, incremental compilation data, and
your binaries. It can be deleted any time (`cargo clean`) and is git-ignored.

---

## 3. Dependencies

```sh
cargo add rand
```

This adds a line under `[dependencies]` in `Cargo.toml`, e.g. `rand = "0.10.3"`. Replace
`src/main.rs` with:

```rust
fn main() {
    let n: u32 = rand::random_range(1..=100);
    println!("Your lucky number is {n}");
}
```

(`rand::random_range` means "the function `random_range` inside the crate `rand`", and
`1..=100` is an inclusive range.) Then run:

```sh
cargo run
cargo tree       # the dependency graph
```

Things to notice:
- **Cargo downloaded source code and compiled it.** Python wheels ship precompiled; Rust crates
  are always built from source on your machine, with your compiler and flags. That's why the
  first build is slow and later builds are fast (cached in `target/`). The sources live in
  `~/.cargo/registry/`.
- `rand` has its own dependencies (`rand_core`, `getrandom`, `libc`...). `cargo tree` shows
  them all. Every one of them is compiled into your binary: there's no `site-packages` at
  runtime.
- **Version requirements are semver ranges.** `"0.10.3"` means `^0.10.3`: any version
  `>= 0.10.3` and `< 0.11.0` (for 0.x crates, the *minor* number is treated as breaking).
  `"1.2"` would mean `>= 1.2.0, < 2.0.0`.

### Cargo.lock

Open `Cargo.lock`. It records the **exact** version and checksum of every crate in the graph
that was resolved:

```sh
grep -A3 'name = "rand"' Cargo.lock
```

`Cargo.toml` says what you *accept*; `Cargo.lock` records what you *got*. As long as the
lockfile exists, the next build (on any machine) uses exactly those versions. `cargo update`
re-resolves to the newest allowed versions.

Normally you **commit `Cargo.lock`**, so builds are reproducible. This repo is an exception:
it's in `.gitignore`. Both `main` (new chapters) and your branch (your project's dependencies)
would keep modifying the one shared lockfile, and every `git pull origin main` would conflict.
For a learning repo, occasionally resolving to a newer patch version is a fine price to pay.

---

## 4. This repo is a workspace

Back in the repo, look at the root `Cargo.toml`:

```toml
[workspace]
resolver = "3"
members = ["chapters/*", "project/*"]
exclude = ["chapters/ch01-compiling"]
```

A **workspace** is a set of packages built together. They share one `target/` directory (at
the repo root) and one `Cargo.lock`. Every directory matched by the `members` globs that
contains a `Cargo.toml` is a member. `chapters/ch02-cargo/` is one.

Because there are many packages, you tell Cargo which one you mean with **`-p`** (package):

```sh
cargo build -p ch02-cargo
cargo test -p ch02-cargo
cargo run -p ch02-cargo --example demo_build_info
cargo build --workspace      # everything
```

You can run these from anywhere inside the repo.

**Predict, then run:** run the demo in both profiles. What will `debug build?` print in each?

```sh
cargo run -p ch02-cargo --example demo_build_info
cargo run -p ch02-cargo --example demo_build_info --release
```

Read `examples/demo_build_info.rs` afterwards. It uses `env!`, a macro that reads an
environment variable **at compile time**. You'll need it in a minute.

### ★ Project step: create minipolars

From the repo root:

```sh
cargo new project/minipolars
```

It's matched by `project/*`, so it's automatically a workspace member. (And because you're
already inside a git repo, Cargo doesn't create a nested one.) Run it:

```sh
cargo run -p minipolars
```

**Milestone m02:** running `minipolars` with no arguments must print exactly
`minipolars v<version>` (for example `minipolars v0.1.0`), where the version is whatever
`project/minipolars/Cargo.toml` says.

1. Install the acceptance test:
   ```sh
   mkdir -p project/minipolars/tests
   cp chapters/ch02-cargo/milestone/m02_version.rs project/minipolars/tests/
   cargo test -p minipolars      # fails: it prints "Hello, world!"
   ```
2. Make it pass by editing `project/minipolars/src/main.rs`. Hardcoding the string works...
3. ...until you bump `version` in `project/minipolars/Cargo.toml` to `0.1.1`. Now the test
   fails again, because the test reads the version from `Cargo.toml`. Fix `main.rs` so that
   `Cargo.toml` is the **single source of truth**. (Hint: look at the demo again. Cargo sets a
   `CARGO_PKG_...` variable for the version too.)

---

## 5. The exercise loop

From chapter 3 on, most exercises work like this crate. Look at its layout:

```
chapters/ch02-cargo/
├── Cargo.toml
├── src/lib.rs          ← YOUR file: functions with `todo!()` bodies
├── tests/warmup.rs     ← tests that call your functions (don't edit)
├── tests/tidy.rs
├── examples/demo_build_info.rs
└── milestone/m02_version.rs
```

Open `src/lib.rs`. `todo!()` is a macro that compiles as "any type" but **panics** (crashes
with a message) when it runs. It lets unfinished code compile, like `raise
NotImplementedError` in Python.

Run the tests:

```sh
cargo test -p ch02-cargo
```

A failing test looks like this:

```
test answer_is_42 ... FAILED

failures:

---- answer_is_42 stdout ----

thread 'answer_is_42' panicked at chapters/ch02-cargo/src/lib.rs:15:5:
not yet implemented
```

That's `todo!()` panicking at line 15 of your `lib.rs`. Once you write an implementation that
returns the wrong value, it looks like this instead:

```
thread 'answer_is_42' panicked at chapters/ch02-cargo/tests/warmup.rs:7:5:
assertion `left == right` failed
  left: 41
 right: 42
```

Now the panic is in the *test* file. `left` is what your function returned (the first
argument to `assert_eq!`), and `right` is what the test expected. Open `tests/warmup.rs` to see
the test itself. Tests are the spec: read them whenever an exercise description leaves you
unsure.

Useful variants:

```sh
cargo test -p ch02-cargo --test warmup     # only tests/warmup.rs
cargo test -p ch02-cargo add               # only tests whose name contains "add"
cargo test -p ch02-cargo --no-fail-fast    # keep going after a failing test file
```

### Writing a function body (a sneak peek at chapter 3)

```rust
pub fn answer() -> i32 {
    42
}
```

`-> i32` is the return type. The **last expression in a function body, without a semicolon,
is the return value**. No `return` needed. (`return 42;` also works, but it's unidiomatic at
the end of a function. Clippy will tell you so in exercise 3.) `pub` makes the function visible
to the tests.

---

## 6. Formatting, linting, docs

```sh
cargo fmt                    # format every package in the workspace (rustfmt)
cargo fmt -p ch02-cargo      # just one package
cargo fmt --check            # CI-style: report, don't change
cargo clippy -p ch02-cargo   # the linter: 700+ lints for bugs, style, performance
cargo doc --open             # build HTML docs for your code AND all dependencies
```

- There's one standard style (rustfmt's), so there are no style debates. Just run it.
- Clippy is like `ruff` with a PhD: it catches things like needless code, suspicious
  comparisons, slow patterns and misused APIs. Every warning has a link explaining *why*. Read
  them; they're a great way to learn idiomatic Rust.
- In the `hello-cargo` project, `cargo doc --open` builds the full documentation for `rand`
  **offline**. Every crate on crates.io gets its docs this way, published at
  [docs.rs](https://docs.rs).

---

## 7. rust-analyzer in Neovim

rust-analyzer is the language server. It runs `cargo check` (clippy, in our config) in the
background and gives your editor types, errors, completion and navigation.

1. Make sure the standard library source is installed, since rust-analyzer needs it to
   understand `std`:
   ```sh
   rustup component add rust-src
   ```
2. Open `chapters/ch02-cargo/src/lib.rs` and run `:checkhealth vim.lsp`. `rust_analyzer`
   should be attached to the buffer.
3. Try these:
   - **`K`** on `i32` or `todo`: hover docs, the same text as the official std docs.
   - **`gd`** on `todo!`: jumps into the standard library's source. Everything in `std` is
     plain Rust you can read. (`<C-o>` jumps back.)
   - **Diagnostics:** after saving, clippy's warnings on the Part 2 functions appear inline.
     `<leader>e` shows the full message.
   - **`<leader>ca`** on a warning: code actions, often including the fix clippy suggests.
   - **Inlay hints** show inferred types inline:
     `:lua vim.lsp.inlay_hint.enable(not vim.lsp.inlay_hint.is_enabled())` toggles them.
     Try it in `examples/demo_build_info.rs`. (Worth a keymap if you like them.)
   - **`<leader>=`**: format the buffer with rustfmt.

---

## Exercises

| # | Task | Command | Difficulty |
|---|---|---|---|
| 1 | Explore `hello-cargo` (sections 2–3): `cargo run`, `cargo build -v`, `--release`, add `rand`, `cargo tree`, `cargo doc --open`. Find the binary in `target/` and run it directly. | — | ●○○ |
| 2 | Implement `answer` and `add` in `src/lib.rs` until the warmup tests pass. Rename `_a`/`_b` to `a`/`b`. | `cargo test -p ch02-cargo --test warmup` | ●○○ |
| 3 | Tidy Part 2: run `cargo fmt -p ch02-cargo`, then fix every clippy warning **by hand** (read each explanation link). Rerun clippy after each fix, because fixing one warning can reveal another. Done when clippy is silent *and* `tests/tidy.rs` still passes. | `cargo clippy -p ch02-cargo` | ●○○ |
| 4 | ★ Create minipolars and make milestone m02 pass (section 4), including the version-bump check. | `cargo test -p minipolars` | ●○○ |
| 5 | Set up rust-analyzer (section 7). Use `gd` to jump into the source of `todo!` and `println!`, and write down in your notes where they live. | — | ●○○ |
| 6 ★ | In `hello-cargo`, add `[profile.dev]` with `opt-level = 3` to `Cargo.toml` and compare `cargo build` times before and after (`cargo clean` in between). Why is `dev` unoptimized by default? | — | ●●○ |

---

## Errors you'll likely meet

| Message | Cause |
|---|---|
| `error: package ID specification 'ch02_cargo' did not match any packages` | `-p` takes the *package* name from Cargo.toml, with hyphens: `ch02-cargo`. (In Rust code the crate is `ch02_cargo`, since hyphens aren't allowed in identifiers.) |
| `current package believes it's in a workspace when it's not` | You ran `cargo new` somewhere inside the repo that isn't matched by the workspace globs. Create packages only in `project/` (or outside the repo). |
| `not yet implemented` | A `todo!()` ran. That's expected until you implement it. |
| `assertion 'left == right' failed` | Your function returned `left`, and the test expected `right`. |

---

## Checkpoint

1. What's the difference between a package and a crate? Between a binary crate and a library
   crate?
2. What do `cargo check`, `cargo build` and `cargo build --release` each do, and when do you use
   which?
3. What's the difference between `rand = "0.10.3"` in Cargo.toml and the `rand` entry in
   Cargo.lock?
4. Why does this repo not commit `Cargo.lock`, and why would you normally commit it?
5. Where did Cargo put the compiled binary for `cargo run -p minipolars`, and why not inside
   `project/minipolars/`?

**Further reading:**
- The Rust Book, [ch. 1.3: Hello, Cargo!](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
  and [ch. 14: More about Cargo](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html)
- [The Cargo Book](https://doc.rust-lang.org/cargo/): the reference for everything Cargo
- [Clippy's lint list](https://rust-lang.github.io/rust-clippy/master/)

---

## Done?

- Add a row to `notes/progress.md`, and commit: `git add -A && git commit -m "ch02 done"`
- Clean up: `rm -rf /tmp/hello-cargo`
- Next: [Chapter 3: Scalars & bits](../ch03-scalars/README.md)
