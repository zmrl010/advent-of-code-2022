use std::{num::ParseIntError, ops::Range};

use itertools::Itertools;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

type Integer = i32;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: Integer,
    y: Integer,
}

impl From<(Integer, Integer)> for Point {
    fn from((x, y): (Integer, Integer)) -> Self {
        Self { x, y }
    }
}

impl Point {
    /// Calculate the [Manhattan distance] between this point and another
    ///
    /// [Manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
    fn manhattan_distance(&self, other: &Point) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Hash)]
struct Reading {
    sensor: Point,
    beacon: Point,
}

impl Reading {
    fn new(sensor: Point, beacon: Point) -> Self {
        Self { sensor, beacon }
    }

    /// Calculate distance between sensor and beacon
    fn distance(&self) -> u32 {
        self.sensor.manhattan_distance(&self.beacon)
    }

    fn covered_x_range(&self, row_index: Integer) -> Option<Range<Integer>> {
        let Point { x, y } = self.sensor;
        let x_offset = self.distance() as Integer - y.abs_diff(row_index) as Integer;

        Some(x - x_offset..x + x_offset + 1).filter(|r| !r.is_empty())
    }
}

/// Extract integers from input into a [`Vec`]
fn collect_numbers(input: &str) -> Result<Vec<Integer>, ParseIntError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
    }

    RE.find_iter(input)
        .map(|raw| raw.as_str().parse())
        .collect()
}

/// Parse sensor reading from one line of input
fn parse_line(line: &str) -> anyhow::Result<Reading> {
    let values = collect_numbers(line)?;

    if values.len() != 4 {
        return Err(anyhow::anyhow!(
            "expected input to contain 4 numbers\n\nReceived: `{line}`"
        ));
    }

    Ok(Reading::new(
        (values[0], values[1]).into(),
        (values[2], values[3]).into(),
    ))
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Reading>> {
    input.lines().map(|line| parse_line(line.trim())).collect()
}

pub fn part1<const EXAMPLE_ROW: i32>(input: &str) -> anyhow::Result<usize> {
    let readings = parse_input(input)?;

    let covered_xs: usize = readings
        .iter()
        .flat_map(|reading| reading.covered_x_range(EXAMPLE_ROW))
        .sorted_unstable_by_key(|r| r.start)
        .coalesce(|a, b| {
            if a.end >= b.start {
                Ok(a.start..b.end.max(a.end))
            } else {
                Err((a, b))
            }
        })
        .map(|x| x.len())
        .sum();

    let blocked_xs = readings
        .into_iter()
        .flat_map(|reading| [reading.sensor, reading.beacon])
        .filter(|point| point.y == EXAMPLE_ROW)
        .unique()
        .count();

    Ok(covered_xs - blocked_xs)
}

pub fn part2(_input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT: &str = include_str!("../basic_input");
    const INPUT: &str = include_str!("../input");

    #[test]
    fn part1_basic_input_result_eq_expected() -> anyhow::Result<()> {
        let result = part1::<10>(BASIC_INPUT)?;

        assert_eq!(result, 26);

        Ok(())
    }

    #[test]
    fn part1_input_result_eq_expected() -> anyhow::Result<()> {
        let result = part1::<2_000_000>(INPUT)?;

        assert_eq!(result, 5_838_453);

        Ok(())
    }

    #[test]
    fn part2_basic_input_result_eq_expected() -> anyhow::Result<()> {
        let result = part2(BASIC_INPUT)?;

        assert_eq!(result, 0);

        Ok(())
    }

    #[test]
    fn part2_input_result_eq_expected() -> anyhow::Result<()> {
        let result = part2(INPUT)?;

        assert_eq!(result, 0);

        Ok(())
    }
}
