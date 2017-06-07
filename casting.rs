/*
 * This program illustrates use of casting in Rust.
 * Unlike C/C++ there is no implicit casting, however, we can do explicit casting using `as`
 * This example also shows what happens when we do unsigned to signed or signed to unsigned casting
 */

 // Supress warning for overflow casts
 #![allow(overflowing_literals)]

 fn main() {
     let a_decimal = 97.5_f32;

     // The below commented line like code won't work
     // as there is no implicit casting in Rust
     // let an_integer : u8 = a_decimal;

     // An explicit conversion is possible
     let an_integer = a_decimal as u8;
     let a_char = an_integer as char;

     println!("Explicit cast {} --> {} --> {}", a_decimal, an_integer, a_char);

     // when casting any value to an unsigned type, T, 
     // std::T::MAX + 1 is added or subtracted until the value
     // fits into the new type

     // For example 1000 as u8 is == 1000 - 256 - 256 - 256 = 232
     // We keep the first 8 least significant bits (LSB), while 
     // Most Significant Bits are discarded
     //
     println!("1000 as u8 is : {}", 1000 as u8);

     // 1234 already fits u16, so no truncation.
     println!("1234 as u16 is : {}", 1234 as u16);

     // -1 as u8 is (-1 + std::u8::MAX + 1 = -1 + 255 + 1)
     println!("-1 as u8 is : {}", (-1i8) as u8);

    // When casting to a signed type, the (bitwise) result is the same as 
    // first casting to the corresponding unsigned type. If the most significant 
    // bit of that value is 1, then the value is negative.

    // For example
    println!(" 128 as a i8 is : {}", 128 as i8);

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);
    
    // 1000 as u8 -> 232
    println!("1000 as a i8 is : {}", 1000 as i8);
    // and the two's complement of 232 is -24
    println!(" 232 as a i8 is : {}", 232 as i8);
 }


