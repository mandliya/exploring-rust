// Structs in Rust

// A unit struct
#[derive(Debug)]
struct Nil;

// A struct with two fields
#[derive(Debug)]
struct Coordinates {
    x : i32,
    y : i32,
}

// A tuple struct
struct Pair(i32, i32);

// Structs types can be used in structs

#[derive(Debug)]
struct Location {
    coordinates : Coordinates,
    sea_level    : f32,
}


// Activity 3.1 : Add a function rect_area which calculates the area of a 
// rectangle (try using nested destructuring).
struct Rectangle {
    top_left: Coordinates,
    bottom_right: Coordinates,
}

fn rect_area(rect: &Rectangle) -> i32 {
    let Rectangle { top_left: Coordinates{ x:x1, y:y1 },
                    bottom_right: Coordinates{ x:x2, y:y2 } } = *rect;
    let len = x2 - x1;
    let wid = y1 - y2;
    len * wid
}

fn main() {
    let origin = Coordinates { x: 3, y: 9 };
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

    let destination = Pair(450, 980);
    println!("Destination coordinates are: ({}, {})", destination.0, destination.1);

    // Destruct a tuple struct
    let Pair(_x_coordinate, _y_coordinate) = destination;
    println!("Stop before destination at : ({}, {})", _x_coordinate - 400, _y_coordinate - 80);

    // Activity from "Rust by example : 3.1"
    // 
    let rect = Rectangle { top_left: Coordinates{ x: 2, y: 5 },
                           bottom_right: Coordinates { x: 7, y: 3} };
    println!("Area of rectangle is: {}", rect_area(&rect));
}


