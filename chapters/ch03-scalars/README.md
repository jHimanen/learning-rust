# Chapter 3: Scalars & bits

> **Goal:** understand Rust's primitive types as what they really are: **fixed-size patterns
> of bits**. You'll learn how integers are stored (including negative ones), what happens when
> they overflow, how casts can silently change values, why `0.1 + 0.2 != 0.3`, what a `char`
> really is, and how to manipulate individual bits.
>
> **Time:** ~1 h. **Prerequisites:** chapter 2 (the exercise loop).

---

## 1. Coming from Python

In Python, `int` is magic: it grows as big as needed. That's because a Python int is an
**object on the heap** (a header, a reference count, a type pointer and an array of digits):

```python
>>> import sys
>>> sys.getsizeof(1)
28          # bytes, to store the number 1
>>> 2 ** 200
1606938044258990275541962092341162602522202993782792835301376
```

In Rust, a number is just its bits: a `u8` is **1 byte**, and an `i64` is **8 bytes** that fit
in one CPU register. There's no header, no heap allocation and no type lookup. That's where a
lot of the speed you saw in chapter 1 comes from. The price is that **you choose the size, and
values can overflow**.

As a data scientist you already know this world: **Rust's primitive types are numpy
dtypes.** `u8` is `np.uint8`, `i32` is `np.int32`, `f32` is `np.float32`. And like numpy,
where `np.uint8(250) + np.uint8(10)` gives `4` (with a RuntimeWarning, if you're lucky), values
can wrap around. Rust makes you decide what should happen in that case (section 4).

---

## 2. Variables: `let`, `mut`, shadowing, `const`

```rust
let x = 5;             // immutable, and the type is inferred: i32 (the default integer type)
let y: u8 = 5;         // explicit type annotation
let mut count = 0;     // mutable: may be reassigned
count += 1;

let data = "42";       // shadowing: a NEW variable with the same name,
let data = 42;         // which may even have a different type

const MAX_ROWS: usize = 1_000; // a compile-time constant: type required, SCREAMING_CASE
```

- **Immutable by default.** This is the opposite of Python. Most variables never change, and
  saying so up front lets both the compiler and the reader rely on it. `mut` marks the
  exceptions.
- **Shadowing** isn't mutation: the old variable still exists (you just can't name it
  anymore). It's handy for transforming a value step by step (`let line = line.trim();`).
- If the compiler can't infer a type, it uses defaults: **`i32`** for integers and **`f64`**
  for floats.
- The editor can show every inferred type as an inlay hint (chapter 2, section 7). Turn them on
  while working through this chapter.

**Try it:** `cargo run -p ch03-scalars --example fixme_01 --features fixme` doesn't compile.
Read the error and fix it (the file is yours to edit).

---

## 3. Integers

| Size | Signed | Unsigned | Signed range |
|---|---|---|---|
| 8 bits | `i8` | `u8` | -128 ..= 127 |
| 16 bits | `i16` | `u16` | -32,768 ..= 32,767 |
| 32 bits | `i32` | `u32` | about ±2.1 billion |
| 64 bits | `i64` | `u64` | about ±9.2 × 10¹⁸ |
| 128 bits | `i128` | `u128` | huge |
| pointer-sized | `isize` | `usize` | 64 bits on your Mac |

**`usize`** is the type Rust uses for **sizes and indices**: lengths of collections, array
positions, memory offsets. It's as wide as a memory address on the machine (8 bytes here).
You'll see it constantly.

```sh
cargo run -p ch03-scalars --example demo_integers
```

Read the demo's source alongside the output. It shows sizes and ranges, all the literal forms
(`255`, `0xff`, `0o377`, `0b1111_1111`, `255u8`, `b'A'`), and the formatting options
(`{:b}`, `{:08b}`, `{:x}`, `{:#x}`).

### Under the hood: two's complement

How do you store `-1` in 8 bits when there's no minus sign in hardware? Almost every CPU uses
**two's complement**: the top bit has a **negative** weight.

