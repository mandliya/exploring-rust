/*
 * This program displays the use of fmt::debug trait.
 */

// Derive fmt::Debug implementation for Record, which contains a name-string
// and id integer.
//
#[derive(Debug)]
struct Record {
    name: String,
    age: i32,
}

#[derive(Debug)]
struct MainRecord(Record);

fn main() {
    let record = Record{name: "John Doe".to_string(), age: 32};
    println!("Employee Record: {:?}", record);

    // All std library types are automatically printable with {:?}
    //
    println!("The answer to life, universe and everything is {:?}.", 42);

    // Debug doesn't give us control on how the results would look.
    // We would have wanted the below line to show similar output as above.
    //
    println!("Main employee record: {:?}", MainRecord(record));
}




