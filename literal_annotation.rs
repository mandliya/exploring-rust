/*
 * Suffixed and Unsuffixed literals and their type annotations in Rust.
 */

 fn main() {
     
     // numeric literals can be suffixed by type, so their types are known at initialization.
     let an_uint8 = 2u8;
     let an_int32 = 2i32;
     let a_float32 = 4.0f32;

     // Un-suffixed numeric literals type annotation is done based on how they are used.
     // If no constraint exists, the compiler will use i32 for integers, and f64 for
     // floating-point numbers.

     let x = 1;
     let f = 4.0;

     // Printing size of each variable in bytes.
     println!("Size of `an_uint8`: {}", std::mem::size_of_val(&an_uint8));
     println!("Size of `an_int32`: {}", std::mem::size_of_val(&an_int32));
     println!("Size of `a_float32`: {}", std::mem::size_of_val(&a_float32));
     println!("Size of `x`: {}", std::mem::size_of_val(&x));
     println!("Size of `f`: {}", std::mem::size_of_val(&f));
 }