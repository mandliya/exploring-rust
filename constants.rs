// globals are declared often in outermost scope.
static BOOK_NAME: &'static str = "The HitchHiker's guide to the galaxy";
const ANSWER_TO_EVERYTHING: i32 = 42;

fn is_this_the_answer(n: i32) -> bool {
    n == ANSWER_TO_EVERYTHING
}

fn main() {
    let n = 21;
    println!("My favorite book is {}", BOOK_NAME);
    println!("{} is {} answer to life, universe and everything.", n, if is_this_the_answer(n) {"the"} else { "not the" });
    println!("The answer to life, universe and everything is {}", ANSWER_TO_EVERYTHING);
}
