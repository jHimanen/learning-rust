# Chapter 4: Functions & control flow

> **Goal:** write real logic: branches, loops and early exits. Along the way you'll see
> Rust's central syntactic idea, **almost everything is an expression that produces a value**,
> and what `if` and loops turn into on your CPU. Then you'll write minipolars' first
> statistics functions.
>
> **Time:** ~1 h. **Prerequisites:** chapter 3.

---

## 1. Coming from Python

The building blocks are the ones you know, with different spelling and a few sharp edges:

| Python | Rust | Note |
|---|---|---|
| `x if cond else y` | `if cond { x } else { y }` | No separate ternary: `if` itself produces a value. |
| `if n:` | `if n != 0 {` | No truthiness. A condition must be a `bool`. |
| `while True: ... break` | `loop { ... break; }` | `loop` can return a value: `break value`. |
| `while cond:` | `while cond {` | |
| `for i in range(5):` | `for i in 0..5 {` | `a..b` excludes `b`, like `range`. |
| `for i in range(1, n + 1):` | `for i in 1..=n {` | `..=` includes the end. |
| `for i in range(n - 1, -1, -1):` | `for i in (0..n).rev() {` | |
| `for i in range(0, n, 2):` | `for i in (0..n).step_by(2) {` | |
| `for ... else:` | a labeled block (section 5) | |
| `def f(x: int) -> int:` | `fn f(x: i64) -> i64 {` | Types are required, not hints. |
| implicit `return None` | returns `()` | |

There's no C-style `for (i = 0; i < n; i++)`, no `++` or `--` (use `+= 1`), and no
`do ... while`. You won't miss them.

From JavaScript, the big difference is the idea of a statement vs an expression. In JS,
`if` is a statement and you can't write `const x = if (...) {...}`. In Rust you can, and
you'll do it all the time.

---

## 2. Expressions, statements and blocks

```sh
cargo run -p ch04-control-flow --example demo_expressions
```

- An **expression** produces a value: `5`, `a + b`, `f(x)`, `if c { 1 } else { 2 }`,
  and also **blocks**, `{ ... }`.
- A **statement** does something and produces nothing. The two kinds you'll write are `let`
  bindings, and an expression followed by `;` (evaluate it and throw the value away).
- A block's value is **its last expression** (the one without a semicolon). A function body
  is just a block, and that's why the last expression of a function is its return value.

```rust
let area = {
    let width = 3;      // statement
    let height = 4;     // statement
    width * height      // expression: the block's value
};                      // area == 12; width and height no longer exist
```

If a block ends in a statement, its value is **`()`**, pronounced "unit": a type with
exactly one value, which carries no information. It's Rust's `None`-that-isn't-an-`Option`,
what a function returns when it has nothing to return. Assignment (`x = 5`) is also an
expression of type `()`, so `let y = x = 5;` compiles and makes `y` a `()`.

### Functions

```rust
fn describe(n: i32) -> &'static str {    // (&'static str: a string literal; chapter 10)
    if n == 0 {
        return "zero";                    // early exit
    }
    if n % 2 == 0 { "even" } else { "odd" }  // the normal return value
}
```

- `return` exits early from anywhere in the function. At the end, the last expression does
  the job, and clippy flags a `return x;` there as unidiomatic (you saw that in chapter 2).
- No `-> Type` means the function returns `()`.
- **Parameters are like `let` bindings**, so they're immutable by default. Write
  `fn f(mut n: u64)` to modify `n` in the body. The caller doesn't notice: `n` is the
  function's own copy of the number.
- Functions can call each other in any order (no need to define before use) and call
  themselves (recursion). Every call gets its own space on the stack (chapter 6), so very
  deep recursion crashes with a stack overflow. Rust doesn't optimize tail calls reliably,
  so a loop is the idiomatic tool for repetition.

---

## 3. `if` as a value

```rust
let feel = if celsius >= 25 {
    "hot"
} else if celsius >= 15 {
    "warm"
} else {
    "cold"
};                              // note the `;`: the whole `if` is the right side of a `let`
```

Two rules follow from "`if` is an expression":

1. **Every branch must have the same type.** The compiler has to know the type of `feel`
   without knowing which branch runs.
2. **An `if` used as a value needs an `else`.** Without one, what would `feel` be when the
   condition is false? An `if` without `else` has type `()`, which is fine when it's used as
   a statement (`if x > max { max = x; }`).

And conditions are `bool`, always: `if count { }` is error E0308, "expected `bool`, found
integer". Write the comparison you mean: `if count != 0 { }`.

