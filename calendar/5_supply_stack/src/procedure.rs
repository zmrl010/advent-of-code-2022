use std::{str::FromStr, vec::IntoIter};

use crate::error::ParseError;

/// String-parsable struct representing a single instruction
///
/// # Example
///
/// ```rust
/// use supply_stack::Instruction;
///
/// let result: Instruction = "move 1 from 2 to 1".parse().unwrap();
///     
/// let expected = Instruction::new(1, "2", "1");
///
/// assert_eq!(result, expected);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    /// number of crates to move
    pub(crate) num_crates: u8,
    /// key of collection that crates are moving from
    pub(crate) source: String,
    /// key of collection that crates are moving to
    pub(crate) target: String,
}

impl Instruction {
    pub fn new(num_crates: u8, source: &str, target: &str) -> Self {
        Self {
            num_crates,
            source: source.to_string(),
            target: target.to_string(),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');

        let mut next = || -> Result<&str, Self::Err> {
            parts.nth(1).ok_or_else(|| ParseError::ParseStringError {
                from: s.to_string(),
            })
        };

        Ok(Instruction {
            num_crates: next()?.parse().unwrap_or_default(),
            source: next()?.to_string(),
            target: next()?.to_string(),
        })
    }
}

/// String-parsable [`Vec<Instruction>`] wrapper
///
/// # Example
///
/// ```rust
/// use supply_stack::{Instruction, Procedure};
///
/// const BASIC_EXAMPLE: &str = "move 1 from 2 to 1
/// move 3 from 1 to 3
/// move 2 from 2 to 1
/// move 1 from 1 to 2";
///
/// let result: Procedure = BASIC_EXAMPLE.parse().unwrap();
///     
/// let expected = Procedure::from([
///     Instruction::new(1, "2", "1"),
///     Instruction::new(3, "1", "3"),
///     Instruction::new(2, "2", "1"),
///     Instruction::new(1, "1", "2"),
/// ]);
///
/// assert_eq!(result, expected);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Procedure(Vec<Instruction>);

impl<const N: usize> From<[Instruction; N]> for Procedure {
    fn from(s: [Instruction; N]) -> Self {
        Self(Vec::from(s))
    }
}

impl FromIterator<Instruction> for Procedure {
    fn from_iter<T: IntoIterator<Item = Instruction>>(iter: T) -> Self {
        let instructions: Vec<Instruction> = iter.into_iter().collect();
        Self(instructions)
    }
}

impl IntoIterator for Procedure {
    type Item = Instruction;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromStr for Procedure {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction_results: Result<Vec<Instruction>, ParseError> =
            s.lines().map(|line| line.parse::<Instruction>()).collect();

        let instructions = instruction_results.map_err(|_| ParseError::ParseStringError {
            from: s.to_string(),
        })?;

        Ok(Procedure::from_iter(instructions))
    }
}
