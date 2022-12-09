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
    pub struct CrateMap<V> {
        map: HashMap<String, V>,
    }

    impl<V: Default> CrateMap<V> {
        /// Initialize an empty map
        pub fn new() -> Self {
            Default::default()
        }
    }

    impl<V> CrateMap<V> {
        /// Exposes the entry api from the underlying HashMap
        pub fn entry(&mut self, key: String) -> Entry<'_, String, V> {
            self.map.entry(key)
        }
    }

    impl<V> IntoIterator for CrateMap<V> {
        type Item = (String, V);
        type IntoIter = IntoIter<String, V>;

        fn into_iter(self) -> Self::IntoIter {
            self.map.into_iter()
        }
    }

    impl<V: Default + Extend<Crate>> FromIterator<(String, Crate)> for CrateMap<V> {
        fn from_iter<T: IntoIterator<Item = (String, Crate)>>(iter: T) -> Self {
            let mut map = Self::new();

            for (key, crate_) in iter {
                let stack = map.entry(key).or_default();
                stack.extend([crate_])
            }

            map
        }
    }

    impl<V: Default + Extend<Crate>> FromStr for CrateMap<V> {
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
                let crates: &mut V = map.entry(key).or_default();

                crates.extend(rows.clone().filter_map(|row| {
                    let char = match row.chars().nth(column_index) {
                        Some(' ') | None => None,
                        Some(char) => Some(char),
                    }?;
                    Some(char)
                }));
            }

            // let map_iterator = rows.flat_map(|row| {
            //     key_indices.iter().filter_map(|(col_index, key)| {
            //         let col = match row.chars().nth(*col_index) {
            //             Some(' ') | None => None,
            //             Some(col) => Some(col),
            //         }?;
            //         Some((key.to_string(), col))
            //     })
            // });

            Ok(map)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::{Crate, CrateMap, ParseError};

        const BASIC_EXAMPLE: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ";

        #[test]
        fn basic_example_should_parse_into_cratemap() -> Result<(), ParseError> {
            let result: CrateMap<Vec<Crate>> = BASIC_EXAMPLE.parse()?;

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
