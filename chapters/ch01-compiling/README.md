# Chapter 1: What compiling means

> **Goal:** understand what actually happens between writing `hello.rs` and running a
> program. You'll build a native executable by hand with the bare compiler (no Cargo yet), look
> inside it, break it on purpose, and race it against Python.
>
> **Time:** ~1 h. **Prerequisites:** the setup check in the [top-level README](../../README.md).

---

## 0. Getting started (5 min)

Everything you do in this tutorial lives on **your own branch**. `main` stays pristine, so
anyone can start from scratch.

```sh
cd learning-rust
git switch -c work/<your-name>          # e.g. work/joel
git push -u origin work/<your-name>     # optional: back it up on GitHub
```

Create your progress log, `notes/progress.md`. There's a suggested format in
[notes/README.md](../../notes/README.md). Commit it:

```sh
git add notes/progress.md && git commit -m "Start the tutorial"
```

When new chapters appear on `main` later, `git pull --no-rebase --no-edit origin main` brings them in.

---

## 1. Coming from Python: two ways to run code

When you run `python3 sum.py`, no machine code is produced from *your* file. Instead:

```
sum.py ──(parsed + compiled at startup)──> bytecode ──> the CPython VM interprets it,
                                                        one instruction at a time
```

The CPython VM is itself a compiled program (written in C) that *simulates* your program. Each
`total += i` goes through a loop inside CPython: fetch the next bytecode instruction, look up
what it means, look up the types of `total` and `i`, find the right `+` implementation,
allocate a new integer object, update the name, and repeat. Everything is dynamic, and it costs
a lot per operation.

Rust does all the translation **ahead of time**:

```
hello.rs ──rustc──> [ parse → type check → borrow check → optimize ] ──> machine code (hello.o)
                                                                               │
                                     the Rust standard library (precompiled) ──┤ linker (cc/ld)
                                                                               ▼
                                                                         hello  (an executable)
```

The result is a file containing instructions for *your CPU* (ARM64 on Apple Silicon). When you
run it, the OS loads it into memory and the CPU executes it directly. There's no interpreter
and no VM. All the type checks happened at compile time, so they cost nothing at runtime.

The trade-off: you have to compile before running, and the compiler rejects programs it can't
prove correct. In exchange you get speed, a single self-contained binary, and whole classes of
bugs caught before the program ever runs.

---

## 2. Your first program, by hand

Make a playground (it's yours, and the build outputs in it are git-ignored):

```sh
mkdir -p chapters/ch01-compiling/playground
cd chapters/ch01-compiling/playground
```

Create `hello.rs` in nvim. **Type it; don't paste it.** Your fingers learn the syntax.

```rust
fn main() {
    println!("Hello, world!");
}
```

- `fn main()` is the entry point. When the OS starts your program, the Rust runtime does a
  little setup and then calls `main`, much like `if __name__ == "__main__":` but mandatory.
- `println!` has a `!` because it's a **macro**, not a function. Macros generate code at
  compile time. `println!` checks your format string *while compiling*, so a mistake like
  `println!("{}")` with no argument is a compile error, not a runtime crash.
- Statements end with `;`. Blocks use `{ }`, not indentation.

Compile and run it:

```sh
rustc hello.rs
./hello
```

`rustc hello.rs` produced a file called `hello` in the current directory. That file *is* your
program. You can copy it to another Mac (with the same CPU architecture) and run it there
without Rust installed. Try `./hello; echo "exit code: $?"`: an exit code of `0` means success
to the shell.

---

## 3. What did you just make?

Let's look inside. Run each of these and read the output:

```sh
file hello
```
```
hello: Mach-O 64-bit executable arm64
```
**Mach-O** is macOS's executable format (Linux uses ELF, Windows uses PE). **arm64** is the
instruction set: this file contains ARM64 machine code and won't run on an Intel CPU as-is.

```sh
ls -lh hello
```
It's roughly **430 KB** for a program that prints one line. Why so big? Rust **statically
links** its standard library: the code for printing, formatting, panicking and so on is copied
into your binary. The precompiled standard library lives in your toolchain:

```sh
ls ~/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib | grep -E "^lib(std|core|alloc)-"
```

`.rlib` files are precompiled Rust libraries: `core` (the basics), `alloc` (heap allocation)
and `std` (the OS-facing layer on top).

```sh
otool -L hello
```
```
hello:
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1356.0.0)
```
This lists the **dynamic** libraries the binary needs at runtime. There's only one:
`libSystem`, macOS's C library, which is how *any* program talks to the operating system (to
write to the terminal, allocate memory, open files). Compare this with Python, where your
`.py` file needs a whole installed interpreter and its libraries.

