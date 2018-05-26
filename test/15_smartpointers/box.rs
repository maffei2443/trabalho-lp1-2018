#[allow(dead_code)]
use std::boxed::Box;
fn main() {
    let b = std::boxed::Box::new(5);
    let r = Node{l: Box::new(Node::new()), r: Box::new(Node::new())};
    println!("Caixa contendo: {}", b);
}
