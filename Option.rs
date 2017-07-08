/* This program demonstrates Rust's special enum Option<T> to catch Null dereferencing errors */

fn checked_division(dividend : i32, divisor : i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn try_division(dividend: i32, divisor: i32) {
    match checked_division(dividend, divisor) {
        None => println!("{} / {} divisoin failed!", dividend, divisor),
        Some(quotient) => println!("{} / {} = {}", dividend, divisor, quotient),
    }
}

fn main() {
    let x = 9;
    let y = 3;
    try_division(x, y);
    let y = 0;
    try_division(x, y);
}