### Under the hood: compiling vs linking

`rustc` did two separate jobs. You can stop it after the first:

```sh
rustc --emit=obj hello.rs      # produces hello.o: machine code for YOUR code only
file hello.o                   # Mach-O 64-bit object arm64
ls -lh hello.o                 # a few KB
nm -C hello.o | grep " U "     # U = "Undefined": symbols this file uses but doesn't contain
```

You'll see `std::io::stdio::_print::h…` listed as **U**. Your code calls the standard
library's print function, but that function's code isn't in `hello.o`. The **linker** (on macOS
`rustc` calls the system linker `cc`/`ld`, which came with the Xcode Command Line Tools) glues
`hello.o` together with the code it needs from the `std` `.rlib` and produces the executable.
Check that the symbol is now defined (**T** = "Text", meaning code):

```sh
nm -C hello | grep "std::io::stdio::_print"
nm -C hello | grep "hello::main"
```

`hello::main::h6acbbac5…` is your `main` function. The `h…` suffix is a hash that keeps
symbol names unique; yours will differ. Without `-C`, `nm` shows the raw **mangled** name
(`__ZN5hello4main17h…E`), which is how Rust encodes paths like `hello::main` into a flat
symbol name the linker understands.

---

## 4. Compile errors are your friend

In Python, a type mix-up is found when that line *runs* (maybe in production). In Rust it's
found before anything runs. Create `oops.rs`:

```rust
fn main() {
    let answer: i32 = "forty-two";
    println!("{answer}");
}
```

`let answer: i32 = …` declares a variable with a type annotation, just like
`answer: int = ...` in Python with mypy, except that here it's enforced. `i32` is a 32-bit
signed integer (chapter 3 covers this). `{answer}` inside the format string prints the
variable.

```sh
rustc oops.rs
```
```
error[E0308]: mismatched types
 --> oops.rs:2:23
  |
2 |     let answer: i32 = "forty-two";
  |                 ---   ^^^^^^^^^^^ expected `i32`, found `&str`
  |                 |
  |                 expected due to this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

How to read a Rust error, top to bottom:
1. **`error[E0308]: mismatched types`**: the error code and a one-line summary.
2. **`--> oops.rs:2:23`**: the file, line and column. Jump straight there.
3. **The annotated snippet**: `^^^` marks the problem and `---` marks the related context
   ("expected due to this").
4. Often a **`help:`** line suggests a fix. Read it, but make sure you understand *why* before
   applying it.
5. **`rustc --explain E0308`** gives a long explanation with examples, offline. Try it now.

No `oops` binary was produced (check with `ls`). Rust won't produce a program it can't fully
type-check.

---

## 5. Debug vs optimized, and a race against Python

Create `sum.py`:

```python
total = 0
for i in range(100_000_000):
    total += i
