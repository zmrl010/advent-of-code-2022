use std::{
    error,
    fmt::{self, Display, Formatter},
    num::ParseIntError,
};

#[derive(Debug)]
struct CPU {
    x: i32,
    cycle: u16,
}

impl CPU {
    fn new() -> Self {
        Self { x: 1, cycle: 0 }
    }

    fn increment_cycle(&mut self, signal: &mut i32, next_cycle: &mut u16) {
        self.cycle += 1;

        if *next_cycle == self.cycle {
            *signal += self.compute_signal_modifier();
            *next_cycle += 40;
        }
    }

    fn execute(&mut self, program: &[Instruction]) -> i32 {
        let mut signal = 0i32;
        let mut next_cycle = 20;

        for instruction in program {
            match instruction {
                Instruction::Noop => {
                    self.increment_cycle(&mut signal, &mut next_cycle);
                }
                Instruction::Addx(value) => {
                    self.increment_cycle(&mut signal, &mut next_cycle);
                    self.increment_cycle(&mut signal, &mut next_cycle);

                    self.x += value;
                }
            }
        }

        signal
    }

    fn compute_signal_modifier(&self) -> i32 {
        self.x * self.cycle as i32
    }
}

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Debug)]
pub enum Error {
    ParseInstructionError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseInstructionError(err) => {
                write!(
                    f,
                    "ParseInstructionError: failed to parse instruction. \n\n{err}"
                )
            }
        }
    }
}

impl error::Error for Error {}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::ParseInstructionError(err.to_string())
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Instruction>, Error> {
    input
        .trim()
        .lines()
        .map(|line| match line.trim() {
            "noop" => Ok(Instruction::Noop),
            value if value.starts_with("addx") => {
                let value = value.split(' ').nth(1).ok_or_else(|| {
                    Error::ParseInstructionError(format!("invalid format `{line}`"))
                })?;
                let value = value.parse()?;
                Ok(Instruction::Addx(value))
            }
            _ => {
                return Err(Error::ParseInstructionError(format!(
                    "unrecognized instruction `{line}`"
                )))
            }
        })
        .collect()
}

pub fn part1(input: &str) -> Result<i32, Error> {
    let input = parse_input(input)?;

    let mut cpu = CPU::new();

    let sum = cpu.execute(&input);

    Ok(sum)
}

pub fn part2(input: &str) -> Result<i32, Error> {
    let input = parse_input(input)?;

    let mut cpu = CPU::new();

    let sum = cpu.execute(&input);

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT: &str = include_str!("../basic_input");
    const INPUT: &str = include_str!("../input");

    #[test]
    fn basic_input_should_eq_13140() {
        let result = part1(BASIC_INPUT).unwrap();
        assert_eq!(result, 13140);
    }

    #[test]
    fn input_should_eq_value() {
        let result = part1(INPUT).unwrap();
        assert_eq!(result, 14060);
    }

    #[test]
    fn part2_basic_input_should_eq_13140() {
        let result = part2(BASIC_INPUT).unwrap();
        assert_eq!(result, 13140);
    }

    #[test]
    fn part2_input_should_eq_value() {
        let result = part2(INPUT).unwrap();
        assert_eq!(result, 14060);
    }
}
