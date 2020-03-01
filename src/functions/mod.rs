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
    pub fn build_huffman(list: &Vec<Node>) {
        let mut root_node = Node::new(
            ValueTypes::Number(list[0].get_freq() + list[1].get_freq()),
            None,
        );
        for index in 2..list.len() {
            if root_node.get_value() > list[index].get_freq() {
                let mut new_root_node = Node::new(
                    ValueTypes::Number(root_node.get_value() + list[index].get_freq()),
                    Some(0),
                );
                new_root_node.left = Some(&root_node);
            }
        }
    }
}