print(total)
```

and `sum.rs`:

```rust
fn main() {
    let mut total: u64 = 0;
    for i in 0..100_000_000 {
        total += i;
    }
    println!("{total}");
}
```

A few new things (covered properly later):
- `let mut`: variables are **immutable by default**, and `mut` allows reassignment. Chapter 3.
- `u64`: an unsigned 64-bit integer (0 to about 1.8 × 10¹⁹). The sum is too big for 32 bits.
- `for i in 0..100_000_000`: a range from 0 up to *but not including* 100,000,000, like
  `range(100_000_000)`. Chapter 4.

Build it twice, once without and once with optimizations:

```sh
rustc sum.rs -o sum_debug          # default: fast to compile, slow code, extra safety checks
rustc -O sum.rs -o sum_release     # -O: optimize (slower to compile, much faster code)
```

**Predict first:** write down how long each of these will take, then run them:

```sh
time python3 sum.py
time ./sum_debug
time ./sum_release
```

On an M-series Mac you'll see something like: Python about **5.5 s**, debug about **0.5 s**,
release about **0.00 s**. That's not a typo. How can the release build do 100 million
additions in no measurable time? It doesn't do them at all. Let's look at the machine code.

### Under the hood: reading the assembly

```sh
rustc -O --emit=asm sum.rs -o sum.s
nvim sum.s
```

Search for `4main` (`/4main` in nvim) to find your `main` function. Inside it you'll find:

```asm
	mov	x8, #36736
	movk	x8, #13541, lsl #16
	movk	x8, #50041, lsl #32
	movk	x8, #17, lsl #48
```

These four instructions build one 64-bit number in register `x8`, 16 bits at a time (`movk` =
"move and keep the other bits", `lsl #16` = "shifted left by 16"). Check which number:

```sh
python3 -c "print(17 * 2**48 + 50041 * 2**32 + 13541 * 2**16 + 36736)"
```

It's **4999999950000000**, your answer. The optimizer (LLVM) recognized the loop as the sum of
an arithmetic series, computed the result **at compile time**, and deleted the loop. The
release binary just prints a constant.

This is the core idea behind Rust performance: the compiler knows every type and every value's
lifetime, so it can transform your code aggressively. Python can't do this, because at runtime
`total` could be anything and `+` could be overloaded.

Now compare with what Python actually executes:

```sh
python3 -m dis sum.py
```

Each line (`LOAD_NAME`, `BINARY_OP`, `STORE_NAME`, `JUMP_BACKWARD`…) is one bytecode
instruction the CPython VM interprets, **100 million times**. Each of those steps is dozens to
hundreds of real CPU instructions.

(Why is the *debug* build still 10× faster than Python even without optimization? It still
runs native machine code with fixed-size integers in CPU registers, with no object allocation
and no dynamic type dispatch. And why is it slower than release? Debug builds keep every
variable in memory for the debugger and check every `+` for overflow. Chapter 3 shows that
overflow check in action.)

---

## 6. The toolchain: what's rustup?

```sh
which rustc               # ~/.cargo/bin/rustc
ls -l ~/.cargo/bin        # cargo, rustc, … all symlinks → rustup
rustup which rustc        # the REAL compiler: ~/.rustup/toolchains/stable-…/bin/rustc
rustc -vV                 # version details, including host: aarch64-apple-darwin
rustup show               # installed toolchains and targets
```

- **rustup** is the toolchain manager, a bit like `pyenv`. The `rustc` and `cargo` in your PATH
  are small **proxies** that ask rustup which real toolchain to run.
- A **toolchain** is a channel plus a host: `stable-aarch64-apple-darwin`. The channels are
  `stable` (every 6 weeks), `beta`, and `nightly` (daily, with experimental features; used in
  chapter 34 for Miri).
- **Components** are optional parts of a toolchain: `clippy`, `rustfmt`, `rust-analyzer`,
  `rust-src` (std source code, used by rust-analyzer), `rust-docs`.
- A **target** is a platform to compile *for*: the "target triple" `aarch64-apple-darwin` means
  architecture, vendor and OS. You can add targets to cross-compile, e.g. for Linux.
- `rustup update` upgrades everything. `rustup doc --std` opens the standard library docs
  **offline** in your browser.

---

## Exercises

