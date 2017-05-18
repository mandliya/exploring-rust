
// reverse a tuple of int and bool
//
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

fn main() {
    let pair = (42, true);
    println!("Pair is {:?}", pair);
    println!("Reveresed pair is {:?}", reverse(pair));
}