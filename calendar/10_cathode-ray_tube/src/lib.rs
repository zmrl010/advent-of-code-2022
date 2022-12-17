use std::{
    error,
    fmt::{self, Display, Formatter},
    num::ParseIntError,
};

#[derive(Debug)]
struct Clock {
    cycle: u32,
}

#[derive(Debug)]
struct CPU {
    x: i32,
}

impl CPU {
    fn new() -> Self {
        Self { x: 1 }
    }
}

fn sum_signal_strengths(cpu: &mut CPU, program: Vec<Instruction>) -> i32 {
    let mut cycle = 0u16;

    let mut sum = 0i32;

    for instruction in program {
        match instruction {
            Instruction::Noop => {
                cycle += 1;

                if let 20 | 60 | 100 | 140 | 180 | 220 = cycle {
                    sum += cpu.x * cycle as i32;
                }
            }
            Instruction::Addx(value) => {
                cycle += 1;

                if let 20 | 60 | 100 | 140 | 180 | 220 = cycle {
                    sum += cpu.x * cycle as i32;
                }

                cycle += 1;

                if let 20 | 60 | 100 | 140 | 180 | 220 = cycle {
                    sum += cpu.x * cycle as i32;
                }

                cpu.x += value;
            }
        }
    }

    dbg!(cycle);

    sum
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

    let sum = sum_signal_strengths(&mut cpu, input);

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
}