```
bit:     7     6    5    4    3    2    1    0
weight: -128   64   32   16   8    4    2    1

0000_0001 =                                  1  =    1
0111_1111 =        64+ 32+ 16+  8+  4+  2+  1  =  127   (largest i8)
1000_0000 = -128                                = -128   (smallest i8)
1111_1111 = -128 + 64+ 32+ 16+  8+  4+  2+  1  =   -1
```

Why this scheme? Because **addition works the same for signed and unsigned numbers**. The CPU
has a single add instruction and doesn't care which interpretation you meant. `1111_1111 +
0000_0001` wraps to `0000_0000`: as unsigned that's 255 + 1 → 0 (overflow), and as signed it's
-1 + 1 = 0 (correct!). This is also why `i8` has one more negative value than positive ones:
there's no +128.

In the demo output, compare `-1` with `1111_1111` and `-128` with `1000_0000`. The same 8
bits mean `255` as a `u8` and `-1` as an `i8`. **The type is how you interpret the bits.**

### Under the hood: endianness

The last part of the demo shows the four bytes of `0x12345678` in memory. Your ARM64 CPU (and
x86) is **little-endian**: the *least* significant byte is stored first, so memory holds
`78 56 34 12`. Network protocols and many file formats use big-endian. You'll care about this
when you read binary data (chapter 29 and later).

---

## 4. Overflow

What's `250u8 + 10`? The true answer, 260, doesn't fit in 8 bits. **Predict**, then run both:

```sh
cargo run -p ch03-scalars --example demo_overflow
cargo run -p ch03-scalars --example demo_overflow --release
```

- **Debug builds panic** with `attempt to add with overflow`. Overflow is almost always a bug,
  so in development you hear about it immediately.
- **Release builds wrap** silently (260 − 256 = 4). Checking every operation costs speed, so
  it's off in release. Don't *rely* on wrapping, though: it's still considered a bug.
- If the compiler can see the overflow at compile time (`let x: u8 = 250 + 10;`), it refuses
  to compile at all.

When overflow is *expected*, say what you want explicitly. Every integer type has these
method families:

| Method | `250u8.___(10)` | Use when |
|---|---|---|
| `wrapping_add` | `4` | you *want* modular arithmetic (hashes, counters, checksums) |
| `saturating_add` | `255` | clamping to the range makes sense (volume, pixel values) |
| `checked_add` | `None` | overflow is an error you want to handle (chapter 13 explains `Option`) |
| `overflowing_add` | `(4, true)` | you need both the result and whether it overflowed |

There are `_sub`, `_mul`, `_neg`, `_pow`… versions of all of them.

---

## 5. Casting with `as`

Rust **never converts between numeric types implicitly**. Not even `u8` → `u32`, which is
always safe. `a + b` requires `a` and `b` to have the same type. You convert explicitly with
`as`:

```sh
cargo run -p ch03-scalars --example demo_casts
```

The rules (`as` never fails, so learn where it silently changes the value):

| Cast | Behavior | Example |
|---|---|---|
| to a wider type | value preserved (sign-extended for signed types) | `-5i8 as i64` → `-5` |
| to a narrower integer | **keeps the low bits** (truncates) | `300i32 as u8` → `44` |
| signed ↔ unsigned, same size | same bits, reinterpreted | `200u8 as i8` → `-56` |
| float → int | rounds toward zero, **saturates** at the range limits, NaN → 0 | `-3.99 as i32` → `-3` |
| int → float | nearest representable value (may lose precision) | `(2^53+1) as f64` → `2^53` |

**Try it:** `cargo run -p ch03-scalars --example fixme_02 --features fixme` needs a couple of
casts. Watch out for *where* the conversion happens.

(Later you'll meet `From`/`TryFrom`, the checked alternatives to `as` for when a lossy
conversion should be an error.)

---

## 6. Floats

`f64` (the default) and `f32` are IEEE 754 binary floating-point numbers, exactly like
Python's `float` (which is an f64) and numpy's `float32`/`float64`.

```sh
cargo run -p ch03-scalars --example demo_floats
```

### Under the hood: the bits of an f64

```
 1 bit    11 bits          52 bits
