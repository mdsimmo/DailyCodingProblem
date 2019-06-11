extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Node<T> {
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl <T> Node <T> {
    fn new(val: T, left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Node<T> {
        Node { val, left, right }
    }
}

fn main() {

    //let a = Node::new("A", None, None);
    //let b = Node::new("B", Some(Box::new(a)), None);

    let a = Node::new("A", None, Some(Box::new(Node::new("b", None, None))));
    println!("{:?}", a);

    let serialized = serde_json::to_string(&a).unwrap();
    println!("{}", serialized);

    let deserialized:Node<String> = serde_json::from_str(&serialized).unwrap();
    println!("{:?}", deserialized);
}
