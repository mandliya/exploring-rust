fn main() {
    let mut count = 0u32;
    loop {
        count += 1;
        if count % 2 == 0 {
            println!("even");
            continue;
        }
        println!("{}", count);
        if count > 10 {
            println!("done printing");
            break;
        }
    }
}