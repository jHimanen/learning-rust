# learning-rust

A hands-on Rust tutorial for people who come from Python/JavaScript and want to understand
**what happens underneath**: compilation, memory, bytes, performance, `unsafe`, C and Python
interop. You learn by writing everything yourself.

Along the way you build **minipolars**, a small dataframe engine written from scratch. It
reads CSV files into typed columns, filters, sorts, groups and aggregates. It gets a CLI, then
gets parallel and fast, and finally becomes importable from Python.

- **Short chapters** of about 1 hour, each in `chapters/chNN-topic/`.
- Each chapter has an **explanation with runnable examples**, then **exercises you do by hand**.
- Early exercises are stubs (`todo!()`) plus tests to make pass. Later ones are specs where you
  write the tests too.
- See [SYLLABUS.md](SYLLABUS.md) for the full plan and which chapters are available.

## Setup check

You need the Rust toolchain (installed with [rustup](https://rustup.rs)), git, and an editor
running rust-analyzer. Check:

```sh
rustup --version       # the toolchain manager
rustc --version        # the compiler (this tutorial was written against 1.91)
cargo --version        # the build tool / package manager
cargo clippy --version # the linter
rustup component list --installed   # should include rust-analyzer, clippy, rustfmt, rust-src
```

If something's missing: `rustup update stable` and
`rustup component add rust-analyzer clippy rustfmt rust-src`.

**Editor.** Any editor with rust-analyzer works. With Neovim (0.11+) and nvim-lspconfig:

```lua
vim.lsp.config("rust_analyzer", {
    settings = { ["rust-analyzer"] = { check = { command = "clippy" } } },
})
vim.lsp.enable("rust_analyzer")
```

Open any `.rs` file inside a chapter crate and run `:checkhealth vim.lsp`. `rust_analyzer`
should be listed as attached. Chapter 2 covers what it gives you.

## How to start

`main` always contains the **unsolved** tutorial. You never work on `main` directly. You work
on your own branch:

```sh
git clone <this repo>        # or fork it first if you want to push your work somewhere
cd learning-rust
git switch -c work/<your-name>
```

Then open [chapters/ch01-compiling/README.md](chapters/ch01-compiling/README.md).

## Daily workflow

```sh
# read the chapter
nvim chapters/ch03-scalars/README.md

# run the chapter's demos
cargo run -p ch03-scalars --example demo_overflow

# work on the exercises (stubs in src/), then check yourself
cargo test -p ch03-scalars

# commit your work to YOUR branch
git add -A && git commit -m "ch03 done" && git push
```

When new chapters are released on `main`, bring them into your branch with:

```sh
git pull --no-rebase --no-edit origin main
```

This always merges cleanly: `main` only ever adds tutorial files and never touches the files
you edit (see [File ownership](#file-ownership)).

## Getting help (with Claude Code)

This repo has a [CLAUDE.md](CLAUDE.md) that puts Claude Code in **tutor mode**. Ask for help
and it answers with a hint ladder instead of a solution:

1. a guiding question
2. the concept, and where to read about it
3. pseudo-code, or the same idea applied to a *different* problem
4. the full solution, **only** if you explicitly say *"show me the solution"*

Useful phrases:
- *"hint please"* or *"I'm stuck on `set_bit`"*: starts the ladder
- *"explain this compiler error"*: walks through reading the error with you
- *"review my solution for ch03"*: feedback on correctness, idioms and performance, without
  rewriting your code
- *"write chapter N"* (or `/write-chapter N`): authors the next chapter on a branch and
  opens a PR against `main`

## Repo layout

```
Cargo.toml          workspace root: every chapter crate + your project build from here
SYLLABUS.md         the full chapter plan
CLAUDE.md           instructions for Claude Code (tutor mode, file ownership)
.claude/            Claude Code config: the write-chapter skill, an ownership-guard hook,
                    and the concept ledger / minipolars API reference
chapters/
  ch01-compiling/   README + a playground (bare rustc, no Cargo)
  chNN-topic/       one crate per chapter:
    README.md         explanation and exercise instructions
    src/              exercise stubs: YOU edit these
    tests/            tests that check your stubs
    examples/         demo_*.rs runnable demos; fixme_*.rs "make it compile" exercises
    milestone/        acceptance tests for minipolars, copied into your project
project/
  minipolars/       your capstone (you create it in ch02)
notes/              your own notes and progress log
data/               CSV datasets
```

## File ownership

Every file belongs either to the tutorial (`main`) or to you. Neither side edits the other's
files, which is what keeps `git pull origin main` conflict-free.

| Tutorial (don't edit) | Yours |
|---|---|
| `chapters/*/README.md`, `chapters/*/tests/`, `chapters/*/examples/demo_*.rs`, `chapters/*/milestone/` | `chapters/*/src/` (the stubs you complete), `chapters/*/examples/fixme_*.rs` (the broken programs you repair) |
| `README.md`, `SYLLABUS.md`, `CLAUDE.md`, `.claude/`, `Cargo.toml`, `data/`, `project/README.md` | `project/minipolars/`, `notes/`, `chapters/ch01-compiling/playground/` |

To experiment with a demo, copy it first, e.g. to `examples/my_overflow.rs`. Any `.rs` file
in `examples/` is automatically an example you can run with `--example my_overflow`.
