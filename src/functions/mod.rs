pub mod functions {
    use super::super::algorithms;
    use algorithms::algorithms::{Node, ValueTypes};
    use std::fs;
    pub fn read_from_file(url: &str) -> String {
        fs::read_to_string(url).unwrap()
    }
    pub fn write_to_file(path: &str, content: &str) {
        use std::io::Write;
        let file_path = std::path::Path::new(path);
        let mut file = fs::File::create(file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
    pub fn get_unique_chars(text: &str) -> Vec<Box<Node>> {
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
        assert_eq!(sort_nodes(&mut test_vector)[0].get_freq(), 15);
        assert_eq!(sort_nodes(&mut test_vector)[3].get_freq(), 2);
    }
    pub fn build_huffman(list: &mut Vec<Box<Node>>) -> Box<Node> {
        if list.len() > 1 {
            let sum = list[list.len() - 1].get_freq() + list[list.len() - 2].get_freq();
            let mut new_root = Node::new(ValueTypes::Number(sum), Some(sum));
            let a = list.pop();
            let b = list.pop();
            new_root.right = a;
            new_root.left = b;
            list.push(Box::new(new_root));
            let list = sort_nodes(list);
            build_huffman(list)
        } else {
            list.pop().unwrap()
        }
    }
    use std::collections::HashMap;
    pub fn get_map(node: &Box<Node>, map: &mut HashMap<char, String>, bin_string: String) {
        if let ValueTypes::Character(value) = node.value {
            map.insert(value, bin_string);
        } else {
            if let Some(ref left) = node.left {
                get_map(left, map, bin_string.clone() + "0");
            }
            if let Some(ref right) = node.right {
                get_map(right, map, bin_string.clone() + "1");
            }
        }
    }
    pub fn encode(characters: &str, map: &HashMap<char, String>) -> String {
        let mut encoded = String::new();
        for val in characters.chars() {
            let v = map.get(&val);
            encoded.push_str(v.unwrap());
        }
        encoded
    }
    pub fn decode(value: &str, root: &Box<Node>) -> String {
        let mut decoded = String::new();
        let mut root_ptr = root;
        for x in value.chars() {
            if x == '0' {
                if let Some(ref l) = root_ptr.left {
                    root_ptr = l;
                }
            } else {
                if let Some(ref r) = root_ptr.right {
                    root_ptr = r;
                }
            }
            if let ValueTypes::Character(ch) = root_ptr.value {
                decoded.push(ch);
                root_ptr = root;
            }
        }
        decoded
    }
}
