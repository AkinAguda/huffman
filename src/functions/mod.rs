pub mod functions {
    use super::super::algorithms;
    use algorithms::algorithms::{Node, ValueTypes};
    pub fn read_from_file(url: &str) -> String {
        use std::fs;
        fs::read_to_string(url).unwrap()
    }
    pub fn get_unique_chars(text: &str) -> Vec<Box<Node>> {
        use std::collections::HashMap;
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut nodes: Vec<Box<Node>> = vec![];
        for (_, c) in text.chars().enumerate() {
            match &map.get(&c) {
                Some(&val) => nodes[val].update_freq(1),
                None => {
                    map.insert(c, nodes.len());
                    nodes.push(Box::new(Node::new(ValueTypes::Character(c), Some(1))));
                }
            }
        }
        nodes
    }
    pub fn sort_nodes(list: &mut Vec<Box<Node>>) -> &mut Vec<Box<Node>> {
        list.sort_by(|a, b| b.freq.cmp(&a.freq));
        list
    }
    #[test]
    fn sort_nodes_works() {
        let mut test_vector = vec![
            Box::new(Node::new(ValueTypes::Character('c'), Some(8))),
            Box::new(Node::new(ValueTypes::Character('d'), Some(3))),
            Box::new(Node::new(ValueTypes::Character('k'), Some(2))),
            Box::new(Node::new(ValueTypes::Number(5), Some(15))),
        ];
        assert_eq!(sort_nodes(&mut test_vector)[3].get_freq(), 2);
    }
    pub fn build_huffman(list: &mut Vec<Box<Node>>) -> &Node {
        if list.len() > 1 {
            let sum = list[0].get_freq() + list[1].get_freq();
            let mut new_root = Node::new(ValueTypes::Number(sum), Some(sum));
            let a = list.pop();
            let b = list.pop();
            new_root.right = a;
            new_root.left = b;
            list.push(Box::new(new_root));
            let list = sort_nodes(list);
            build_huffman(list)
        } else {
            &list[0]
        }
    }
}
