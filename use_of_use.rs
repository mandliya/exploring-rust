// Hiding unused code for now.
#![allow(dead_code)]

enum IcecreamTypes {
    Cone,
    Cup,
}

enum Flavors {
    Vanilla,
    Chocolate,
    StrawBerry,
}


fn main() {
    use IcecreamTypes::{Cone, Cup};
    use Flavors::*;

    let icecream_type = Cone;
    let flavor = Vanilla;

    match icecream_type {
        Cone => println!("I like ice-cream in cone."),
        Cup  => println!("Ice-cream in a cup is lame!"),
    }

    match flavor {
        Vanilla => println!("I don't like vanilla!"),
        Chocolate => println!("I want a chocolate ice-cream right now."),
        StrawBerry => println!("Some people love their strawberry ice-cream, I don't!"),
    }
}