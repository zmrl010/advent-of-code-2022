use std::{collections::VecDeque, error, fmt::Display, num::ParseIntError, str::FromStr};

pub type Index = usize;
pub type WorryLevel = u64;

#[derive(Debug)]
pub enum ParseError {
    Monkey(String),
    Operation(String),
    Test(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Monkey(err) => write!(
                f,
                "ParseError(Monkey): failed to parse monkey.\n\nCaused by: {err}"
            ),
            ParseError::Operation(err) => write!(
                f,
                "ParseError(Operation): failed to parse operation.\n\nCaused by: {err}"
            ),
            ParseError::Test(err) => write!(
                f,
                "ParseError(Test): failed to parse monkey test.\n\nCaused by: {err}"
            ),
        }
    }
}

impl error::Error for ParseError {}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        Self::Monkey(err.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: VecDeque<WorryLevel>,
    operation: Operation,
    test: MonkeyTest,
    inspection_count: u128,
}

impl Monkey {
    /// Monkey inspects an item.
    ///
    /// Apply's the monkey's operation to an item's worry level and
    /// to returns the worry level after inspection
    fn inspect(&mut self, worry_level: &WorryLevel) -> WorryLevel {
        self.inspection_count += 1;

        match self.operation {
            Operation::Square => worry_level * worry_level,
            Operation::Multiply(operand) => worry_level * operand,
            Operation::Add(operand) => worry_level + operand,
        }
    }

    /// Perform monkey's test on a worry level value and return the
    /// index of the monkey to throw the item to
    fn test(&self, worry_level: WorryLevel) -> Index {
        if worry_level % self.test.divisor == 0 {
            self.test.if_true
        } else {
            self.test.if_false
        }
    }
}

impl FromStr for Monkey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value_chunks: Result<Vec<&str>, Self::Err> = s
            .trim()
            .lines()
            .map(|line| line.trim())
            .filter(|line| *line != "")
            .skip(1)
            .map(|line| {
                let value = line
                    .split_once(": ")
                    .ok_or_else(|| {
                        ParseError::Monkey(format!("Expected deliminator `: ` in `{line}`"))
                    })?
                    .1;

                Ok(value)
            })
            .collect();

        let value_chunks = value_chunks?;

        let starting_items: Result<VecDeque<WorryLevel>, ParseError> = value_chunks
            .get(0)
            .ok_or_else(|| ParseError::Monkey("Missing value chunk at index 0".to_string()))?
            .split(", ")
            .map(|worry_level| Ok(worry_level.parse::<WorryLevel>()?))
            .collect();

        let operation: Operation = value_chunks
            .get(1)
            .ok_or_else(|| ParseError::Monkey("Missing value chunk at index 1".to_string()))?
            .parse()?;

        let test: MonkeyTest = value_chunks
            .get(2..=4)
            .ok_or_else(|| {
                ParseError::Monkey("Missing value chunks in the range 2..=4".to_string())
            })?
            .try_into()?;

        Ok(Self {
            items: starting_items?,
            operation,
            test,
            inspection_count: 0,
        })
    }
}

#[derive(Debug, Clone)]
struct MonkeyTest {
    /// Number to test if an operand is divisible by
    divisor: u64,
    /// Index of monkey to throw to if test results in true
    if_true: Index,
    /// Index of monkey to throw to if test results in false
    if_false: Index,
}

impl TryFrom<&[&str]> for MonkeyTest {
    type Error = ParseError;

