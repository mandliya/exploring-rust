use std::mem;

fn print_slice(slice: &[i64]) {
    // printing the slice
    //
    for x in slice.iter() {
        println!("{} ", x);
    }
}

fn main() {

    // Declaring an array! the type info is superflous in this case.
    let arr1: [i64; 5] = [2, 4, 6, 8, 10];

    // Initializing entire array with a value (7 here)
    let arr2: [i32; 500] = [7; 500];

    // Print array 1
    println!("arr1: {:?}", arr1);

    // Accessing the element
    println!("Third element of arr1 i.e. arr1[2] : {}", arr1[2]);

    // Length of arr2
    println!("Length of arr2  is : {}", arr2.len());

    // Memory used on stack by array1
    println!("Memory in bytes used on stack by arr1: {}", mem::size_of_val(&arr1));

    // Borrow array as slices
    println!("Borrow the whole array as slice:");
    print_slice(&arr1);

    // Borrowing just a part of array as slice
    println!("Borrow the par of array as slice (index 2-4)");
    print_slice(&arr1[2 .. 4]);

}