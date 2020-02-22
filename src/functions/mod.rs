pub mod functions {
    use super::super::algorithms;
    use algorithms::algorithms::{Node, ValueTypes};
    pub fn read_from_file(url: &str) -> String {
        use std::fs;
        fs::read_to_string(url).unwrap()
    }
    pub fn get_unique_chars(text: &str) -> Vec<Node> {
        use std::collections::HashMap;
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut nodes: Vec<Node> = vec![];
        for (_, c) in text.chars().enumerate() {
            match &map.get(&c) {
                Some(&val) => nodes[val].update_freq(1),
                None => {
                    map.insert(c, nodes.len());
                    nodes.push(Node::new(ValueTypes::Character(c), Some(1)));
                }
            }
        }
        nodes
    }
    pub fn sort_nodes<'a, 'b, 'c>(list: &'c mut Vec<Node<'a, 'b>>) -> &'c mut Vec<Node<'a, 'b>> {
        list.sort_by(|a, b| a.freq.cmp(&b.freq));
        list
    }
    // pub fn build_huffman(list: &Vec<Node>) {
    //     let mut huffman = Node::new(list[0].freq.unwrap() + list[1].freq.unwrap(), None);
    //     let mut currentNode = &huffman;
    //     huffman.right = Some(&list[0]);
    //     huffman.left = Some(&list[1]);
    //     for index in 2..list.len() {
    //         if (list[index].freq.unwrap() > huffman.freq.unwrap()) {
    //             let newNode = Node::new(list[index].freq.unwrap() + huffman.freq.unwrap(), None);
    //             newNode.left = Some(&list[index]);
    //             newNode.right = Some(currentNode);
    //             let huffman = &newNode;
    //         }
    //     }
    // }
}
