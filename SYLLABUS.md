# Syllabus

About 38 chapters of roughly one hour each. ★ marks a **minipolars** milestone, a step in the
capstone project. **Available** chapters exist in `chapters/`. The rest are written one at
a time as the tutorial progresses.

The arc: first, what a compiled program even is. Then how Rust manages memory without a
garbage collector (the core of the language). Then modeling data and abstraction. Then
growing the dataframe engine, making it fast, and finally stepping outside safe Rust (unsafe,
C, Python).

## Part 0: Toolchain & first contact

| # | Chapter | Status |
|---|---|---|
| 1 | **What compiling means.** `rustc` by hand, inspecting the binary (`file`, `otool`, `nm`), debug vs optimized, Python's interpreter vs a native executable, rustup and toolchains. *Getting started with your own branch.* | available |
| 2 | **Cargo & tooling.** `cargo new`, `Cargo.toml`/`Cargo.lock`, `target/`, check/build/run/test, profiles, dependencies, workspaces, clippy, rustfmt, docs, rust-analyzer in nvim, how the exercises work. ★ create `minipolars` | available |
| 3 | **Scalars & bits.** Integer widths, two's complement, overflow, casting, floats (IEEE 754), `char`, bitwise operations, formatting. | available |
| 4 | **Functions & control flow.** Expressions vs statements, `if` as a value, `loop`/`while`/`for`, ranges, labeled breaks. ★ first statistics functions | available |
| 5 | **Tuples, arrays & a first look at memory.** Contiguous layout, `size_of`, stack-allocated arrays. | |

## Part 1: Memory & ownership (the heart of Rust)

| # | Chapter | Status |
|---|---|---|
| 6 | **Process memory.** Stack frames vs the heap, `Box<T>`, printing addresses. | |
| 7 | **Ownership & moves.** `String` as (ptr, len, cap), Copy vs move, `Drop`; compared with Python's refcounting and GC. | |
| 8 | **Borrowing.** `&T`/`&mut T`, aliasing XOR mutation, and the bugs it prevents. | |
| 9 | **Slices & strings.** `&str` vs `String`, `&[T]`, UTF-8 bytes vs chars. ★ read a CSV file, print header and row count | |
| 10 | **Lifetimes, an intro.** Dangling references, annotations, elision. | |

## Part 2: Modeling data

| # | Chapter | Status |
|---|---|---|
| 11 | **Structs & methods.** `impl`, `self`, constructors, layout, padding and alignment. | |
| 12 | **Enums & pattern matching.** `match`, `if let`, `let else`, enum layout, niche optimization. ★ `Value` and `Column` types | |
| 13 | **Option, Result and `?`.** Errors as values vs exceptions. ★ column type inference, nulls | |
| 14 | **Collections.** `Vec` growth and capacity, `HashMap`, `HashSet`, how hashing works. ★ `DataFrame`, `select`, `head` | |
| 15 | **Modules, crates, packages.** Library + binary, visibility. ★ split minipolars into modules | |

## Part 3: Abstraction

| # | Chapter | Status |
|---|---|---|
| 16 | **Traits.** `Display`/`Debug`, derive, trait bounds. ★ pretty-printed tables | |
| 17 | **Generics & monomorphization.** Seeing the generated copies in the binary. ★ generic aggregations | |
| 18 | **Trait objects.** `dyn`, vtables, fat pointers, static vs dynamic dispatch. ★ pluggable aggregations | |
| 19 | **Closures.** `Fn`/`FnMut`/`FnOnce`, captures, closures as structs. ★ `map`, predicates | |
| 20 | **Iterators.** Laziness, adaptors, writing your own, zero-cost abstractions. ★ mask filtering | |
| 21 | **Error handling for real.** Custom error types, `From`, `?`, `Box<dyn Error>`. ★ precise parse errors | |
| 22 | **Testing properly.** Unit, integration and doc tests. *From here on you write your own tests.* ★ parser test suite | |

## Part 4: The engine grows (spec-driven)

| # | Chapter | Status |
|---|---|---|
| 23 | **A byte-level CSV state machine.** Quotes, escapes, embedded newlines. ★ | |
| 24 | **Group-by.** `Hash`/`Eq`, why `f64` isn't `Hash`, row-index maps. ★ `group_by().agg(...)` | |
| 25 | **Sorting.** `Ord`/`PartialOrd`, `total_cmp`, argsort. ★ `sort_by` | |
| 26 | **CLI & packaging.** clap, features, semver, `cargo install`. ★ `minipolars head / describe / groupby` | |
| 27 | **Smart pointers.** `Rc`, `RefCell`, `Weak`, `Arc`. ★ cheap column sharing | |

## Part 5: Performance & the metal

| # | Chapter | Status |
|---|---|---|
| 28 | **Benchmarking & profiling.** A data generator, criterion, flamegraphs. ★ allocation-free parsing | |
| 29 | **Memory layout for speed.** Cache lines, AoS vs SoA, validity bitmaps. ★ null bitmap | |
| 30 | **Reading assembly.** ARM64 basics, bounds checks, auto-vectorization. | |
| 31 | **Threads.** `spawn`, `Send`/`Sync`, channels, scoped threads, `Arc<Mutex<T>>`. ★ parallel parsing | |
| 32 | **Data parallelism.** rayon, vs Python's GIL. ★ parallel aggregation | |
| 33 | **Async: what and why** (bonus). | |

## Part 6: Unsafe & interop

| # | Chapter | Status |
|---|---|---|
| 34 | **unsafe & raw pointers.** Undefined behavior, Miri. Build your own `Vec`, part 1. | |
| 35 | **Your own `Vec`, part 2.** `Drop`, `Deref`, iteration, checked by Miri. | |
| 36 | **FFI with C.** `extern "C"`, `#[repr(C)]`, `build.rs`, linking. | |
| 37 | **Python bindings.** PyO3 + maturin. ★ minipolars from Python, benchmarked vs pandas/polars | |
| 38 | **Wrap-up & extension menu.** Lazy query plans, mmap, Arrow, SIMD, publishing. | |
