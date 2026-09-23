# notes/

This directory is **yours**. Apart from this README, `main` never puts anything here. It's the
place to write things down in your own words, which is where most of the learning sticks.

## Suggested files

**`progress.md`**: a log of where you are. Create it in chapter 1. For example:

```markdown
# Progress

| Ch | Done | Date | Time | Notes |
|---|---|---|---|---|
| 01 | ✅ | 2026-09-24 | 50 min | otool is neat; the release binary computed the sum at compile time?! |
| 02 | ✅ | 2026-09-25 | 70 min | |
| 03 | 🚧 | | | stuck on two's complement for a while |
```

**`chNN.md`** (optional): a short journal per chapter. A template that works well:

```markdown
# ch03: Scalars & bits

## In my own words
- ...

## Python → Rust
| Python | Rust | Why different |
|---|---|---|
| `int` (unbounded) | `u8`…`u128`, `i8`…`i128` | fixed-size: maps to CPU registers, no heap allocation |

## Surprised me
- ...

## Still fuzzy (ask about it / revisit)
- ...
```

When you ask Claude Code for help, it may read these notes to see what you've found hard, and
the tutorial's explanations get improved based on that.
