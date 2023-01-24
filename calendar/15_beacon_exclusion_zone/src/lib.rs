use std::{
    collections::{HashMap, HashSet},
    default,
};

use regex::Regex;

#[macro_use]
extern crate lazy_static;

type Integer = isize;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: Integer,
    y: Integer,
}

impl From<(Integer, Integer)> for Point {
    fn from((x, y): (Integer, Integer)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Hash)]
struct Reading {
    sensor: Point,
    beacon: Point,
}

impl Reading {
    fn new(position: Point, beacon: Point) -> Self {
        Self {
            sensor: position,
            beacon,
        }
    }
}

#[derive(Debug, Default)]
enum Cell {
    #[default]
    Unknown,
    Empty,
    Sensor,
    Beacon,
}

/// Calculate the manhattan distance between two points
fn calculate_distance(a: Point, b: Point) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

/// Parse plot points into a Sensor from one line of input
fn read_sensor(line: &str) -> anyhow::Result<Reading> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
    }

    let coordinates: anyhow::Result<Vec<Integer>> = RE
        .find_iter(line)
        .map(|raw| Ok(raw.as_str().parse()?))
        .collect();

    if let [sensor_x, sensor_y, beacon_x, beacon_y] = coordinates?[..] {
        return Ok(Reading::new(
            (sensor_x, sensor_y).into(),
            (beacon_x, beacon_y).into(),
        ));
    }

    Err(anyhow::anyhow!(
        "expected 2 sets of coordinates in `{line}`"
    ))
}

fn parse_input(input: &str) -> impl Iterator<Item = anyhow::Result<Reading>> + '_ {
    input.lines().map(|line| read_sensor(line.trim()))
}

pub fn part1(input: &str) -> u32 {
    let mut cell_map = HashMap::<Point, Cell>::new();
    let readings = parse_input(input);

    for reading in readings {
        let reading = reading.unwrap();

        cell_map.insert(reading.sensor, Cell::Sensor);
        cell_map.entry(reading.beacon).or_insert(Cell::Beacon);

        let distance = calculate_distance(reading.sensor, reading.beacon);
    }

    0
}

pub fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT: &str = include_str!("../basic_input");
    const INPUT: &str = include_str!("../input");

    #[test]
    fn part1_basic_input_result_eq_expected() {
        let result = part1(BASIC_INPUT);

        assert_eq!(result, 26);
    }

    #[test]
    fn part1_input_result_eq_expected() {
        let result = part1(INPUT);

        assert_eq!(result, 0);
    }

    #[test]
    fn part2_basic_input_result_eq_expected() {
        let result = part2(BASIC_INPUT);

        assert_eq!(result, 0)
    }

    #[test]
    fn part2_input_result_eq_expected() {
        let result = part2(INPUT);

        assert_eq!(result, 0);
    }
}
