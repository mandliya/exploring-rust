
// reverse a tuple of int and bool
//
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let pair = (42, true);
    println!("Pair is {:?}", pair);
    println!("Reveresed pair is {:?}", reverse(pair));

    let triplet = (3u8, 4u8, 5u8);
    println!("triplets's first value: {}", triplet.0);
    println!("triplets's second value: {}", triplet.1);

    let tuple_of_tuples = ((1u32, -4i32), (-5.3f32, -11.4f32));
    println!("Tuple of tuples: {:?}", tuple_of_tuples);
    println!("Tuple of tuples[0]: {:?}", tuple_of_tuples.0);
    println!("Tuple of tuples[0][1]: {:?}", (tuple_of_tuples.0).1);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix Debug: {:?}", matrix);
}
