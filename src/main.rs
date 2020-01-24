mod algorithms;
use algorithms::algorithms::Node;
fn main() {
    let me = Node::new("hello", 12);
    println!("{}", me.key);
}