**Try it:** `cargo run -p ch04-control-flow --example fixme_01 --features fixme` breaks both
rules. Read the errors, which point exactly at the problem, and fix them.

---

## 4. Loops and ranges

```sh
cargo run -p ch04-control-flow --example demo_loops
```

Rust has three loops:

```rust
// loop: repeat forever, until a `break`. `break value` makes the loop an expression.
let first_big = loop {
    x *= 3;
    if x > 1_000 {
        break x;
    }
};

// while: repeat as long as a condition holds.
while n > 1 {
    n /= 2;
}

// for: once per item of something iterable. For now: ranges.
for i in 0..5 {
    println!("{i}");
}
```

Use `for` when you know what to iterate over, `while` when you have a condition, and `loop`
when the exit is somewhere in the middle of the body, or when you want a value out of it.
Inside any of them, `continue` skips to the next iteration and `break` leaves the loop.

**Only `loop` can `break` with a value.** A `for` or `while` loop can also end on its own
(the range runs out, or the condition becomes false). There's no value in that case, so
these loops always have type `()`.

### Ranges

| Range | Items | Python |
|---|---|---|
| `0..5` | 0, 1, 2, 3, 4 | `range(5)` |
| `1..=5` | 1, 2, 3, 4, 5 | `range(1, 6)` |
| `(1..=5).rev()` | 5, 4, 3, 2, 1 | `range(5, 0, -1)` |
| `(0..10).step_by(3)` | 0, 3, 6, 9 | `range(0, 10, 3)` |
| `1..` | 1, 2, 3, ... forever | `itertools.count(1)` |
| `5..5`, `9..3` | nothing (empty) | `range(5, 5)`, `range(9, 3)` |

- A range is an ordinary value (`let r = 0..n;`), not special loop syntax.
- Its type comes from its ends: in `for i in 0..n` with `n: u64`, `i` is a `u64`. With
  inlay hints on, you see it next to `i`.
