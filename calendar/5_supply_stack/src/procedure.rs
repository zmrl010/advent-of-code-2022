use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Debug)]
pub struct Instruction {
    /// number of crates to move
    pub(crate) num_crates: u8,
    /// key of stack crates are moving from
    pub(crate) from_key: char,
    /// key of stack crates are moving to
    pub(crate) to_key: char,
}

#[derive(Debug)]
pub enum ParseError {
    Instruction(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Instruction(msg) => {
                write!(
                    f,
                    "ParseError: unable to parse `Instruction` from string `{}`",
                    msg
                )
            }
        }
    }
}

impl Error for ParseError {}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        let instruction = Instruction {
            num_crates: parts[1].parse().unwrap_or_default(),
            from_key: parts[3]
                .chars()
                .next()
                .ok_or_else(|| ParseError::Instruction(s.to_string()))?,
            to_key: parts[5]
                .chars()
                .next()
                .ok_or_else(|| ParseError::Instruction(s.to_string()))?,
        };

        Ok(instruction)
    }
}

#[derive(Debug)]
struct Procedure;

pub fn parse_procedure(input: &str) {
    input
        .lines()
        .into_iter()
        .map(|line| line.parse::<Instruction>());
}