┌──────┬────────────┬──────────────────────────────┐
│ sign │  exponent  │           mantissa           │   value = ±1.mantissa × 2^(exponent − 1023)
└──────┴────────────┴──────────────────────────────┘
```

The demo prints these fields for a few numbers. Look at `0.1`: its mantissa is the repeating
pattern `1001 1001 1001…`, cut off after 52 bits. 0.1 has **no exact binary representation**,
just as 1/3 has none in decimal. So `0.1 + 0.2` is `0.30000000000000004`, and comparing
floats with `==` is almost always a mistake. Compare with a tolerance instead.

Things worth knowing:
- **NaN** ("not a number", e.g. `0.0 / 0.0`) is **not equal to anything, including itself**.
  Every comparison with NaN is `false`. This will matter a lot when you sort and group floats
  in minipolars (chapters 24–25).
- Float division by zero doesn't panic: it gives `inf`, `-inf` or `NaN`. (Integer division by
  zero *does* panic.)
- `f32` has about 7 significant digits and `f64` about 16. `16777216f32 + 1.0` is still
  `16777216`. (ML frameworks use `f32`, and even 16-bit floats, because the memory and speed
  savings beat the precision loss for weights.)

---

## 7. `bool` and `char`

```sh
cargo run -p ch03-scalars --example demo_chars
```

- `bool` is `true`/`false`, stored in 1 byte. There's no truthiness: `if 1 { }` is a type
  error. Conditions must be `bool`.
- `char` is **4 bytes**: one Unicode scalar value (a "code point"), from `U+0000` to
  `U+10FFFF`. `'A'` is `U+0041`, `'é'` is `U+00E9` and `'🦀'` is `U+1F980`.
- `u8` is a byte. `b'A'` is a **byte literal**, of type `u8` with value 65. Text *files* and
  Rust *strings* are sequences of bytes in UTF-8 encoding, where one `char` takes 1 to 4 bytes.
  (Chapter 9 is all about this, and it matters for CSV parsing.)
- `'a' as u32` gives the code point. `65u8 as char` goes the other way (only from `u8`, since
  not every `u32` is a valid char).
- Note the quotes: `'a'` is a `char` and `"a"` is a string. They're different types.

### Under the hood: a char is just a number

Memory only holds numbers, so a `char` is a number too: a `u32`-sized integer that Rust
promises is a valid code point. **Unicode is a lookup table** from numbers to symbols, and
ASCII is its first 128 entries (0–127). The symbol only appears when something, like your
terminal, draws the number using a font.

So `'A' as u32` doesn't *convert* anything. It reads back the number that was already stored
(65), exactly like Python's `ord('A')`. `65u8 as char` is Python's `chr(65)`. And `b'A'` is
just another way of writing the number 65 as a `u8`. Since they're numbers, you can compare
them, subtract them and do bit operations on them like any other integer (after an `as` to
get both sides to the same type).

An excerpt of the ASCII table:

| Symbols | Decimal values |
|---|---|
| space | `32` |
| `'0'` `'1'` `'2'` … `'9'` | `48` `49` `50` … `57` |
| `'A'` `'B'` `'C'` … `'Z'` | `65` `66` `67` … `90` |
| `'a'` `'b'` `'c'` … `'z'` | `97` `98` `99` … `122` |

Each block is **consecutive**: the table was designed so that the next digit or letter is
always the next number. The layout was also chosen with the *binary* values in mind, which
decimal hides. To see the patterns, print some byte literals with `{:08b}` (from §3): the
digits, a letter in both cases, a few letters in a row. The end of `demo_chars` starts you
off, and you can copy it to `examples/my_scratch.rs` to explore further (see "Predict, then
run" below).

Everything in ASCII fits in 7 bits, and UTF-8 stores those code points as a single byte with
the same value. That's why ASCII text is also valid UTF-8. Code points above 127 need more
bytes.

---

## 8. Bitwise operations

| Operator | Name | `0b1100 ⋄ 0b1010` | Python |
|---|---|---|---|
| `&` | AND: 1 where **both** are 1 | `0b1000` | `&` |
| `\|` | OR: 1 where **either** is 1 | `0b1110` | `\|` |
| `^` | XOR: 1 where they **differ** | `0b0110` | `^` |
| `!x` | NOT: flip every bit | `!0b1100u8` = `0b1111_0011` | `~` |
| `<<` | shift left: `x << n` moves bits up by n (× 2ⁿ) | `0b0011 << 2` = `0b1100` | `<<` |
| `>>` | shift right: moves bits down by n (÷ 2ⁿ) | `0b1100 >> 2` = `0b0011` | `>>` |

Note that `!` is bitwise NOT for integers, and logical NOT for `bool`. Python spells bitwise
NOT `~`.

A **mask** is a number with 1s in the bit positions you care about. AND with a mask *selects*
bits, OR with a mask *turns bits on*, and XOR with a mask *flips* them. Combined with shifts,
this lets you treat a single integer as a row of individual flags, or pack several small
numbers into one bigger number.

Why bother, in 2026? Because this is how compact data formats work. **Apache Arrow and Polars
store "is this value null?" as one bit per row**: a validity bitmap, 8 rows per byte. You'll
build exactly that for minipolars in chapter 29. The bit exercises below are the warm-up.

---

## 9. Reading and writing function bodies

The exercises are functions. You've seen the shape:

```rust
/// Returns the square of `x`, plus one.
pub fn square_plus_one(x: i64) -> i64 {
    let squared = x * x;
    squared + 1          // last expression, no `;` = the return value
}
```

- Parameters **always** need types, and so do return values (`-> i64`). Inside the body,
  types are mostly inferred.
- A block's value is its last expression. Adding a `;` turns it into a statement, and the
  function would return `()` ("unit", the empty value) instead: error E0308.
- `&&` and `||` are logical and/or. They **short-circuit**: in `a && b`, `b` isn't evaluated
  if `a` is `false`. That's useful for guarding against an edge case.
- Comparison operators (`==`, `!=`, `<`, `>=`...) produce a `bool`.
- Primitive types have **methods**, called with a dot: `x.abs()`, `x.pow(2)`,
  `a.wrapping_add(b)`. They also have **associated constants**: `u8::MAX`, `f64::NAN`.
- **Finding methods is a core skill.** `K` on a type name in nvim shows its docs. For the full
  list, open `rustup doc --std`, search for `u32` (the primitive type page), and skim the
  method names. A few exercises require this.
- Operator precedence: `*` `/` `%` bind tighter than `+` `-`, which bind tighter than `<<`
  `>>`, then `&`, then `^`, then `|`, then comparisons, then `&&`, then `||`. **When in doubt,
  add parentheses.** Clippy will suggest them where precedence is surprising.

---

## Predict, then run

Write your answers down before running anything:

1. What's `-1i8 as u8`? And `-128i8 as u8`? (Check with the two's complement table.)
2. What does `0.1f32 as f64` print? Why isn't it `0.1`?
3. What's `(1u8 << 7) >> 7`? And `(1u8 << 7) as i8`?
4. `let x: i32 = 7 / 2;` What is `x`? What about `-7 / 2`, and `-7 % 2`?
5. Is `f64::NAN < f64::INFINITY`? Is `f64::NAN > f64::INFINITY`?

To check, copy a demo to `examples/my_scratch.rs`, edit it, and run it with `--example
my_scratch`.

---

## Exercises

Stubs are in `src/lib.rs`. Each group has its own test file:

| Group | Functions | Command | Difficulty |
|---|---|---|---|
| Fixme | `fixme_01` (mut), `fixme_02` (casts) | `cargo run -p ch03-scalars --example fixme_0N --features fixme` | ●○○ |
| Integers | `mean_from_sum`, `low_byte`, `as_signed`, `python_floor_div` | `cargo test -p ch03-scalars --test integers` | ●○○ – ●●○ |
| Overflow | `average_u8`, `volume_up`, `counter_advance`, `abs_difference`, `negate_wrapping` | `cargo test -p ch03-scalars --test overflow` | ●○○ – ●●○ |
| Bits | `is_even`, `set_bit`, `clear_bit`, `is_bit_set`, `pack_rgb`, `red`, ★`is_power_of_two` | `cargo test -p ch03-scalars --test bits` | ●○○ – ●●● |
| Floats | `celsius_to_fahrenheit`, `approx_eq`, ★`is_nan_manual` | `cargo test -p ch03-scalars --test floats` | ●○○ – ●●○ |
| Chars | `digit_value`, `ascii_to_upper`, `utf8_len` | `cargo test -p ch03-scalars --test chars` | ●○○ – ●●○ |

Notes:
- **Read each doc comment carefully.** Several contain a trap (a sum that doesn't fit, a
  subtraction that goes below zero, a negation with no positive counterpart). The tests
  include exactly those cases. A naive solution panics in debug mode, which is the lesson
  working.
- For `average_u8`: there are at least two good solutions, one with casts and one with only
  bit operations (★ harder). Find one and then look for the other.
- For the bit functions: draw the bits on paper. Build the mask first (which bits do I want?),
  then decide which operator applies it.
- `abs_difference` and `python_floor_div` are easiest with the right std method. Go looking in
  the docs. (Can you also do `abs_difference` with casts?)
- When all tests pass, run `cargo clippy -p ch03-scalars` and address what it says.

---

## Compiler errors you'll likely meet

| Error | Meaning |
|---|---|
| E0384 `cannot assign twice to immutable variable` | You forgot `mut`. |
| E0308 `mismatched types` | E.g. `u32` where `u64` is expected. Rust never converts implicitly, so use `as`. Also appears if you end a function with `;`. |
| E0277 `cannot add 'i64' to 'i32'` | Both sides of an arithmetic operator must have the same type. |
| `this arithmetic operation will overflow` | The compiler proved an overflow at compile time. |
| `literal out of range for 'u8'` | E.g. `let x: u8 = 256;`. |
| panic: `attempt to add with overflow` (at runtime, debug) | A real overflow happened. Use a wider type or an explicit `wrapping_`/`saturating_`/`checked_` method. |
| panic: `attempt to subtract with overflow` | Usually an unsigned value going below zero. |

---

## nvim tip

Put the cursor on `u8` in any stub and press `K`. The hover shows the docs. Then run `gd`
on a method like `wrapping_add` inside a demo, and you land in the standard library's source.
Many integer methods are generated by a macro for all integer types at once. Seeing that
answers "why is `u8` documented exactly like `u64`?"

---

## Checkpoint

1. Why is a Rust `u8` 1 byte while a Python `int` holding 1 is 28 bytes? What does Python gain
   by paying that price?
2. What bit pattern is `-2i8`? Why does two's complement let the CPU use one add instruction
   for both signed and unsigned values?
3. Your code overflows. What happens in a debug build, and in a release build? Which method
   would you use for a hash computation, and which for image brightness?
4. What does `300i32 as u8` give, and why? What about `-1.7 as u8`?
5. Why is comparing floats with `==` dangerous? What's special about NaN?
6. How many bytes is a `char`? How many bytes does `'é'` take in a UTF-8 file?

**Further reading:**
- The Rust Book, [ch. 3.1–3.3: Variables, data types, functions](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
- [The Floating-Point Guide](https://floating-point-gui.de/): short and practical
- std docs: the [`u32`](https://doc.rust-lang.org/std/primitive.u32.html),
  [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) and
  [`char`](https://doc.rust-lang.org/std/primitive.char.html) primitive pages. Skim the
  method lists.

---

## Done?

- `cargo test -p ch03-scalars` is all green, and `cargo clippy -p ch03-scalars` has nothing
  left to say.
- Add a row to `notes/progress.md`. (Maybe start a `notes/ch03.md` with a Python → Rust
  table.)
- Commit: `git add -A && git commit -m "ch03 done" && git push`
- Next: [Chapter 4: Functions & control flow](../ch04-control-flow/README.md)
