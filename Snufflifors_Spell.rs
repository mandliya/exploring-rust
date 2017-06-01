// Variable bindings are immuatable by default in Rust.
// However they can be overridden using the `mut` modifier.

// The Snufflifors Spell (Snufflifors) is a spell used to transfigure objects into mice, 
// but Flying books are especially vulnerable to this spell.
// However, in Rust world, it does not apply to immutable objects.

 // snufflifors spell converts an object to mice!!!
 //
fn cast_snufflifors_spell(object: &mut String) {
    object.clear();
    object.push_str("Mice");
}

fn main() {
    let mut the_book = String::from("The Tales of Beedle the Bard");
    let the_special_book = String::from("Warlock at War");

    // book before casting snufflifors spell
    println!("Book before casting snufflifors spell: {}", the_book);

    // casting the spell
    cast_snufflifors_spell(&mut the_book);

    // after spell
    println!("Book after casting snufflifors spell {}", the_book);

    // the_special_book can't be changed, as it is not mutable.
    // you will get an error in Rust world!
    println!("The special book is {}, it can't be casted to mice since it is immutable",
        the_special_book);
    
    //cast_snufflifors_spell(the_special_book);
    // println!("The special book after casting snufflifors spell {}", the_special_book);
}   