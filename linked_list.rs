use List::*;

enum List {
    // A tuple struct that represents an element and pointer to next element
    Node(u32, Box<List>),
    // A node which specifies end of the list.
    Nil,
}

impl List {

    // Create an empty list
    fn new() -> List {
        Nil
    }

    // Add an element at the start of the list.
    fn prepend(self, elem: u32) -> List {
        Node(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // This function can take two types of self.
        // self has type &List or *self has type List
        match *self {
            // Take a reference to the self, as self is borrowed.
            Node(_, ref tail) => 1 + tail.len(),
            // Base case: Empty list
            Nil => 0
        } 
    }

    // Return representation of list as heap allocated string
    fn stringify(&self) -> String {
        match *self {
            Node(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Append some elements
    list = list.prepend(2);
    list = list.prepend(4);
    list = list.prepend(6);

    // Show the final state of the list
    println!("Linked list has length: {}", list.len());
    println!("{}", list.stringify());
}


