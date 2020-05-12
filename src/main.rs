mod ds;
mod functions;
use functions::functions::{
    build_huffman, decode, encode, get_map, get_unique_chars, read_from_file, sort_nodes,
    write_to_file,
};
use std::collections::HashMap;

fn main() {
    let data = read_from_file("./uncompressed/file");
    let mut stru = get_unique_chars(&data);
    let sorted = sort_nodes(&mut stru);
    let huffman = build_huffman(sorted);
    let mut map: HashMap<char, String> = HashMap::new();
    get_map(&huffman, &mut map, "".to_string());
    let encoded = encode(&data, &map);
    write_to_file("./compressed/file", &encoded);
    let decoded = decode(&encoded, &huffman);
    println!("{:?}", decoded);
}
