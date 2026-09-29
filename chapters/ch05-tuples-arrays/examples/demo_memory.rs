// Tuples and arrays in memory: sizes, alignment, padding and addresses.
//   cargo run -p ch05-tuples-arrays --example demo_memory

use std::mem::{align_of, offset_of, size_of};

// Clippy would rather we loop over the elements than over the indices, but here we want the
// index loops: they print exactly the expressions we're asking about.
#[allow(clippy::needless_range_loop)]
fn main() {
    // 1. An array is its elements, back to back. Nothing else: no header, no length.
    println!("== arrays: size = length × element size");
    println!("i32            {:>5} bytes", size_of::<i32>());
    println!("[i32; 5]       {:>5} bytes", size_of::<[i32; 5]>());
    println!("[u8; 1000]     {:>5} bytes", size_of::<[u8; 1000]>());
    println!("[[f64; 3]; 3]  {:>5} bytes", size_of::<[[f64; 3]; 3]>());
    println!("[bool; 0]      {:>5} bytes", size_of::<[bool; 0]>());

    // 2. The elements sit at consecutive addresses. `&x` is "the address of x" and {:p}
    //    prints it in hex. (References are chapter 8, addresses in general chapter 6.)
    println!();
    println!("== addresses of the elements of a [i32; 4]");
    let numbers = [10, 20, 30, 40];
    for i in 0..numbers.len() {
        println!("&numbers[{i}] = {:p}", &numbers[i]);
    }

    // A grid is rows back to back: row 0, then row 1 ("row-major", NumPy's C order).
    println!();
    println!("== addresses in a [[u8; 3]; 2]");
    let grid = [[1u8, 2, 3], [4, 5, 6]];
    for row in 0..2 {
        for col in 0..3 {
            println!("&grid[{row}][{col}] = {:p}", &grid[row][col]);
        }
    }

    // 3. Tuples: every type has an alignment, and a value's address must be a multiple of it.
    println!();
    println!("== alignment");
    println!("u8  align {}", align_of::<u8>());
    println!("u16 align {}", align_of::<u16>());
    println!("u32 align {}", align_of::<u32>());
    println!("u64 align {}", align_of::<u64>());

    // To keep the u32 aligned, the compiler adds unused bytes: padding.
    println!();
    println!("== tuple sizes and field offsets (in bytes from the start)");
    println!(
        "(u8, u32)      size {:>2}  .0 at {}, .1 at {}",
        size_of::<(u8, u32)>(),
        offset_of!((u8, u32), 0),
        offset_of!((u8, u32), 1),
    );
    // Rust may reorder the fields to waste less space.
    println!(
        "(u8, u32, u8)  size {:>2}  .0 at {}, .1 at {}, .2 at {}",
        size_of::<(u8, u32, u8)>(),
        offset_of!((u8, u32, u8), 0),
        offset_of!((u8, u32, u8), 1),
        offset_of!((u8, u32, u8), 2),
    );
    // The size is always a multiple of the alignment, so that in an array, every element
    // starts at an aligned address.
    println!(
        "(u64, u8)      size {:>2}  .0 at {}, .1 at {}   (align {})",
        size_of::<(u64, u8)>(),
        offset_of!((u64, u8), 0),
        offset_of!((u64, u8), 1),
        align_of::<(u64, u8)>(),
    );
    println!("[(u64, u8); 4] size {:>2}", size_of::<[(u64, u8); 4]>());
    println!("()             size {:>2}", size_of::<()>());

    // 4. Local variables live in the function's stack frame, close together.
    println!();
    println!("== where main's locals live");
    let small = 5u64;
    println!("&small   = {:p}", &small);
    println!("&numbers = {:p}", &numbers);
    println!("&grid    = {:p}", &grid);
}
