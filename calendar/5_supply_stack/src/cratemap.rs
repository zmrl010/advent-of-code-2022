use std::{
    collections::{hash_map::Entry, HashMap},
    str::FromStr,
};

use crate::{Crate, CrateStack};

#[derive(Debug)]
pub enum ParseError {
    Invalid(String),
}

/// Map string keys to stacks of crates with this thin wrapper around HashMap
#[derive(Debug, PartialEq, Eq)]
pub struct CrateMap(HashMap<String, CrateStack>);

impl CrateMap {
    /// Initialize an empty map
    fn new() -> Self {
        Self(HashMap::new())
    }

    /// Exposes the entry api from the underlying HashMap
    fn entry(&mut self, key: String) -> Entry<'_, String, CrateStack> {
        self.0.entry(key)
    }
}

// impl IntoIterator for CrateMap {
//     type Item = Crate;
//     type IntoIter = IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

impl FromStr for CrateMap {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks = CrateMap::new();

        let mut rows = s.lines().rev();
        let index_row = rows
            .next()
            .ok_or_else(|| ParseError::Invalid("index row not found".to_string()))?;
        for row in rows {
            // let crates =
            //     index_row
            //         .match_indices(char::is_numeric)
            //         .filter_map(|(col_index, key)| {
            //             let col = match row.chars().nth(col_index) {
            //                 Some(' ') | None => None,
            //                 Some(col) => Some(Crate(col)),
            //             }?;
            //             Some((key, col))
            //         });
            for (col_index, stacks_key) in index_row.match_indices(char::is_numeric) {
                let col = match row.chars().nth(col_index) {
                    Some(' ') | None => continue,
                    Some(col) => col,
                };
                let stack = stacks.entry(stacks_key.to_string()).or_default();
                stack.push(Crate(col))
            }
        }

        Ok(stacks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_EXAMPLE: &str = "    [D]    
    [N] [C]    
    [Z] [M] [P]
     1   2   3 ";

    #[test]
    fn basic_example_should_parse_into_cratemap() -> Result<(), ParseError> {
        let result: CrateMap = BASIC_EXAMPLE.parse()?;

        let expected = {
            let mut map = CrateMap::new();

            map.entry("1".to_string())
                .or_insert(CrateStack::from(['Z', 'N']));
            map.entry("2".to_string())
                .or_insert(CrateStack::from(['M', 'C', 'D']));
            map.entry("3".to_string())
                .or_insert(CrateStack::from(['P']));

            map
        };

        assert_eq!(result, expected);

        Ok(())
    }
}
