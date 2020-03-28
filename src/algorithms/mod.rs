pub mod algorithms {
    #[derive(Debug, Clone, Copy)]
    pub enum ValueTypes {
        Character(char),
        Number(i64),
    }
    #[derive(Debug)]
    pub struct Node {
        pub value: ValueTypes,
        pub freq: Option<i64>,
        pub right: Option<Box<Node>>,
        pub left: Option<Box<Node>>,
    }

    impl Node {
        pub fn new(value: ValueTypes, freq: Option<i64>) -> Node {
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
    }
}
