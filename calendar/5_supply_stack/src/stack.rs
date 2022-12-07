use std::vec::IntoIter;

use crate::Crate;

/// Stack (LIFO) data structure containing Crate objects.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct CrateStack(Vec<Crate>);

impl CrateStack {
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

impl<const N: usize> From<[char; N]> for CrateStack {
    fn from(s: [char; N]) -> Self {
        Self(Vec::from_iter(s.iter().map(|c| Crate(*c))))
    }
}

impl IntoIterator for CrateStack {
    type Item = Crate;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
