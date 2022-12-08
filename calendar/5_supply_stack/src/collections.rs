pub mod stack {
    use crate::Crate;
    use std::vec::IntoIter;

    /// Simple Stack (LIFO) data structure
    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Stack(Vec<Crate>);

    impl Stack {
        pub fn new() -> Self {
            Self(Vec::new())
        }

        pub fn pop(&mut self) -> Option<Crate> {
            self.0.pop()
        }

        pub fn push(&mut self, val: Crate) {
            self.0.push(val)
        }
    }

    impl<const N: usize> From<[char; N]> for Stack {
        fn from(s: [char; N]) -> Self {
            Self(Vec::from(s))
        }
    }

    impl IntoIterator for Stack {
        type Item = Crate;
        type IntoIter = IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }
}

pub mod queue {
    use crate::Crate;
    use std::collections::VecDeque;

    /// Simple Queue (FIFO) data structure
    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Queue(VecDeque<Crate>);

    impl Queue {
        pub fn new() -> Self {
            Self(VecDeque::new())
        }

        pub fn pop_front(&mut self) -> Option<Crate> {
            self.0.pop_front()
        }

        pub fn push_back(&mut self, val: Crate) {
            self.0.push_back(val)
        }
    }
}
