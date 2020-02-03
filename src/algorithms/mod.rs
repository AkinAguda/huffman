pub mod algorithms {
    use std::fmt::Display;
    pub struct Node<'a, 'b, T> {
        pub value: T,
        pub freq: i64,
        pub right: Option<&'a Node<'a, 'b, T>>,
        pub left: Option<&'b Node<'b, 'b, T>>,
    }
    pub struct Huffman<'a, 'b, T> {
        pub root: &'a mut Node<'a, 'b, T>,
    }
    impl<'a, 'b, T> Node<'a, 'b, T> {
        pub fn new(value: T, freq: i64) -> Node<'a, 'b, T> {
            Node {
                value,
                freq,
                right: None,
                left: None,
            }
        }
        pub fn update_freq(&mut self, value: i64) {
            self.freq += value;
        }
    }
}
