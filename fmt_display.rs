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

#[derive(Debug)]
struct Point(i32, i32);

// fmt::Display implementation for Point
//
impl fmt::Display for Point {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x, y) ==> ({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}

// fmt::Display for complex
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

#[derive(Debug)]
struct List(Vec<i32>);

// fmt::Display implementation which displays list in [index:value] format.
//
impl fmt::Display for List {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{} : {}", count, v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let record = Record {name: "John Doe".to_string(), age: 32};
    let point = Point(0, 4);
    let complex = Complex {real : 3.3, imag: 7.2};
    let list = List(vec![2, 4, 6]);
    println!("fmt::Display for Record : {}", record);
    println!("fmt::Debug for Record: {:?}", record);
    println!("fmt::Display for Point: {}", point);
    println!("fmt::Debug for Point: {:?}", point);
    println!("fmt::Display for Complex: {}", complex);
    println!("fmt::Debug for Debug: {:?}", complex);
    println!("fmt::Display for List: {}", list);
    println!("fmt::Debug for List: {:?}", list);
}