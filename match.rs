use std::io;
use std::f32;

fn main() {
    println!("Enter an number: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().parse::<u32>() {
                Ok(number) => match number {
                    number if is_prime(number) => println!("{} is prime.", number),
                    number if is_even(number)  => println!("{} is even.", number),
                    _                => println!("{} is odd.", number),
                },
                Err(error) => println!("Error while parsing input:{}", error),
            }
        },
        Err(error) => println!("Error while reading input: {}", error),
    }
}

fn is_prime(number : u32) -> bool {
    if number == 1 {
        return false;
    }
    if number == 2 {
        return true;
    }
    if number % 2 == 0 {
        return false;
    }
    
    let square_root_num = (number as f32).sqrt() as u32;
    for i in 2..(square_root_num + 1) {
        if number % i == 0 {
            return false;
        }
    }
    true
}

fn is_even(number : u32) -> bool {
    number % 2 == 0
}