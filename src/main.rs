mod algorithms;
mod functions;
// use algorithms::algorithms::Node;
use functions::functions::{get_unique_chars, read_from_file, sort_nodes};

fn main() {
    // let me = Node::new("hello", 12);
    let data = read_from_file("./uncompressed/file");
    let mut stru = get_unique_chars(&data);
    let sorted = sort_nodes(&mut stru);
    for node in sorted {
        println!("{:?} {:?}", node.value, node.freq);
    }
}
