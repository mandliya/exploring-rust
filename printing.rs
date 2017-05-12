fn main() {
    println!("This program illustrates how printing in Rust works with quotes from
    \"The Hitchhiker's Guide to the Galaxy\" series.");

    println!();
    
    // A.1 Printing with place holders.
    //
    // Place holder in Rust is `{}`. It will be replaced by arguments to println!
    // It will stringify the arguments. 42 here is converted to string.
    // For example.
    println!("The answer to life the universe and everything is {}", 42);

    // One argument per place holder.
    //
    println!("I refuse to {} that question on the grounds that I don't know the {}.", "answer", "answer");

    // Placeholders can be used with positional arguments.
    //
    println!("You {1} and {0}. At any rate, you {1}.", "learn", "live");

    // Named arguments.
    println!("{part1} {part2}",
            part1="Time is an illusion.",
            part2="Lunchtime doubly so.");
    
    // Right-align text with a specified width. This will output
    // 5 whitepaces and then number 42, total width is 6.
    println!("Again I repeat, the answer to life the universe and everything is: {number:>width$}", number=42, width=6);
    
    // Adding 0s to answer to everything.
    println!("Padding answer to everything with 0s : {number:>0width$}", number=42, width=6);

    // Finally some encouraging words!
    let wise_words = "don't panic!!!";
    println!("While learning rust!, as Adam Douglas would have said: {}", wise_words.to_uppercase());

    // Rust by example activities
    // Add a println! macro that prints: Pi is roughly 3.142 by controlling the number of decimal places shown.
    // For the purposes of this exercise, use let pi = 3.141592 as an estimate for Pi.
    let pi = 3.141592;
    println!("Pi is roughly {:.*}", 3, pi);
}