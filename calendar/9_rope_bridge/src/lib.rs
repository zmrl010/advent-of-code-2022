use std::{
    cmp::Ordering,
    collections::HashSet,
    error::Error,
    fmt::{self, Display, Formatter},
    num::ParseIntError,
    str::FromStr,
};

use shared_lib::grid::{Point, Touch};

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Rope {
    head: Point,
    tail: Point,
}

impl Default for Rope {
    fn default() -> Self {
        Self {
            head: (0, 0).into(),
            tail: (0, 0).into(),
        }
    }
}

impl Rope {
    pub fn new() -> Self {
        Default::default()
    }

    /// Move `head` by applying a `motion` and adjust `tail` accordingly
    ///
    /// # Arguments
    ///
    /// * `motion` - (direction, steps) tuple that describes a direction to move
    /// toward and the number of steps to take to get there.
    ///
    /// # Tail Movement Rules
    ///
    /// * If `head` is ever two steps directly up, down, left, or right
    /// from the `tail`, the `tail` **must** also move one step in that direction
    ///
    /// * If `head` and `tail` aren't touching, and aren't in the same row
    /// and column, the `tail` **always** moves one step diagonally to keep up
    pub fn apply_motion(&mut self, motion: Motion) {
        let (direction, steps) = motion;

        for _ in 0..steps {
            self.step(&direction);
        }
    }

    /// Take a single step toward `direction`
    fn step(&mut self, direction: &Direction) {
        move_point(&mut self.head, direction);

        if self.head == self.tail {
            return;
        }

        let x_diff = self.head.x.abs_diff(self.tail.x);
        let y_diff = self.head.y.abs_diff(self.tail.y);

        match (x_diff, y_diff) {
            // If `head` is ever two steps directly up, down, left, or right
            // from the `tail`, the `tail` **must** also move one step in that direction
            (2, 0) | (0, 2) => move_point(&mut self.tail, direction),
            // If `head` and `tail` aren't touching, and aren't in the same row
            // and column, the `tail` **always** moves one step diagonally to keep up
            (2, 2) => {
                let horizontal_direction = if self.head.x > self.tail.x {
                    Direction::Right
                } else {
                    Direction::Left
                };
                move_point(&mut self.tail, &horizontal_direction);

                let vertical_direction = if self.head.y > self.tail.y {
                    Direction::Up
                } else {
                    Direction::Down
                };
                move_point(&mut self.tail, &vertical_direction);
            }
            _ => {}
        }
    }
}

/// Move a [`Point`] a single step in `direction`
fn move_point(point: &mut Point, direction: &Direction) {
    match direction {
        Direction::Left => {
            point.x -= 1;
        }
        Direction::Right => {
            point.x += 1;
        }
        Direction::Up => {
            point.y += 1;
        }
        Direction::Down => {
            point.y -= 1;
        }
    }
}

pub type Motion = (Direction, u8);

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => return Err(ParseError::Direction(s.to_string())),
        };

        Ok(direction)
    }
}

#[derive(Debug)]
pub enum ParseError {
    Direction(String),
    Steps(ParseIntError),
    Input(String),
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Direction(value) => {
                write!(f, "ParseError: failed to parse direction from\n\n`{value}`")
            }
            ParseError::Input(value) => {
                writeln!(f, "ParseError: failed to parse input from\n\n`{value}`")
            }
            ParseError::Steps(value) => {
                writeln!(
                    f,
                    "ParseError: failed to parse steps\n\nCaused by: `{value}`"
                )
            }
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::Steps(err)
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Motion>, ParseError> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split_once(' ')
                .ok_or_else(|| ParseError::Input(input.to_string()))
                .and_then(|(raw_direction, raw_steps)| {
                    Ok((
                        raw_direction.parse::<Direction>()?,
                        raw_steps.parse::<u8>()?,
                    ))
                })
        })
        .collect()
}

pub fn part1_count_points_tail_visited(input: &str) -> Result<usize, ParseError> {
    let moves = parse_input(input)?;

    let mut rope = Rope::new();

    let mut set: HashSet<Point> = HashSet::new();

    for (direction, steps) in moves {
        for _ in 0..steps {
            rope.step(&direction);
            set.insert(rope.tail);
        }
    }

    Ok(set.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";

    const INPUT: &str = include_str!("../input");

    #[test]
    fn basic_input_should_eq_13() -> Result<(), ParseError> {
        let result = part1_count_points_tail_visited(BASIC_INPUT)?;

        assert_eq!(result, 13);

        Ok(())
    }

    #[test]
    fn input_should_eq_value() -> Result<(), ParseError> {
        let result = part1_count_points_tail_visited(INPUT)?;

        assert_eq!(result, 13);

        Ok(())
    }
}
