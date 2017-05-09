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
}