All exercises happen in `chapters/ch01-compiling/playground/`. There are no tests this time:
you check your results by looking.

| # | Task | Difficulty |
|---|---|---|
| 1 | Type, compile and run `hello.rs` (section 2). Run it from another directory with a relative path, e.g. `cd .. && ./playground/hello`. | ●○○ |
| 2 | Run `file`, `ls -lh`, `otool -L` and `nm -C … \| grep main` on it. In `notes/ch01.md`, explain in your own words why the binary is ~430 KB and why only `libSystem` shows up in `otool -L`. | ●○○ |
| 3 | Extend `hello.rs`: store your name in a variable (`let name = "…";`) and print `Hello, <name>!` using `{name}` in the format string. Then print a second line. Recompile and run. | ●○○ |
| 4 | **Break it five ways.** Make each of these changes to a copy of hello.rs (`cp hello.rs broken.rs`), compile, and read the full error. For each, note the error code (if any) and what the compiler suggested: (a) misspell `println` as `printn`; (b) delete the closing `}`; (c) remove the `!` from `println!`; (d) use `{nme}` instead of `{name}` in the format string; (e) declare `let unused = 5;` and never use it. Which one is only a *warning*, and does a binary still get produced? | ●●○ |
| 5 | Do the race in section 5. **Predict before running.** Record the three timings in your notes. | ●○○ |
| 6 | Find the four `mov`/`movk` instructions in `sum.s` (section 5) and verify the constant with the Python one-liner. Then generate the *debug* assembly (`rustc --emit=asm sum.rs -o sum_debug.s`) and compare the line counts with `wc -l sum.s sum_debug.s`. Can you find the loop in the debug version? (Hint: look for a branch instruction like `b.` or `b` that jumps *backwards* to an earlier label.) | ●●○ |
| 7 | Do the compile-vs-link experiment in section 3: `--emit=obj`, then `nm -C hello.o \| grep " U "`. Which symbols are undefined, and where do they come from? | ●●○ |
| 8 ★ | Change `sum.rs` so the loop runs up to 1,000,000,000 and time the debug and release builds again. Then run `python3 sum.py` with 10× the iterations only if you have a minute to spare. What does the ratio tell you? | ●○○ |

---

## Compiler errors you'll likely meet

| Code | Meaning | Typical cause in this chapter |
|---|---|---|
| E0308 | mismatched types | Assigning a string to an `i32` variable |
| E0425 | cannot find value in this scope | A typo in a variable name, e.g. `{nme}` |
| E0423 | expected function, found macro `println` | A missing `!`. The help line says "use `!` to invoke the macro" |
| (no code) | cannot find macro `printn` | A typo in a macro name. The help suggests a similar name (here `print`, which also exists, but isn't the one you meant) |
| (no code) | this file contains an unclosed delimiter | A missing `}` or `)` |
| warning: unused variable | Not an error: the binary is still produced | Prefix the name with `_` to silence it on purpose |

---

## Checkpoint

1. What's the difference between what `python3 sum.py` executes and what `./sum_release`
   executes?
2. Why does `hello.o` have undefined symbols while `hello` doesn't? What tool resolved them?
3. What does `-O` change, and why was the optimized loop "free"?
4. What are `rustup`, a toolchain, a component and a target?
5. Why is it useful that `println!` is a macro rather than a function?

**Further reading:**
- The Rust Book, [ch. 1: Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [Overview of the compiler](https://rustc-dev-guide.rust-lang.org/overview.html) in the rustc
  dev guide (skim it: parse → HIR → MIR → LLVM IR → machine code)
- [Compiler Explorer](https://godbolt.org/): paste `sum.rs`, pick "rustc" and add `-O`, and see
  the assembly live

---

## Done?

- Add a row to `notes/progress.md`.
- Commit your playground and notes to your branch:
  `git add -A && git commit -m "ch01 done" && git push`
- Next: [Chapter 2: Cargo & tooling](../ch02-cargo/README.md)
