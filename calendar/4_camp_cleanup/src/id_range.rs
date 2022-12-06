use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    ops::RangeInclusive,
    str::FromStr,
};

pub type Index = u32;

#[derive(Debug, PartialEq, Eq, Hash)]
/// A custom type wrapped around an inclusive range that can be
/// parsed from strings like `"50-100"`
///
/// # Example
///
/// ```rust
/// use camp_cleanup::IdRange;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!(
///         IdRange::from_str("50-100").unwrap(),
///         IdRange::from(50..=100)
///     )
/// }
/// ```
pub struct IdRange(RangeInclusive<Index>);

impl IdRange {
    pub fn new(start: Index, end: Index) -> Self {
        Self(start..=end)
    }
}

impl From<RangeInclusive<Index>> for IdRange {
    fn from(range: RangeInclusive<Index>) -> Self {
        IdRange(range)
    }
}

impl Iterator for IdRange {
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[derive(Debug)]
pub struct ParseRangeError(String);

impl Display for ParseRangeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Error for ParseRangeError {}

impl FromStr for IdRange {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once('-') {
            if let (Ok(min), Ok(max)) = (a.parse(), b.parse()) {
                return Ok(IdRange::new(min, max));
            }
        };

        Err(ParseRangeError(s.to_string()))
    }
}
