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
    pub fn sort_nodes(list: &mut Vec<Node>) -> &mut Vec<Node> {
        list.sort_by(|a, b| a.freq.cmp(&b.freq));
        list
    }
    pub fn handle_node(node: Option<Box<Node>>, list: &Vec<Node>, iter: u64) -> Box<Node> {
        if iter == 0 {
            let first = Box::new(Node::new(
                ValueTypes::Number(list[0].get_freq() + list[1].get_freq()),
                None,
            ));
            handle_node(Some(first), list, 2)
        } else {
            let ptr = *node.unwrap();
            if ptr.get_int_value() < list[iter as usize].get_freq() {
                let mut new_root = Box::new(Node::new(
                    ValueTypes::Number(list[iter as usize].get_freq() + ptr.get_int_value()),
                    None,
                ));
                new_root.right = Some(Box::new(ptr));
                new_root.left = Some(Box::new(Node::new(
                    ValueTypes::Character(list[iter as usize].get_char_value()),
                    None,
                )));
                if (iter + 1) < list.len() as u64 {
                    handle_node(Some(new_root), list, iter + 1)
                } else {
                    new_root
                }
            } else {
                let mut new_tree = Node::new(
                    ValueTypes::Number(
                        list[iter as usize].get_freq() + list[iter as usize + 1].get_freq(),
                    ),
                    None,
                );
                new_tree.left = Some(Box::new(Node::new(
                    ValueTypes::Character(list[iter as usize].get_char_value()),
                    None,
                )));
                new_tree.right = Some(Box::new(Node::new(
                    ValueTypes::Character(list[iter as usize + 1].get_char_value()),
                    None,
                )));
                let mut new_root = Node::new(
                    ValueTypes::Number(ptr.get_int_value() + new_tree.get_int_value()),
                    None,
                );
                new_root.right = Some(Box::new(ptr));
                new_root.left = Some(Box::new(new_tree));
                if iter + 2 >= (list.len() as u64 - 1) {
                    Box::new(new_root)
                } else {
                    handle_node(Some(Box::new(new_root)), list, iter + 2)
                }
            }
        }
    }
    pub fn build_huffman(list: &mut Vec<Node>) -> Box<Node> {
        handle_node(None, list, 0)
    }
}