- An empty range is fine: the body just never runs. That's often exactly the edge-case
  behavior you want (what's `1 × 2 × ... × n` for `n = 0`?).
- `9..3` doesn't count down: it's empty. Use `.rev()`. Clippy flags literal empty ranges like
  `9..3` as an error, because it's almost always a mistake.
- `.rev()` and `.step_by()` are methods. Chapter 20 (iterators) shows how they work, and what
  `for` really does.

**Try it:** `cargo run -p ch04-control-flow --example fixme_02 --features fixme`. The compiler
reports three errors, but one of them is only a consequence of another.

### Clippy and `%`

A heads-up for the exercises: `n % d == 0` asks "is `n` divisible by `d`?". For unsigned
integers, recent clippy versions (including this repo's toolchain) flag that and suggest **`n.is_multiple_of(d)`**. Take the
suggestion: it says what you mean, and it doesn't panic when `d` is 0. (Signed integers don't
have this method, so there `n % d == 0` is still the way.)

---

## 5. Nested loops and labels

A `break` or `continue` applies to the innermost loop. To target an outer loop, give it a
**label**, a name starting with `'`:

```rust
'rows: for i in 1..=12 {
    for j in 1..=12 {
        if i * j > 12 {
            continue 'rows;          // skip the remaining j's: next i
        }
        // ...
    }
}
```

`break 'rows;` would leave both loops at once. In Python you'd need a flag variable, or put
the loops in a function and `return`.

**Labeled blocks** are the other half. Any block can have a label, and `break 'label value`
leaves it with a value. That makes search-with-a-default one expression, like Python's
`for ... else`:

```rust
let smallest_factor = 'search: {
    for d in 2..n {
        if n % d == 0 {
            break 'search d;         // found: this is the block's value
        }
    }
    n                                // not found: the block's value is n itself
};
```

Inside a function, `return` often does the same job. A labeled block is for when you want
the value *inside* a function, without leaving it.

---

## 6. Under the hood

### Nothing, and never

`()` has one possible value, so storing it takes **zero bytes**: `size_of::<()>()` is 0 (the
end of `demo_expressions` prints it). The compiler doesn't generate any code for it. It's
purely a type-level "nothing here".

There's also a type with **no** values: `!`, called *never*. It's the type of expressions
that never finish: `panic!(...)`, `return`, `break`, `continue`, a `loop` with no `break`, and
**`todo!()`**. An expression that can't produce a value can pretend to be any type, so `!`
fits wherever a type is expected:

```rust
let limit: u32 = if ok { 100 } else { panic!("bad config") };  // u32 and !: fine
```

That's how every exercise stub compiles. `pub fn gcd(...) -> u64 { todo!() }` is accepted
because `todo!()` has type `!`, which satisfies `u64`, and at runtime it panics before a
`u64` is ever needed.

### What `if` and loops compile to

Generate the assembly for `examples/demo_asm.rs` (two small functions, `pick_larger` and
`halvings`), optimized and unoptimized, and open it:

```sh
rustc -O --emit=asm chapters/ch04-control-flow/examples/demo_asm.rs -o target/demo_asm.s
rustc --emit=asm chapters/ch04-control-flow/examples/demo_asm.rs -o target/demo_asm_debug.s
nvim target/demo_asm.s
cargo run -p ch04-control-flow --example demo_asm --release   # just runs it
```

Search for `/11pick_larger` (the name is mangled, as in chapter 1's `nm` output). The
optimized version is this:

```asm
	cmp	x0, x1              ; compare a (in register x0) with b (in x1)
	csel	x0, x0, x1, gt      ; x0 = if "greater than" { x0 } else { x1 }
	ret                         ; return: the result is whatever is in x0
```

Three things to notice:

- **Arguments arrive in registers `x0`, `x1`, ... and the return value leaves in `x0`.**
  That's the ARM64 calling convention: the agreement that lets any function call any other.
  No memory is touched at all.
- **`csel` (conditional select) *is* your `if` expression.** Both values already exist, and
  the CPU just picks one. There's no jump. That matters because a modern CPU works many
  instructions ahead, *guessing* which way each branch goes. A wrong guess throws that work
  away (on the order of ten cycles). `csel` leaves nothing to guess.
- `gt` means "greater than, **signed**": `a` and `b` are `i64`. For unsigned numbers you'll
  see `hi` (higher) and `lo` (lower) instead. Same bits, different comparison (chapter 3).

Now look at the same function in `target/demo_asm_debug.s`: 15 instructions, with
`b.gt` and `b` (branch) jumps, and every value stored to and loaded from memory (`str`,
`ldr`, relative to `sp`, the stack pointer). The debug build translates your code literally
so that a debugger can follow it. The optimizer noticed that both branches are "pick a
value" and replaced the jumps with `csel`.

Back in the optimized file, search for `/8halvings`. Its loop:

```asm
	cmp	x0, #2              ; n < 2?
	b.lo	LBB6_4              ; then skip the loop (the label number may differ for you)
	mov	w8, #0              ; steps = 0
LBB6_2:                             ; ← top of the loop
	add	w8, w8, #1          ; steps += 1
	cmp	x0, #3              ; compare n with 3 (before halving)
	lsr	x0, x0, #1          ; n = n >> 1         ← `n /= 2` became a shift (chapter 3)
	b.hi	LBB6_2              ; if n was > 3 (now > 1): jump BACK to the top
	mov	x0, x8              ; return steps
	ret
```

**A loop is a jump backwards.** There's no loop instruction, just a conditional branch to an
earlier label. `while`, `for` and `loop` all compile down to this. Two more details:

- `w8` is the low 32 bits of register `x8`. `steps` is a `u32`, so it lives in a `w`
  register, while `n` (a `u64`) uses the full `x0`.
- The optimizer reordered the test: "is `n / 2 > 1`" became "was `n > 3`", which it can
  check at the same time as the shift. You wrote *what* to compute. The compiler chose *how*.

---

## Predict, then run

Write your answers down before running anything:

1. What's the value of `let v = { 3; };`? What does `println!("{v:?}")` print?
2. What do `for i in (0..10).step_by(4)` and `for i in (0..=10).rev().step_by(5)` print?
3. How many times does the body of `while n < 0 { ... }` run when `n = 0`? And the body of a
   `loop` whose only `break` is at its end?
4. Does `let x = loop { break 5; };` compile? What about `let x = while true { break 5; };`?
   Why is one accepted and not the other?
5. If `pick_larger` took two `u64`s instead of `i64`s, what would change in its optimized
   assembly?

To check, copy a demo to `examples/my_scratch.rs`, edit it, and run it with `--example
my_scratch`. For question 5, run the `rustc -O --emit=asm` command from section 6 on your
copy.

---

## Exercises

Stubs are in `src/lib.rs`. Each group has its own test file:

| Group | Functions | Command | Difficulty |
|---|---|---|---|
| Fixme | `fixme_01` (`if` as a value), `fixme_02` (loops) | `cargo run -p ch04-control-flow --example fixme_0N --features fixme` | ●○○ |
| Expressions | `grade`, `is_leap_year`, `median3` | `cargo test -p ch04-control-flow --test expressions` | ●○○ – ●●○ |
| Loops | `factorial`, `count_digits`, `gcd`, `collatz_steps`, `is_prime`, `smallest_power_of_two_at_least`, ★`isqrt` | `cargo test -p ch04-control-flow --test loops` | ●○○ – ●●● |
| Labels | `smallest_multiple`, `pythagorean_product` | `cargo test -p ch04-control-flow --test labels` | ●●○ |

Notes:
- **Each doc comment names the construct to practice** (a single `if` expression, a `loop`
  with `break value`, a labeled block). The tests can't check that, but that's the exercise.
- Several functions change their parameter as they go (`n /= 10`). Parameters are immutable,
  so write `mut n` in the signature (section 2).
- `median3`: there are only six orderings of three values. Draw the decision tree on paper
  first. The tests try all six, plus ties.
- `count_digits`: the test for `0` is the lesson. Compare how many times the body of a
  `while` runs with how many times the body of a `loop` with a `break` at the end runs.
- `is_prime`: if a test seems to hang, your loop does far too many steps. (After 60 seconds,
  `cargo test` prints "has been running for over 60 seconds".) And a panic saying `attempt to
  multiply with overflow` means your bound computes something bigger than a `u32` can hold:
  the largest test input is close to `u32::MAX`.
- `smallest_multiple`: `for n in 1..` gives `n` a type from its first use. If you call a
  method on `n` before anything pins that type down, you get E0689. Give the range a type,
  e.g. `1u64..`.
- `pythagorean_product`: `perimeter - a - b` is unsigned. Make sure your loop bounds keep it
  from going below zero, or you'll get `attempt to subtract with overflow` (chapter 3).
- ★ `isqrt`: binary search is easy to get *almost* right. Write down what `lo` and `hi` mean
  (which values could still be the answer?) and make sure every step keeps that true.
- When all tests pass, run `cargo clippy -p ch04-control-flow` and address what it says.

---

## ★ Project step: first statistics functions (minipolars)

Time for minipolars to compute something. You'll add six functions: the start of what becomes
`df["col"].mean()` and friends.

### A library next to the binary

So far `project/minipolars` is a binary crate (`src/main.rs`). Tests can't call a binary's
functions, only a library's. A package can hold **both**: create
**`project/minipolars/src/lib.rs`**, and Cargo builds a library crate called `minipolars`
from it, next to the binary. Its `pub fn`s are what the tests `use`. Leave `main.rs` alone:
`m02_version.rs` must keep passing. (Chapter 15 covers packages and crates properly.)

### The spec

```rust
pub fn sum(values: &[f64]) -> f64
pub fn mean(values: &[f64]) -> f64
pub fn min(values: &[f64]) -> f64
pub fn max(values: &[f64]) -> f64
pub fn variance(values: &[f64]) -> f64   // the SAMPLE variance: divide by n - 1
pub fn std_dev(values: &[f64]) -> f64    // the square root of variance()
```

| Function | Empty input | Why |
|---|---|---|
| `sum` | `0.0` | Adding nothing gives 0. |
| `mean` | NaN | 0 / 0 (chapter 3: float division by zero doesn't panic). |
| `min` | `f64::INFINITY` | Every value is ≤ infinity, so it's the neutral starting point. |
| `max` | `f64::NEG_INFINITY` | The same, the other way around. |
| `variance`, `std_dev` | NaN for fewer than **2** values | You can't estimate spread from one point. |

You may assume the input contains no NaN. (Missing values arrive with nulls, in chapter 13.)

About the variance: statistics has two. The **population** variance divides the sum of
squared deviations from the mean by `n`. The **sample** variance divides by `n - 1`, which
corrects for estimating the mean from the same data. pandas' `.var()` and polars' `.var()`
use `n - 1` by default, and NumPy's `np.var` uses `n`. minipolars follows pandas and polars.

### New syntax: a first look at slices

`&[f64]` is a **slice**: a view of a sequence of `f64`s stored side by side in memory, like a
1-D NumPy array you're allowed to read. Chapters 5 (arrays), 8 (`&`) and 9 (slices) explain
what it really is. For this milestone, you need three things:

```rust
values.len()          // the number of values, as a usize
values[i]             // the value at index i (panics if i >= len)
for &x in values {    // loop over the values, x: f64
    // ...
}
```

Why `&x` and not `x`? The loop hands you each element **by reference**: the address of the
value, not the value itself. The `&` in the pattern says "follow the reference and copy the
`f64` out". If you write `for x in values`, then `x` is a `&f64`, and `x < smallest` fails
with E0308 "expected `&f64`, found `f64`". (`for i in 0..values.len()` with `values[i]` also
works, but clippy will suggest the direct loop: `needless_range_loop`.)

### Install the tests and go

```sh
cp chapters/ch04-control-flow/milestone/m04_stats.rs project/minipolars/tests/
cargo test -p minipolars
```

Hints:
- Choose the **starting value** of each loop so that the empty case comes out right without
  a special case. Only `variance` needs an explicit check. When it does, return early.
- `max(&[-5.0, -3.0, -9.0])` is `-3.0`. Is `0.0` a good starting value?
- `n - 1` where `n` is a `usize`: what happens when `n` is 0 (chapter 3)?
- Functions can call your other functions: `mean` can use `sum`, and `std_dev` is one line.
  `f64` has a `.sqrt()` method.

---

## Compiler errors you'll likely meet

| Error | Meaning |
|---|---|
| E0308 `` `if` and `else` have incompatible types `` | The branches produce different types. Often a stray `;` turned one branch into `()`. |
| E0317 `` `if` may be missing an `else` clause `` | An `if` is used as a value but has no `else`: what would it be when the condition is false? |
| E0308 ``expected `bool`, found integer`` | No truthiness in Rust. Compare explicitly: `if n != 0`. |
| E0425 `` cannot find value `x` in this scope `` | `x` was declared inside a block or loop body, and it's gone after the closing `}`. Declare it before the loop. |
| E0571 `` `break` with value from a `for` loop `` | Only `loop` can produce a value. Use `loop`, or a labeled block. |
| E0277 `` `()` doesn't implement `std::fmt::Display` `` | You printed something whose value is `()`: a `for` loop, a block ending in `;`... |
| E0689 ``can't call method `…` on ambiguous numeric type `{integer}` `` | The compiler can't pick an integer type yet. Annotate: `1u64..` or `let n: u64 = ...`. |
| E0308 ``expected `&f64`, found `f64` `` | In `for x in values`, `x` is a reference. Use `for &x in values`. |
| E0384 `cannot assign twice to immutable variable` | Modifying a parameter: write `mut n` in the signature. |
| panic `attempt to subtract with overflow` | An unsigned value (often a loop bound or `len() - 1`) went below zero. |

---

## nvim tip

Put the cursor on the `if` keyword of an `if`/`else` and press `<leader>ca`. rust-analyzer
offers **"Invert if"**: it swaps the branches and negates the condition. It's a quick way to
see whether a branch reads better the other way around. And keep inlay hints on: they show
the type of every loop variable, and the type of each `let x = if ...`.

---

## Checkpoint

1. What's the difference between an expression and a statement? What does a semicolon do to
   an expression?
2. Why must both branches of an `if` have the same type, and why does an `if` used as a value
   need an `else`?
3. Why can `loop` produce a value with `break`, but `for` and `while` can't?
4. `todo!()` appears in functions returning `u64`, `bool` and `char`. How can one macro have
   all those types?
5. What did `if a > b { a } else { b }` compile to in the optimized build, and why is it
   good that there's no jump?

**Further reading:**
- The Rust Book, [ch. 3.3: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
  and [ch. 3.5: Control flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- Rust by Example, [Flow of control](https://doc.rust-lang.org/rust-by-example/flow_control.html)
- The Rust Reference, [Loops and other breakable expressions](https://doc.rust-lang.org/reference/expressions/loop-expr.html):
  the precise rules, including labeled blocks
- std docs: [`!` (never)](https://doc.rust-lang.org/std/primitive.never.html) and
  [`()` (unit)](https://doc.rust-lang.org/std/primitive.unit.html)

---

## Done?

- `cargo test -p ch04-control-flow` is all green, and `cargo clippy -p ch04-control-flow`
  has nothing left to say.
- `cargo test -p minipolars` passes both `m02_version` and `m04_stats`.
- Add a row to `notes/progress.md`, and write up `notes/ch04.md`.
- Commit: `git add -A && git commit -m "ch04 done" && git push`
- Next: chapter 5, tuples, arrays and a first look at memory (coming soon; see
  [SYLLABUS.md](../../SYLLABUS.md))
