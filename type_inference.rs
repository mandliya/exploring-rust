/*
 * A simple example of showing Rust's intelligent type inference
 */

 fn main() {
     // Here we annotated elem as f32, therefore compiler would know type of elem.
     //
     let elem = 4.5f32;

     // an empty vector, however we don't yet know what kind of elements it carry.
     // Therefore, compiler just knows `vec` is a vector of something. 
     let mut vec = Vec::new();

     // push elem to vector
     vec.push(elem);

     // Now compiler knows vec is vector of f32s (Vec<f32>)

     println!("{:?}", vec);
 }