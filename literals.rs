/*
 * This program shows some of the ways of how we represent literals in Rust
 */

fn main() {
    // Type annotated variables
    let do_you_rock: bool = true;

    // Regular annotation
    let pi: f32 = 3.1415;

    // Suffix annotation
    let lucky_number = 7i32;

    // Defaults
    let golden_ratio = 1.6180; //f64

    let zilch = 0; //i32

     println!("do_you_rock: {}", do_you_rock);
     println!("pi : {}", pi);
     println!("lucky_number: {}", lucky_number);
     println!("golden_ratio: {}", golden_ratio);
     println!("nada - zilch: {}", zilch);


    // Integer addition
    println!("1 + 7 = {}", 1u32 + 7);

    // Integer substraction
    println!("1 - 7 = {}", 1i32 - 7);

    // Boolean logic short circuiting
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("Not true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is : {:04b}", 0b0011i32 & 0b0101);
    println!("0011 OR 0101 is : {:04b}", 0b0011i32 | 0b0101);
    println!("0011 XOR 0101 is : {:04b}", 0b0011i32 ^ 0b0101);
    println!("1 << 4 is {}", 1u32 << 4);
    println!("0x80 >> 2 is 0x{:x}", 0x80i32 >> 2);

    // Cool way to write millions to read better
    println!("One million is : {}", 1_000_000u32);
}