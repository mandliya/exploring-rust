/*
    Simple and block expressions in Rust.
*/

fn main() {
    let a = 4u32;
    let b = 2u32;

    let a_plus_b_squared = {
        let a_squared = a * a;
        let b_squared = b * b;

        // This expression is assigned to a_plus_b_squared
        // notice absence of semi-colon here.
        a_squared + (2 * a * b) + b_squared
    };

    let c = {
        // Here we have a semi-colon, so c would be ()
        // semicolon will suppress the expression
        2 * a;
    };

    println!("a_plus_b_squared: {:?}", a_plus_b_squared);
    println!("c is {:?}", c);
}