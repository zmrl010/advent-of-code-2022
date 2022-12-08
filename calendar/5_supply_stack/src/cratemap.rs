use std::{
    collections::{
        hash_map::{Entry, IntoIter},
        HashMap,
    },
    str::FromStr,
};

use crate::{error::ParseError, Crate, Stack};

/// Map string keys to stacks of crates.
///
/// Wraps [`HashMap`]
#[derive(Debug, PartialEq, Eq, Default)]
pub struct CrateMap(HashMap<String, Stack>);

impl CrateMap {
    /// Initialize an empty map
    pub fn new() -> Self {
        Default::default()
    }

    /// Exposes the entry api from the underlying HashMap
    pub fn entry(&mut self, key: String) -> Entry<'_, String, Stack> {
        self.0.entry(key)
    }
}

impl IntoIterator for CrateMap {
    type Item = (String, Stack);
    type IntoIter = IntoIter<String, Stack>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<(String, Crate)> for CrateMap {
    fn from_iter<T: IntoIterator<Item = (String, Crate)>>(iter: T) -> Self {
        let mut map: HashMap<String, Stack> = HashMap::new();

        for (key, crate_) in iter {
            let stack = map.entry(key).or_default();
            stack.push(crate_)
        }

        Self(map)
    }
}

impl FromStr for CrateMap {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = s.lines().rev();
        let index_row = rows.next().ok_or_else(|| ParseError::ParseStringError {
            from: s.to_string(),
        })?;

        let map_iterator = rows.flat_map(|row| {
            index_row
                .match_indices(char::is_numeric)
                .filter_map(|(col_index, key)| {
                    let col = match row.chars().nth(col_index) {
                        Some(' ') | None => None,
                        Some(col) => Some(col),
                    }?;
                    Some((key.to_string(), col))
                })
        });

        let stacks = CrateMap::from_iter(map_iterator);

        Ok(stacks)
    }
}

#[cfg(test)]
mod tests {
    use super::{CrateMap, ParseError, Stack};

    const BASIC_EXAMPLE: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ";

    #[test]
    fn basic_example_should_parse_into_cratemap() -> Result<(), ParseError> {
        let result: CrateMap = BASIC_EXAMPLE.parse()?;

        let expected = {
            let mut map = CrateMap::new();

            map.entry("1".to_string())
                .or_insert(Stack::from(['Z', 'N']));
            map.entry("2".to_string())
                .or_insert(Stack::from(['M', 'C', 'D']));
            map.entry("3".to_string()).or_insert(Stack::from(['P']));

            map
        };

        assert_eq!(result, expected);

        Ok(())
    }
}
