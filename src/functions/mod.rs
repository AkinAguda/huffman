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
            Box::new(Node::new(
                ValueTypes::Number(list[0].get_freq() + list[1].get_freq()),
                None,
            ))
        } else {
            let ptr = *node.unwrap();
            if ptr.get_value() < list[iter as usize].get_freq() {
                println!("higher");
                let mut new_root = Box::new(Node::new(
                    ValueTypes::Number(
                        list[iter as usize].get_freq() + list[(iter + 1) as usize].get_freq(),
                    ),
                    None,
                ));
                new_root.right = Some(Box::new(ptr));
                new_root
            } else {
                println!("higher");
                let mut new_root = Box::new(Node::new(
                    ValueTypes::Number(
                        list[iter as usize].get_freq() + list[(iter + 1) as usize].get_freq(),
                    ),
                    None,
                ));
                new_root.right = Some(Box::new(ptr));
                new_root
            }
        }
    }
    pub fn build_huffman(list: &mut Vec<Node>) -> Box<Node> {
        let first = handle_node(None, list, 0);
        handle_node(Some(first), list, 2)
    }
}
