pub mod algorithms {
    #[derive(Debug)]
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
        pub fn get_value(&self) -> i64 {
            match self.value {
                ValueTypes::Number(value) => value,
                _ => panic!("Value not set"),
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
