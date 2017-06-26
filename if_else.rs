/*
    Example of simple flow control in Rust with "if/else"
 */

 fn main() {
    let x = 4;

    if x > 0 {
        print!("{} is positive.", x);
    } else if x < 0 {
        print!("{} is negative.", x);
    } else {
        print!("{} is zero.", x);
    }

    let balance_number = 
        if x < 10 && x > -10 {
            println!(", and a small number, increase 10 fold.");
            
            // expression here, notice absence of ;
            10 * x
        } else {
            println!(", and is a large number, reduce by 2.");
            x / 2
        };

    println!("{} -> {}", x, balance_number);
 }