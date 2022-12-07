/// A crate of supplies
#[derive(Debug, PartialEq, Eq)]
pub struct Crate(pub(crate) char);

impl Crate {
    pub fn new(val: char) -> Self {
        Self(val)
    }

    pub fn value(&self) -> &char {
        &self.0
    }
}
