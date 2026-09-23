# Chapter template

## Layout

```
chapters/chNN-topic/
  README.md
  Cargo.toml          # package name == directory name, edition 2024
  src/lib.rs          # stubs: `todo!()` bodies with doc comments explaining the task
  tests/<topic>.rs    # integration tests, one file per exercise group
  examples/demo_*.rs  # runnable demos for the concepts
  examples/fixme_*.rs # (optional) programs that don't compile until the learner fixes them
  milestone/mNN_*.rs  # (optional) minipolars acceptance tests; learner copies to project/minipolars/tests/
```

## Cargo.toml

```toml
[package]
name = "chNN-topic"
version = "0.1.0"
edition = "2024"
publish = false

# Only if the chapter has fixme examples. `required-features` keeps the broken files out of
# `cargo test`/`cargo clippy`. The learner runs them with:
#   cargo run -p chNN-topic --example fixme_01 --features fixme
[features]
fixme = []

[[example]]
name = "fixme_01"
required-features = ["fixme"]
```

## README (every chapter)

1. **Title, goal, time estimate (~1 h), prerequisites.**
2. **Coming from Python/JS:** the familiar equivalent, and what's different and why.
3. **Concepts:** short sections, each backed by a runnable `examples/demo_*.rs`, with the exact
   `cargo run -p ... --example ...` command.
4. **Under the hood:** the low-level view (ASCII memory diagrams, `size_of`, addresses,
   generated code, arm64 specifics).
5. **Predict, then run:** questions the learner answers *before* running a demo.
6. **Exercises:** a table of file, command, difficulty (●○○ to ●●●) and concepts, plus notes
   per exercise. Stretch exercises are marked ★.
7. **Project step (minipolars):** the milestone spec (★ chapters only). Early chapters include
   exact signatures plus milestone tests. From ch22, only the public surface. From ch26, CLI
   acceptance tests.
8. **Compiler errors you'll likely meet:** error codes and how to read them.
9. **nvim tip** (where relevant).
10. **Checkpoint:** 3–5 self-quiz questions, and further reading (The Book, Rust by Example, std
    docs, and later the Rustonomicon).
11. **Done?** Remind them to log it in `notes/progress.md` and commit.

Style: explain like a senior engineer pairing with a smart newcomer. Concrete, with small
examples, and no fluff. Keep the chapter to about 1 hour, and cut or mark extras as ★ rather
than bloating it.

## Exercise design

- Stubs in `src/`: `pub fn` with a doc comment that states the task precisely, and a body of
  `todo!()`. Parameters that are unused until implemented get a `_` prefix, so the stubs
  compile without warnings.
- Tests in `tests/` call the public stubs via `use chNN_topic::*;`. Cover the normal case,
  edge cases and the "gotcha" that teaches the concept (overflow, empty input, UTF-8 and so on).
- Tests fail at `todo!()` before implementation, and must never pass by accident.
- Fixme examples: a small program with one or two deliberate errors that teach the chapter's
  concept, and a comment at the top explaining what "fixed" means (compiles, and prints X).
