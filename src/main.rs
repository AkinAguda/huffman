mod algorithms;
mod functions;
use algorithms::algorithms::{Node, ValueTypes, ValueTypes::Number};
use functions::functions::{build_huffman, get_unique_chars, read_from_file, sort_nodes};

fn main() {
    let data = read_from_file("./uncompressed/file");
    let mut stru = get_unique_chars(&data);
    let sorted = sort_nodes(&mut stru);
    build_huffman(&sorted);
    // let mut root = Node::new(
    //     ValueTypes::Number(sorted[0].freq.unwrap() + sorted[1].freq.unwrap()),
    //     None,
    // );
    // root.right = Some(&sorted[0]);
    // root.left = Some(&sorted[1]);

    // match root.value {
    //     Number(num) => println!("{:?}", num),
    //     _ => println!("Nothing"),
    // }
    // build_huffman(sorted);
    // println!(
    //     "right node is {} with a frequency of {} and left node is {} with a frequency of {}",
    //     root.right.unwrap().value,
    //     root.right.unwrap().freq.unwrap(),
    //     root.left.unwrap().value,
    //     root.left.unwrap().freq.unwrap(),
    // )
}
