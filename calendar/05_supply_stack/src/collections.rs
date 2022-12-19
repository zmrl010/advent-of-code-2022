pub mod cratemap {
    use std::{
        collections::{
            hash_map::{Entry, IntoIter},
            HashMap,
        },
        str::FromStr,
    };

    use crate::{error::ParseError, Crate};

    /// Map string keys to crate collections
    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct CrateMap {
        map: HashMap<String, Vec<Crate>>,
    }

    impl CrateMap {
        /// Initialize an empty map
        pub fn new() -> Self {
            Default::default()
        }

        /// Exposes the entry api from the underlying HashMap
        pub fn entry(&mut self, key: String) -> Entry<'_, String, Vec<Crate>> {
            self.map.entry(key)
        }
    }

    impl IntoIterator for CrateMap {
        type Item = (String, Vec<Crate>);
        type IntoIter = IntoIter<String, Vec<Crate>>;

        fn into_iter(self) -> Self::IntoIter {
            self.map.into_iter()
        }
    }

    impl FromIterator<(String, Crate)> for CrateMap {
        fn from_iter<T: IntoIterator<Item = (String, Crate)>>(iter: T) -> Self {
            let mut map = Self::new();

            for (key, crate_) in iter {
                let stack = map.entry(key).or_default();
                stack.extend([crate_])
            }

            map
        }
    }

    impl FromStr for CrateMap {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut rows = s.lines().rev();
            let index_row = rows.next().ok_or_else(|| ParseError::ParseStringError {
                from: s.to_string(),
            })?;

            let mut map = CrateMap::new();

            let key_indices: Vec<(usize, String)> = index_row
                .match_indices(char::is_numeric)
                .map(|(i, key)| (i, key.to_string()))
                .collect();

            for (column_index, key) in key_indices {
                let crates: &mut Vec<Crate> = map.entry(key).or_default();

                crates.extend(rows.clone().filter_map(|row| {
                    let char = match row.chars().nth(column_index) {
                        Some(' ') | None => None,
                        Some(char) => Some(char),
                    }?;
                    Some(char)
                }));
            }

            Ok(map)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::{CrateMap, ParseError};

        const BASIC_EXAMPLE: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ";

        #[test]
        fn basic_example_should_parse_into_cratemap() -> Result<(), ParseError> {
            let result: CrateMap = BASIC_EXAMPLE.parse()?;

            let expected = {
                let mut map = CrateMap::new();

                map.entry("1".to_string()).or_insert(Vec::from(['Z', 'N']));
                map.entry("2".to_string())
                    .or_insert(Vec::from(['M', 'C', 'D']));
                map.entry("3".to_string()).or_insert(Vec::from(['P']));

                map
            };

            assert_eq!(result, expected);

            Ok(())
        }
    }
}
