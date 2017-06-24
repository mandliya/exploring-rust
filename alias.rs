/**
 * Alias in Rust.
 */

 use std::f32::consts::PI;

 type Length =u64;
 type Breadth= u64;
 type Radius = f32;

fn main() {
    let len:Length = 10u64;
    let wid:Breadth = 20u64;
    let rad:Radius = 10f32;
    println!("Area of rectangle is {}", len * wid);
    println!("Area of circle is {}", PI * (rad * rad));
}