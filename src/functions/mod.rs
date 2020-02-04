pub mod functions {
    use super::super::algorithms;
    use algorithms::algorithms::{Huffman, Node};
    pub fn read_from_file(url: &str) -> String {
        use std::fs;
        fs::read_to_string(url).unwrap()
    }
    pub fn get_unique_chars(text: &str) -> Vec<Node<char>> {
        use std::collections::HashMap;
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut nodes: Vec<Node<char>> = vec![];
        for (i, c) in text.chars().enumerate() {
            match &map.get(&c) {
                Some(&val) => nodes[val].update_freq(1),
                None => {
                    map.insert(c, nodes.len());
                    nodes.push(Node::new(c, 1));
                }
                _ => println!("HELLO"),
            }
        }
        nodes
    }
    pub fn sort_nodes<'a, 'b, 'c>(
        list: &'c mut Vec<Node<'a, 'b, char>>,
    ) -> &'c mut Vec<Node<'a, 'b, char>> {
        use std::cmp::Ordering;
        list.sort_by(|a, b| a.freq.cmp(&b.freq));
        list
    }
}
