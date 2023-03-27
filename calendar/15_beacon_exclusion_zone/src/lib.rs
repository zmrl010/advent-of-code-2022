use std::{
    cmp::PartialOrd,
    collections::{HashMap, HashSet},
    num::ParseIntError,
    ops::Range,
};

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

const TUNING_FREQUENCY_MULTIPLIER: Integer = 4_000_000;

impl Point {
    /// Calculate the [Manhattan distance] between this point and another
    ///
    /// [Manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
    fn manhattan_distance(&self, other: &Point) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    /// Calculate the tuning frequency for this point
    ///
    /// To isolate the distress beacon's signal,
    /// you need to determine its tuning frequency,
    /// which can be found by multiplying its x coordinate
    /// by 4_000_000 and then adding its y coordinate.
    fn tuning_frequency(&self) -> Integer {
        dbg!(&self);
        self.x * TUNING_FREQUENCY_MULTIPLIER + self.y
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
    fn distance(&self) -> Integer {
        self.sensor.manhattan_distance(&self.beacon) as Integer
    }

    /// Get range of x values that is covered for a row at the passed `row_index`
    fn covered_x_range(&self, row_index: Integer) -> Range<Integer> {
        let Point { x, y } = self.sensor;
        let x_offset = self.distance() - y.abs_diff(row_index) as Integer;

        x - x_offset..x + x_offset + 1
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

type TryMergeRangeResult<Idx> = Result<Range<Idx>, (Range<Idx>, Range<Idx>)>;

fn try_merge_range<Idx>(a: Range<Idx>, b: Range<Idx>) -> TryMergeRangeResult<Idx>
where
    Idx: PartialOrd + Ord,
{
    if a.end >= b.start {
        Ok(a.start..b.end.max(a.end))
    } else {
        Err((a, b))
    }
}

fn count_covered_cells(readings: Vec<Reading>, row_index: Integer) -> usize {
    let covered_cell_count: usize = readings
        .iter()
        .filter_map(|reading| {
            let covered_columns = reading.covered_x_range(row_index);
            if covered_columns.is_empty() {
                return None;
            }
            Some(covered_columns)
        })
        .sorted_unstable_by_key(|r| r.start)
        .coalesce(try_merge_range)
        .map(|x| x.len())
        .sum();

    let blocked_cell_count = readings
        .into_iter()
        .flat_map(|reading| [reading.sensor, reading.beacon])
        .filter(|point| point.y == row_index)
        .unique()
        .count();

    covered_cell_count - blocked_cell_count
}

fn find_beacon<const LOWER_BOUND: Integer, const UPPER_BOUND: Integer>(
    readings: Vec<Reading>,
) -> Option<Point> {
    for row_index in LOWER_BOUND..UPPER_BOUND {}

    let possible_cells: HashSet<Point> = (LOWER_BOUND..UPPER_BOUND)
        .flat_map(|y| (LOWER_BOUND..UPPER_BOUND).map(move |x| (x, y).into()))
        .collect();

    let covered_cells_in_bounds: HashSet<Point> = readings
        .iter()
        .flat_map(|reading| {
            (LOWER_BOUND..UPPER_BOUND).flat_map(|y| {
                let covered_columns = reading.covered_x_range(y);
                covered_columns
                    .filter(|x| x > &LOWER_BOUND && x <= &UPPER_BOUND)
                    .map(move |x| (x, y).into())
            })
        })
        .collect();

    possible_cells
        .difference(&covered_cells_in_bounds)
        .next()
        .cloned()
}

pub fn part1<const ROW_INDEX: Integer>(input: &str) -> anyhow::Result<usize> {
    parse_input(input).and_then(|readings| {
        let count = count_covered_cells(readings, ROW_INDEX);
        Ok(count)
    })
}

pub fn part2<const LOWER_BOUND: Integer, const UPPER_BOUND: Integer>(
    input: &str,
) -> anyhow::Result<Integer> {
    let readings = parse_input(input)?;

    if let Some(beacon) = find_beacon::<LOWER_BOUND, UPPER_BOUND>(readings) {
        return Ok(beacon.tuning_frequency());
    }

    Err(anyhow::anyhow!("beacon not found"))
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
        let result = part2::<0, 20>(BASIC_INPUT)?;

        assert_eq!(result, 56_000_011);

        Ok(())
    }

    #[test]
    fn part2_input_result_eq_expected() -> anyhow::Result<()> {
        let result = part2::<0, 4_000_000>(INPUT)?;

        assert_eq!(result, 0);

        Ok(())
    }
}
