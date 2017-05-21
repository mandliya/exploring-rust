// Structs in Rust

// A unit struct
#[derive(Debug)]
struct Nil;

// A tuple struct
#[derive(Debug)]
struct Coordinates {
    x : f32,
    y : f32,
}

// Structs types can be used in structs

#[derive(Debug)]
struct Location {
    coordinates : Coordinates,
    sea_level    : f32,
}

fn main() {
    let origin : Coordinates = Coordinates { x: 3.5, y: 9.2 };
    println!("Coordinates are : ({}, {})", origin.x, origin.y);

    // Destructure the point using a `let` binding
    let Coordinates{ x:_x, y:_y } = origin;

    println!("Destructured values: ({}, {})", _x, _y);

    let start_location = Location {
        coordinates : Coordinates { x: _x, y:_y },
        sea_level : 98.2,
    };

    println!("Print location {:?}", start_location);

    println!("Nil Struct: {:?}", Nil);
}


