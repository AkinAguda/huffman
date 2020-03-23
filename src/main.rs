mod algorithms;
mod functions;
use algorithms::algorithms::{Node, ValueTypes, ValueTypes::Number};
use functions::functions::{build_huffman, get_unique_chars, read_from_file, sort_nodes};

fn main() {
    let data = read_from_file("./uncompressed/file");
    let mut stru = get_unique_chars(&data);
    let mut sorted = sort_nodes(&mut stru);
    let huffman = build_huffman(sorted);
    println!("{:?}", huffman);
}