    fn try_from(parts: &[&str]) -> Result<Self, Self::Error> {
        let test_parts: Result<Vec<u64>, Self::Error> = parts
            .into_iter()
            .map(|line| {
                let value: u64 = line
                    .split(' ')
                    .last()
                    .ok_or_else(|| ParseError::Test("Missing a line's last value".to_string()))?
                    .parse()?;
                Ok(value)
            })
            .collect();

        let test_parts = test_parts?;

        Ok(MonkeyTest {
            divisor: test_parts[0],
            if_true: test_parts[1] as Index,
            if_false: test_parts[2] as Index,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Square,
    Multiply(u64),
    Add(u64),
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');

        let operator = parts.nth(3).ok_or_else(|| {
            ParseError::Operation("Missing expected operator (* | +)".to_string())
        })?;
        let operand = parts.nth(0).ok_or_else(|| {
            ParseError::Operation("Missing expected operand (old | integer)".to_string())
        })?;

        let operation = match (operator, operand) {
            ("+", operand) if operand.chars().all(char::is_numeric) => {
                Operation::Add(operand.parse()?)
            }
            ("*", operand) if operand.chars().all(char::is_numeric) => {
                Operation::Multiply(operand.parse()?)
            }
            ("*", "old") => Operation::Square,
            (operator, operand) => {
                return Err(ParseError::Operation(format!(
                    "Invalid operator ({operator}) and/or operand ({operand})"
                )))
            }
        };

        Ok(operation)
    }
}

fn parse_input(input: &str) -> Result<Vec<Monkey>, ParseError> {
    input
        .trim()
        .split("\n\n")
        .map(|raw_monkey| -> Result<Monkey, ParseError> { raw_monkey.trim().parse() })
        .collect()
}

fn process_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        let mut items = monkeys[i].items.clone();
        monkeys[i].items.clear();

        while let Some(worry_level) = items.pop_front() {
            let worry_level = monkeys[i].inspect(&worry_level);
            let worry_level = ((worry_level as f64) / 3f64) as u64;

            let next_monkey_index = monkeys[i].test(worry_level);
            monkeys[next_monkey_index].items.push_back(worry_level);
        }
    }
}

fn process_round_part2(monkeys: &mut Vec<Monkey>) {
    let lcm: u64 = monkeys.iter().map(|monkey| monkey.test.divisor).product();
    for i in 0..monkeys.len() {
        let mut items = monkeys[i].items.clone();
        monkeys[i].items.clear();

        while let Some(worry_level) = items.pop_front() {
            let worry_level = monkeys[i].inspect(&worry_level);
            let worry_level = worry_level % lcm;

            let next_monkey_index = monkeys[i].test(worry_level);
            monkeys[next_monkey_index].items.push_back(worry_level);
        }
    }
}

fn calculate_monkey_business(monkeys: Vec<Monkey>, num_rounds: u64) -> u128 {
    let mut monkeys = monkeys.clone();

    for _ in 0..num_rounds {
        process_round(&mut monkeys)
    }

    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));

    monkeys
        .iter()
        .take(2)
        .map(|monkey| monkey.inspection_count)
        .product()
}

fn calculate_monkey_business_part2(monkeys: Vec<Monkey>, num_rounds: u64) -> u128 {
    let mut monkeys = monkeys.clone();

    for _ in 0..num_rounds {
        process_round_part2(&mut monkeys)
    }

    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));

    monkeys
        .iter()
        .take(2)
        .map(|monkey| monkey.inspection_count)
        .product()
}

pub fn part1(input: &str) -> Result<u128, ParseError> {
    let monkeys = parse_input(input)?;

    let result = calculate_monkey_business(monkeys, 20);

    Ok(result)
}

pub fn part2(input: &str) -> Result<u128, ParseError> {
    let monkeys = parse_input(input)?;

    let result = calculate_monkey_business_part2(monkeys, 10000);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT: &str = include_str!("../basic_input");
    const INPUT: &str = include_str!("../input");

    #[test]
    fn basic_input_results_in_10605() -> Result<(), ParseError> {
        let result = part1(BASIC_INPUT)?;

        assert_eq!(result, 10605);

        Ok(())
    }

    #[test]
    fn input_results_in_value() -> Result<(), ParseError> {
        let result = part1(INPUT)?;

        assert_eq!(result, 61005);

        Ok(())
    }

    #[test]
    fn part2_basic_input_results_in_2_713_310_158() -> Result<(), ParseError> {
        let result = part2(BASIC_INPUT)?;

        assert_eq!(result, 2_713_310_158);

        Ok(())
    }

    #[test]
    fn part2_input_results_in_value() -> Result<(), ParseError> {
        let result = part2(INPUT)?;

        assert_eq!(result, 20_567_144_694);

        Ok(())
    }
}
