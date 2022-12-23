use std::{
    error,
    fmt::{self, Display, Formatter},
    num::ParseIntError,
};

use pathfinding::{prelude::Grid, utils::in_direction};

#[derive(Debug)]
enum Error {
    ParseInput(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseInput(reason) => write!(
                f,
                "Error(ParseInput): Error parsing puzzle input.\n\nCaused by: `{reason}`"
            ),
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::ParseInput(err.to_string())
    }
}

impl error::Error for Error {}

pub type Point = (usize, usize);

pub type RockPath = Vec<Point>;

pub fn parse_input(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .flat_map(|line| -> RockPath {
            let mut path = RockPath::new();

            let mut points = line.trim().split(" -> ").map(|raw_point| -> Point {
                let (x, y) = raw_point
                    .split_once(",")
                    .expect("raw point should contain `,`");

                (
                    x.parse().expect("left side should be a valid integer"),
                    y.parse().expect("right side should be a valid integer"),
                )
            });

            if let Some(previous) = points.next() {
                for point in points {
                    if point.0 == previous.0 {
                        let (min, max) = (previous.1.min(point.1), previous.1.max(point.1));

                        path.extend((min..=max).map(|y| (point.0, y)));
                    } else {
                        let (min, max) = (previous.0.min(point.0), previous.0.max(point.0));

                        path.extend((min..=max).map(|x| (x, point.1)));
                    }
                }
            }

            path
        })
        .collect()
}

const START: Point = (500, 0);

pub fn drop_sand(grid: &mut Grid) {
    let points = in_direction(
        START,
        (500, grid.height as isize),
        (grid.width, grid.height),
    )
    .rev();
}

pub fn part1(input: &str) -> usize {
    let grid = parse_input(input);

    let mut sand_has_fallen_through = false;

    while !sand_has_fallen_through {
        let sand_plots = grid.dfs_reachable((500, 0), |(x, y)| grid);

        sand_has_fallen_through = true;
    }

    0
}

pub fn part2(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT: &str = include_str!("../basic_input");
    const INPUT: &str = include_str!("../input");

    #[test]
    fn part1_basic_input_result_eq_expected() {
        let result = part1(BASIC_INPUT);

        assert_eq!(result, 0);
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
