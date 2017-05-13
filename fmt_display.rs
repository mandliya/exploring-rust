/*
 * As we saw in fmt::Debug, it is not very customizable and clean.
 * In order to customize output at will, it is done by manually implementing 
 * fmt::Display trait which uses {} marker.
 */

// import the fmt module.
// 
use std::fmt;

// We will implement fmt::Display for the structure Record
//
#[derive(Debug)] 
struct Record {
    name: String,
    age: i32,
}

// In order to use {} marker for Record, we will implement fmt::Display for it.
//
impl fmt::Display for Record {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Employee record: Name: {} Age: {}", self.name, self.age)
    }
}


fn main() {
    let record = Record {name: "John Doe".to_string(), age: 32};
    println!("Display: {}", record);
    println!("Debug: {:?}", record);
}