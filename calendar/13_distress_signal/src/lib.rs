use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum ParseError {
    Unknown,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError(Unknown): unknown error has occurred.")
    }
}

impl Error for ParseError {}

pub fn part1(_input: &str) -> Result<usize, ParseError> {
    todo!()
}

pub fn part2(_input: &str) -> Result<usize, ParseError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT: &str = include_str!("../basic_input");
    const INPUT: &str = include_str!("../input");

    #[test]
    fn part1_basic_input_eq_13() -> Result<(), ParseError> {
        let result = part1(BASIC_INPUT)?;

        assert_eq!(result, 13);

        Ok(())
    }

    #[test]
    fn part1_basic_input_eq_value() -> Result<(), ParseError> {
        let result = part1(INPUT)?;

        assert_eq!(result, 394);

        Ok(())
    }

    #[test]
    fn part2_basic_input_eq_29() -> Result<(), ParseError> {
        let result = part2(BASIC_INPUT)?;

        assert_eq!(result, 29);

        Ok(())
    }

    #[test]
    fn part2_basic_input_eq_value() -> Result<(), ParseError> {
        let result = part2(INPUT)?;

        assert_eq!(result, 388);

        Ok(())
    }
}
