// cargo run -p ch03-scalars --example demo_integers

use std::mem::size_of;

fn main() {
    println!("== Sizes (bytes) and ranges ==");
    println!(
        "u8    {} byte   {} ..= {}",
        size_of::<u8>(),
        u8::MIN,
        u8::MAX
    );
    println!(
        "i8    {} byte   {} ..= {}",
        size_of::<i8>(),
        i8::MIN,
        i8::MAX
    );
    println!(
        "u16   {} bytes  {} ..= {}",
        size_of::<u16>(),
        u16::MIN,
        u16::MAX
    );
    println!(
        "i32   {} bytes  {} ..= {}",
        size_of::<i32>(),
        i32::MIN,
        i32::MAX
    );
    println!(
        "u64   {} bytes  {} ..= {}",
        size_of::<u64>(),
        u64::MIN,
        u64::MAX
    );
    println!(
        "i128  {} bytes {} ..= {}",
        size_of::<i128>(),
        i128::MIN,
        i128::MAX
    );
    println!(
        "usize {} bytes  (pointer-sized: this is a 64-bit machine)",
        size_of::<usize>()
    );

    println!();
    println!("== Literals: all of these are the number 255 ==");
    let decimal = 255;
    let with_underscores = 2_55; // underscores are ignored, useful for 1_000_000
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;
    let typed = 255u8; // a type suffix
    println!("{decimal} {with_underscores} {hex} {octal} {binary} {typed}");
    println!("byte literal b'A' = {} (a u8)", b'A');

    println!();
    println!("== The same number, formatted differently ==");
    let n: u8 = 202;
    println!("{{}}      -> {n}");
    println!("{{:b}}    -> {n:b}");
    println!("{{:08b}}  -> {n:08b}   (pad with zeros to 8 digits)");
    println!("{{:x}}    -> {n:x}");
    println!("{{:#x}}   -> {n:#x}  (with the 0x prefix)");
    println!("{{:>6}}   -> [{n:>6}] (right-align in 6 chars)");

    println!();
    println!("== Two's complement: how i8 stores negative numbers ==");
    // `x as u8` reinterprets the same 8 bits as unsigned, so we can print them in binary.
    for x in [0i8, 1, 2, 127, -1, -2, -127, -128] {
        println!("{x:>5}  bits: {:08b}", x as u8);
    }

    println!();
    println!("== Bytes in memory: endianness ==");
    let big: u32 = 0x1234_5678;
    println!(
        "0x{big:08x} as little-endian bytes: {:02x?}",
        big.to_le_bytes()
    );
    println!(
        "0x{big:08x} as big-endian bytes:    {:02x?}",
        big.to_be_bytes()
    );
    println!(
        "this CPU stores numbers as:          {:02x?}",
        big.to_ne_bytes()
    );
}
