# Chapter 5: Tuples, arrays & a first look at memory

> **Goal:** group values together. Tuples hold a few values of different types, and arrays
> hold many values of one type. Both have a size fixed at compile time, and that's why you can
> see exactly how they sit in memory: byte for byte, with no hidden headers. This is the first
> chapter where you look at memory directly, and it sets up Part 1 (ownership).
> Along the way you'll learn `dbg!`, which makes debugging a failing test much faster.
>
> **Time:** ~1 h. **Prerequisites:** chapter 4.

---

## 1. Coming from Python

| Python | Rust | Note |
|---|---|---|
| `t = (7, 21.5, True)` | `let t = (7, 21.5, true);` | Type `(i32, f64, bool)`: one type **per position**, fixed. |
| `t[0]` | `t.0` | Field access with a literal number, not indexing: `t.i` with a variable doesn't exist. |
| `lo, hi = min_max(a, b)` | `let (lo, hi) = min_max(a, b);` | Same idea: return a tuple, unpack it. |
| `a, b = b, a + b` | `(a, b) = (b, a + b);` | Works exactly the same. |
| `[0] * 5` | `[0; 5]` | An **array**: type `[i32; 5]`. The length is part of the type. |
| `xs.append(4)` | (none) | An array can't grow. The growable list is `Vec` (chapter 14). |
| `b = a` (both names, one list) | `let b = a;` (**a full copy**) | Section 4. |
| `np.zeros(5, dtype=np.int32)` | `[0i32; 5]` | The closest match: raw numbers side by side, 20 bytes. |

The big difference is underneath. A Python list is an array of **pointers**, each to a
separate int object somewhere on the heap. A Rust array is the numbers themselves, one after
the other. A NumPy array is much closer to Rust: a contiguous block of raw numbers (plus a
Python object that describes it). Section 6 measures all three.

---

## 2. Tuples

```sh
cargo run -p ch05-tuples-arrays --example demo_tuples
```

A tuple is a fixed-size group of values. Each position has its own type, and the tuple's type
lists them all: `(u32, f64, bool)`.

```rust
let reading: (u32, f64, bool) = (7, 21.5, true);
let id = reading.0;                     // fields are .0, .1, .2 ...
let (id, celsius, _) = reading;         // destructuring; `_` ignores a position

fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a <= b { (a, b) } else { (b, a) }    // several return values = one tuple
}

(x, y) = (y, x);                        // destructuring assignment: a swap, no temporary
```

- `.0`, `.1` are **fields**, checked at compile time. `reading.3` is a compile error (E0609),
  not an `IndexError` at runtime. For the same reason there's no `reading.i` with a variable
  `i`: the compiler must know which field, and so which type, you mean.
- Tuples compare position by position, like Python: `(1, 9) < (2, 0)` is `true`.
- A one-element tuple needs a comma, `(5,)`. Without it, `(5)` is just `5` in parentheses.
- The zero-element tuple is `()`: chapter 4's **unit** type *is* the empty tuple.

Tuples are for small, short-lived groups, where the position says enough. Once the fields
deserve names (`reading.celsius` instead of `reading.1`), you want a struct (chapter 11).

---

## 3. Arrays

```sh
cargo run -p ch05-tuples-arrays --example demo_arrays
```

An array is a fixed number of values, all of one type, stored side by side.

```rust
let primes: [u32; 5] = [2, 3, 5, 7, 11];    // type: [element type; length]
let mut counts = [0u32; 10];                 // [value; count]: ten zeros
counts[3] += 1;                              // indices are usize, starting at 0
let n = primes.len();                        // 5, known at compile time
let [red, green, blue] = [255u8, 128, 0];    // destructuring works for arrays too
let grid = [[0; 3]; 2];                      // 2 rows of 3: grid[row][column]
```

