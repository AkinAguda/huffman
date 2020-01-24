pub mod algorithms {
    pub struct Node<'a, 'b, T> {
        pub value: T,
        pub key: i64,
        pub right: Option<&'a Node<'a, 'b, T>>,
        pub left: Option<&'b Node<'b, 'b, T>>,
    }
    impl<'a, 'b, T> Node<'a, 'b, T> {
        pub fn new(value: T, key: i64) -> Node<'a, 'b, T> {
            Node {
                value,
                key,
                right: None,
                left: None,
            }
        }
    }
}
