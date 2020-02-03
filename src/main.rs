mod algorithms;
mod functions;
// use algorithms::algorithms::Node;
use functions::functions::{get_unique_chars, read_from_file};

fn main() {
    // let me = Node::new("hello", 12);
    let data = read_from_file("./uncompressed/file");
    let stru = get_unique_chars(&data);
    for node in stru {
        println!("{:?} {:?}", node.value, node.freq);
    }
}