- **The length is part of the type.** `[u32; 5]` and `[u32; 6]` are different types, and a
  function that takes a `[f64; 7]` accepts exactly seven values. An array can't grow or
  shrink. (That's `Vec`, chapter 14. And functions that accept *any* length, like
  minipolars' `&[f64]`, take a slice: chapter 9.)
- **All elements have the same type.** `[1, 2.5]` is an error: no silent conversion to float.
- **Indices are `usize`.** Indexing with an `i32` or `u64` is error E0277, "the type `[i32]`
  cannot be indexed by `i32`". Convert with `as usize` (chapter 3). Why `usize`? An index
  becomes part of a memory address (section 6), and `usize` is the address-sized integer.
- **Every index is checked.** `primes[7]` doesn't read whatever happens to be in memory
  after the array. It panics: `index out of bounds: the len is 5 but the index is 7`. If the
  compiler can already see the index is out of bounds, it refuses to compile the program.

### Looping

```rust
for p in primes {                              // each element, by value
    total += p;
}

for (i, p) in primes.into_iter().enumerate() { // (index, element) tuples, destructured
    println!("primes[{i}] = {p}");
}
```

`.enumerate()` turns each element into an `(index, element)` tuple, like Python's
`enumerate()`, and the `for` pattern destructures it. Chapter 20 explains what
`.into_iter()` and `.enumerate()` really are. For now, use the pattern as it is. If you write
`for i in 0..primes.len()` and then use `primes[i]`, clippy suggests almost the same thing,
spelled `primes.iter().enumerate()`: `.iter()` hands out references to the elements
(chapter 8), while `.into_iter()` on an array of numbers hands out copies.

**Try it:** `cargo run -p ch05-tuples-arrays --example fixme_01 --features fixme` (types), then
`cargo run -p ch05-tuples-arrays --example fixme_02 --features fixme` (indexing). In the
second one, fixing the first error reveals a new one: the compiler checks your program in
phases, and the out-of-bounds check only runs once the types are right.

---

## 4. Copies, not shared lists

This one surprises every Python programmer:

```rust
let a = [1, 2, 3, 4, 5];
let mut b = a;          // copies all five numbers
b[0] = 99;
// a is still [1, 2, 3, 4, 5], b is [99, 2, 3, 4, 5]
```

In Python, `b = a` makes a second name for the **same** list, and `b[0] = 99` changes `a`
too. In Rust, a variable **is** its value: `a` is 20 bytes of numbers, and `let b = a`
makes 20 more bytes, a separate copy. The same happens when you pass an array or a tuple to a
function: the function gets its own copy, so it can change its parameter (`mut values`)
without the caller ever noticing. The last test in `tests/grids.rs` checks exactly that.

For five numbers, copying is cheaper than anything else you could do. For a
`[f64; 1_000_000]` it would copy 8 MB on every call, so you'd pass a **reference** instead
(chapter 8). And for types that own memory elsewhere, like a `String`, assignment doesn't
copy but **moves** (chapter 7). Tuples and arrays copy because everything inside them is a
plain number.

---

## 5. Seeing your values: `{:?}` and `dbg!`

`{}` (the `Display` format) works for numbers and text, but not for tuples or arrays: there's
no single obvious way to show them to a user. `{:?}` (the `Debug` format) prints them the way
a programmer wants to see them, and `{:#?}` spreads nested values over several lines:

```rust
println!("{primes:?}");         // [2, 3, 5, 7, 11]
println!("{:?}", (1, 'a'));     // (1, 'a')
```

For debugging, **`dbg!`** is better still. It prints the file and line, the expression's source
code and its value, and then returns the value, so you can wrap it around any expression
without changing what the code does:

```rust
let area = dbg!(width * 10) * height;
// prints: [src/lib.rs:12:16] width * 10 = 30
```

It prints to stderr, so it doesn't mix with a program's real output. Use it like `print()`
debugging in Python, and delete it when you're done.

### Debugging a failing test

Chapters 3 and 4 had plenty of "why does this test fail?". Here's a faster loop than rereading
your code:

1. **Run only the failing test.** Any word after the command filters tests by name:
   `cargo test -p ch05-tuples-arrays --test tuples divmod` runs every test with `divmod` in
   its name.
2. **Read the assertion.** `left` is what your function returned, `right` is what the test
   expected. The test's source (`tests/*.rs`) shows which call produced it.
3. **Add `dbg!`s** in your function (inside the loop, for example) and rerun. `cargo test`
   captures output and shows it under `---- test_name stdout ----` **for failing tests
   only**. To see it for passing tests too, add `-- --nocapture` at the end of the command.

---

## 6. Under the hood: a first look at memory

```sh
cargo run -p ch05-tuples-arrays --example demo_memory
```

### Arrays are their elements, and nothing else

```text
size_of::<[i32; 5]>()      = 20      5 × 4 bytes
size_of::<[u8; 1000]>()    = 1000
size_of::<[[f64; 3]; 3]>() = 72      9 × 8 bytes
```

There's no header and no stored length: the length is in the **type**, so the compiler
already knows it and never needs to store it. In an optimized build, `primes.len()` is just
the constant 5.

The demo prints the address of each element (`&x` means "the address of `x`", and `{:p}`
prints it: chapters 6 and 8 explain both). Your addresses will differ, but they step by 4
bytes, the size of an `i32`:

```text
&numbers[0] = 0x16d349720
&numbers[1] = 0x16d349724
&numbers[2] = 0x16d349728
&numbers[3] = 0x16d34972c
```

So an array is one block of memory:

```text
address:  …720      …724      …728      …72c
         ┌─────────┬─────────┬─────────┬─────────┐
numbers  │   10    │   20    │   30    │   40    │   16 bytes, nothing before or after
         └─────────┴─────────┴─────────┴─────────┘
```

That's also how indexing works. The address of `numbers[i]` is `start + i × 4`: one multiply
and one add, whatever the length. (This is why an index must be a `usize`: it goes into an
address calculation. The bounds check is one comparison before it, `i < 4`.)

In C, there's no such check: reading `numbers[7]` just reads the 4 bytes that happen to come
next in memory. That's a *buffer overflow*, one of the most common security bugs in history.
In Rust it's a panic, or a compile error if the index is a constant.

A grid, `[[u8; 3]; 2]`, is its rows back to back: `grid[0][0..3]`, then `grid[1][0..3]`,
six consecutive bytes. It's the same layout as a NumPy array in its default "C order" (row-major).
Walking along a row visits neighboring bytes, and chapter 29 shows why that's much faster than
walking down a column.

### Python, measured

On this machine, with Python 3.12:

| | Bytes | Why |
|---|---|---|
| Rust `[i64; 3]` | **24** | 3 × 8 bytes of numbers |
| NumPy `np.zeros(3, dtype=np.int64)` | 24 of data, 136 in all | the same 24 bytes, plus a ~112-byte Python object describing them |
| Python `[1, 2, 3]` | 88 for the list, plus 28 per int object | a 56-byte list header, then 3 × 8-byte **pointers** to separate int objects |

```text
Python list                         Rust [i64; 3]
┌────────┬─────┬─────┬─────┐        ┌───┬───┬───┐
│ header │ ptr │ ptr │ ptr │        │ 1 │ 2 │ 3 │
└────────┴──┬──┴──┬──┴──┬──┘        └───┴───┴───┘
            ▼     ▼     ▼
          ┌───┐ ┌───┐ ┌───┐
          │ 1 │ │ 2 │ │ 3 │  each a 28-byte object
          └───┘ └───┘ └───┘
```

Summing the Python list means following a pointer to a different object for every element.
Summing the Rust array means reading consecutive memory. That difference, more than the
interpreter, is why NumPy (and Polars, and soon minipolars) store columns as contiguous raw
numbers.

### Tuples: alignment and padding

Every type has an **alignment**: its address must be a multiple of it. A `u32` has alignment
4, a `u64` 8. The CPU loads aligned values in one step. So when you mix sizes in a tuple, the
compiler inserts unused **padding** bytes to keep each field aligned:

```text
(u8, u32): size 8, align 4
byte:   0     1     2     3     4     5     6     7
     ┌─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┐
     │ .0  │ pad │ pad │ pad │ .1                    │   .1 must start at a multiple of 4
     └─────┴─────┴─────┴─────┴───────────────────────┘

(u64, u8): size 16, align 8
byte:   0 ...................... 7     8     9 ..................... 15
     ┌─────────────────────────────┬─────┬───────────────────────────┐
     │ .0                          │ .1  │ pad (7 bytes)             │
     └─────────────────────────────┴─────┴───────────────────────────┘
```

Why does `(u64, u8)` end with 7 wasted bytes? In an array `[(u64, u8); 4]`, the element after
it has to start at an 8-aligned address too. **A type's size is always a multiple of its
alignment**, so `[(u64, u8); 4]` is exactly 4 × 16 = 64 bytes.

And look at `(u8, u32, u8)` in the demo output: size 8, not 12, with `.1` at offset 0.
**Rust reorders fields** to waste less space: the `u32` goes first and the two `u8`s share
the next 4 bytes. In C, fields stay in the order you wrote them. Rust only promises that when
you ask for it (`#[repr(C)]`, chapters 11 and 36).

### Where they live: the stack

A local variable, array or tuple, lives in its function's **stack frame**: a block of memory
reserved when the function starts and released when it returns. The end of `demo_memory`
prints the addresses of three locals of `main`, all within a few kilobytes of each other.
Chapter 6 explains the stack properly. For now, one consequence: the stack is small. The
main thread gets **8 MB** on macOS (`ulimit -s` prints `8176`, in kilobytes), so a
`[u8; 10_000_000]` local doesn't fit (see "Predict, then run", question 5). Big data goes on
the heap: `Vec`, `Box` (chapter 6).

---

## Predict, then run

Write your answers down before running anything:

1. What are `size_of::<(u8, u16)>()`, `size_of::<(u16, u8, u16)>()` and
   `size_of::<[(u8, u16); 10]>()`?
2. `size_of::<(u64, u8)>()` is 16. What's `size_of::<((u64, u8), u8)>()`? Can the outer `u8`
   use the inner tuple's padding?
3. After `let t = (1, [2, 3]); let mut u = t; u.1[0] = 99;`, what's `t`?
4. In `demo_memory`, the addresses of `numbers[0]` and `numbers[1]` differ by 4. By how much
   do `grid[0][2]` and `grid[1][0]` differ? And the elements of a `[(u64, u8); 4]`?
5. Put `let big = [0u8; 9 * 1024 * 1024];` in `main`, **after** a `println!("start")`, and
   use it afterwards (for example `println!("{}", big[1])`). Run it (a debug build). Is
   `start` printed?

To check, copy a demo to `examples/my_scratch.rs`, edit it, and run it with `cargo run -p
ch05-tuples-arrays --example my_scratch`. For question 5, the answer to "why?" is that a
function's whole stack frame, `big` included, is reserved **when the function starts**,
before its first line runs.

---

## Exercises

Stubs are in `src/lib.rs`. Each group has its own test file:

| Group | Functions | Command | Difficulty |
|---|---|---|---|
| Fixme | `fixme_01` (types), `fixme_02` (indexing) | `cargo run -p ch05-tuples-arrays --example fixme_0N --features fixme` | ●○○ |
| Tuples | `to_hms`, `fib_pair`, `divmod` | `cargo test -p ch05-tuples-arrays --test tuples` | ●○○ – ●●○ |
| Arrays | `rgb_to_hex`, `hex_to_rgb`, `digit_histogram`, `warmest_day` | `cargo test -p ch05-tuples-arrays --test arrays` | ●○○ – ●●○ |
| Grids | `transpose`, `mat_mul`, ★`tic_tac_toe_winner` | `cargo test -p ch05-tuples-arrays --test grids` | ●○○ – ●●● |

This chapter has fewer exercises than chapter 4, and most are short: a ●○○ should take
minutes. If one takes much longer, run just that test and `dbg!` it (section 5).

Notes:
- `fib_pair`: `(a, b) = (b, a + b)` evaluates the whole right side first, from the **old**
  values, and only then assigns. That's why no temporary is needed.
- `divmod`: work out Python's answer and Rust's `/` and `%` for `-7, 2` and `7, -2` on paper
  first. When do they disagree, and by how much? (`i64::div_euclid` looks like the answer
  but isn't: the tests with a negative `b` catch it.)
- `rgb_to_hex`/`hex_to_rgb`: the colors are `u8`s and the hex code is a `u32`, so you'll
  convert with `as` (chapter 3). Which direction truncates, and is that what you want?
- `digit_histogram`: a digit is `n % 10`, a `u64`. An index must be a `usize`.
- `warmest_day`: a function returns one value, so the "best so far" is a pair. Either keep
  two variables or one tuple; try the tuple. And the test with a Finnish January is there for
  a reason: what's a safe starting value?
- `transpose`: your function gets a copy of the matrix. The test
  `transpose_leaves_its_argument_alone` shows the caller's matrix is unchanged afterwards.
- ★ `tic_tac_toe_winner`: the table's type is an array of 8 lines, each an array of 3
  coordinates, each a tuple. You can destructure all of it in the `for` pattern.
- When all tests pass, run `cargo clippy -p ch05-tuples-arrays` and address what it says.

There's no minipolars step in this chapter. The next one comes in chapter 9, when minipolars
reads its first CSV file.

---

## Compiler errors you'll likely meet

| Error | Meaning |
|---|---|
| E0308 ``expected an array with a size of 5, found one with a size of 6`` | The length is part of the type: the literal and the annotation disagree. |
| E0308 ``expected integer, found floating-point number`` | Arrays hold one type. Write `20.0`, not `20`, among floats. |
| E0609 ``no field `2` on type `(i32, f64)` `` | Tuple fields start at `.0`. |
| E0277 ``the type `[i32]` cannot be indexed by `i32` `` | Indices are `usize`. Convert with `as usize`, or make the variable a `usize` to begin with. |
| error ``this operation will panic at runtime`` | A constant index is out of bounds, and the compiler can prove it. |
| E0308 ``expected a tuple with 2 elements, found one with 3 elements`` | A returned tuple doesn't match the `-> (...)` in the signature. |
| E0277 `` `[i32; 3]` doesn't implement `std::fmt::Display` `` | Print tuples and arrays with `{:?}`, not `{}`. |
| E0594 ``cannot assign to `a[_]`, as `a` is not declared as mutable`` | Changing an element needs the whole array to be `let mut`. |
| panic ``index out of bounds: the len is 3 but the index is 3`` | An index computed at runtime was too big. The last valid index is `len - 1`. |

---

## nvim tip

Press `K` on a local variable, for example `numbers` or `grid` in `demo_memory.rs`.
rust-analyzer's hover includes the value's memory layout: for `numbers` it's
`size = 16 (0x10), align = 0x4, no Drop` (`Drop` is chapter 7). It's `size_of` without
running anything. (If you don't see it, it's the `hover.memoryLayout` setting.) Then write `let t: (u8, u32, u8) = (1, 2, 3);` and hover over `t`: does the size
match the demo's output?

---

## Checkpoint

1. What's the difference between `t.0` on a tuple and `a[0]` on an array? Why can't you write
   `t.i` with a variable `i`?
2. Why is `[i32; 5]` 20 bytes and not more? Where is its length stored?
3. Why is `(u64, u8)` 16 bytes? What's padding for?
4. After `let b = a;`, where `a` is an array, what happens to `b` when you change `a`? What
   would happen in Python with a list?
5. What does `dbg!` print, where does it print it, and why can you wrap it around an
   expression without changing your program?

**Further reading:**
- The Rust Book, [ch. 3.2: Data types, compound types](https://doc.rust-lang.org/book/ch03-02-data-types.html#compound-types)
- Rust by Example, [Tuples](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html)
  and [Arrays and slices](https://doc.rust-lang.org/rust-by-example/primitives/array.html)
- std docs: [`array`](https://doc.rust-lang.org/std/primitive.array.html),
  [`tuple`](https://doc.rust-lang.org/std/primitive.tuple.html) and
  [`dbg!`](https://doc.rust-lang.org/std/macro.dbg.html)
- The Rust Reference, [Type layout](https://doc.rust-lang.org/reference/type-layout.html):
  the exact rules for size, alignment and tuple layout

---

## Done?

- `cargo test -p ch05-tuples-arrays` is all green, and `cargo clippy -p ch05-tuples-arrays`
  has nothing left to say.
- Add a row to `notes/progress.md`, and write up `notes/ch05.md`.
- Commit: `git add -A && git commit -m "ch05 done" && git push`
- Next: chapter 6, process memory: the stack and the heap (coming soon; see
  [SYLLABUS.md](../../SYLLABUS.md))
