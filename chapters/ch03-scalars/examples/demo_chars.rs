// cargo run -p ch03-scalars --example demo_chars

use std::mem::size_of;

fn main() {
    println!("size_of::<char>() = {} bytes", size_of::<char>());
    println!("size_of::<u8>()   = {} byte", size_of::<u8>());
    println!("size_of::<bool>() = {} byte", size_of::<bool>());

    println!();
    println!("A char is a Unicode scalar value, i.e. a code point:");
    for c in ['A', 'a', '0', 'é', '€', '🦀'] {
        println!("{c}  U+{:04X}  decimal {:>6}", c as u32, c as u32);
    }

    println!();
    println!("ASCII: letters and digits are consecutive:");
    println!(
        "'a' as u32 = {}, 'b' as u32 = {}, 'z' as u32 = {}",
        'a' as u32, 'b' as u32, 'z' as u32
    );
    println!("'0' as u32 = {}, '9' as u32 = {}", '0' as u32, '9' as u32);
    println!("b'a' = {} (a byte literal is already a u8)", b'a');
    println!("65u8 as char = {}", 65u8 as char);

    println!();
    println!("b'A' = {:08b}", b'A');
    println!("b'a' = {:08b}", b'a');
    println!("(spot the one bit that differs)");
}
