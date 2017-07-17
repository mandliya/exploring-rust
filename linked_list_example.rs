use std::fmt;

struct Node {
    value: u32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val : u32) -> Node {
        Node {
            value : val,
            next: None,
        }
    }

    fn append(&mut self, val: u32) {
        match self.next {
            None => self.next = Some(Box::new(Node::new(val))),
            Some(ref mut node) => node.append(val),
        }
    }

    fn length(&self) -> u32 {
        match self.next {
            None => 1,
            Some(ref node) => node.length() + 1,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match self.next {
            None => write!(f, "{}.", self.value),
            Some(ref node) => write!(f, "{} -> {}", self.value, node),
        }
    }
}


fn main() {
    let mut root = Node::new(1);
    root.append(2);
    root.append(3);
    println!("List: {}", root);
    println!("Length of list: {}", root.length());
}