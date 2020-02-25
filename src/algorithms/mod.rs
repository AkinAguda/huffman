pub mod algorithms {
    #[derive(Debug)]
    pub enum ValueTypes {
        Character(char),
        Number(i64),
    }
    pub struct Node<'a, 'b> {
        pub value: ValueTypes,
        pub freq: Option<i64>,
        pub right: Option<&'a Node<'a, 'b>>,
        pub left: Option<&'b Node<'a, 'b>>,
    }
    // pub struct Huffman<'a, 'b, T, M> {
    //     pub root: Option<&'a mut Node<'a, 'b, M, T>>,
    //     pub new_root: Node<'a, 'b, i64, M>,
    // }
    // impl<'a, 'b, T, M> Huffman<'a, 'b, T, M> {
    //     pub fn new(
    //         mut node_one: &'b Node<'a, 'b, M, i64>,
    //         mut node_two: &'b Node<'a, 'b, M, i64>,
    //     ) -> Huffman<'a, 'b, i64, M> {
    //         let mut new_root_instance = Node::new(node_one.freq + node_two.freq, 0);
    //         new_root_instance.left = Some(&node_one);
    //         new_root_instance.right = Some(&node_two);
    //         Huffman {
    //             new_root: new_root_instance,
    //             root: None,
    //         }
    //     }
    // }
    impl<'a, 'b> Node<'a, 'b> {
        pub fn new(value: ValueTypes, freq: Option<i64>) -> Node<'a, 'b> {
            Node {
                value,
                freq,
                right: None,
                left: None,
            }
        }
        pub fn update_freq(&mut self, value: i64) {
            match &self.freq {
                Some(num) => self.freq = Some(num + value),
                None => panic!("Frequency Not Set"),
            }
        }
        pub fn get_freq(&self) -> i64 {
            match self.freq {
                Some(value) => value,
                None => panic!("Frequency Not Set"),
            }
        }
        pub fn update_int_value(&mut self, value: i64) {
            match self.value {
                _ => self.value = ValueTypes::Number(value),
            }
        }
        pub fn update_char_value(&mut self, value: char) {
            match self.value {
                _ => self.value = ValueTypes::Character(value),
            }
        }
    }
}
