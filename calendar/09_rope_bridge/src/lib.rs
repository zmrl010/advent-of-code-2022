use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Display, Formatter},
    num::ParseIntError,
    str::FromStr,
};

use shared_lib::point::{self, Relative};

type Point = point::Point<isize>;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
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

impl Direction {
    /// Get the opposite direction
    ///
    /// Returns a new instance of the direction that is directly opposite to this one
    ///
    /// # Example
    ///
    /// ```rust
    /// use rope_bridge::Direction;
    ///
    /// let original = Direction::Left;
    /// assert_eq!(original.opposite(), Direction::Right)
    /// ```
    pub fn opposite(&self) -> Self {
        use Direction::*;

        match self {
            Left => Right,
            Right => Left,
            Up => Down,
            Down => Up,
        }
    }

    /// Get a [`Point`] value that can be applied to another [`Point`]
    /// in order to move it in that direction
    pub fn to_movement(&self) -> Point {
        use Direction::*;

        match self {
            Up => (0, 1).into(),
            Right => (1, 0).into(),
            Left => (-1, 0).into(),
            Down => (0, -1).into(),
        }
    }
}

#[derive(Debug)]
pub struct Rope(Vec<Point>);

impl Default for Rope {
    fn default() -> Self {
        Self(vec![(0, 0).into(), (0, 0).into()])
    }
}

impl Rope {
    /// Initialize a new rope with `length` number of points initialized to 0,0
    pub fn new(length: usize) -> Self {
        Self(vec![(0, 0).into(); length])
    }

    /// get last knot in rope
    fn tail(&self) -> Option<&Point> {
        self.0.last()
    }

    fn move_head(&mut self, direction: &Direction) {
        self.0[0] += direction.to_movement();
    }

    fn move_tail(&mut self) {
        let mut prev = self.0[0];

        for point in &mut self.0[1..] {
            point.move_relative(&prev);
            prev = *point
        }
    }

    fn step(&mut self, direction: &Direction) {
        move_rope(self, direction)
    }
}

/// Move `head` one step in `direction` and adjust `tail` accordingly
///
/// # Tail Movement Rules
///
/// * If `head` is ever two steps directly up, down, left, or right
/// from the `tail`, the `tail` **must** also move one step in that direction
///
/// * If `head` and `tail` aren't touching, and aren't in the same row
/// and column, the `tail` **always** moves one step diagonally to keep up
fn move_rope(rope: &mut Rope, direction: &Direction) {
    rope.move_head(direction);
    rope.move_tail();
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

fn count_points_tails_visited(moves: Vec<Motion>, rope: &mut Rope) -> usize {
    let mut set: HashSet<Point> = HashSet::new();

    for (direction, steps) in moves {
        for _ in 0..steps {
            rope.step(&direction);
            if let Some(point) = rope.tail() {
                set.insert(*point);
            }
        }
    }

    set.len()
}

pub fn part1_count_points_tail_visited(input: &str) -> Result<usize, ParseError> {
    let moves = parse_input(input)?;

    let mut rope = Rope::new(2);

    let result = count_points_tails_visited(moves, &mut rope);

    Ok(result)
}

pub fn part2_count_points_tails_visited(input: &str) -> Result<usize, ParseError> {
    let moves = parse_input(input)?;

    let mut rope = Rope::new(10);

    let result = count_points_tails_visited(moves, &mut rope);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";

    const LARGE_INPUT: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

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

        assert_eq!(result, 6212);

        Ok(())
    }

    #[test]
    fn part_2_basic_input_should_eq_1() -> Result<(), ParseError> {
        let result = part2_count_points_tails_visited(BASIC_INPUT)?;

        assert_eq!(result, 1);

        Ok(())
    }

    #[test]
    fn part_2_large_input_should_eq_36() -> Result<(), ParseError> {
        let result = part2_count_points_tails_visited(LARGE_INPUT)?;

        assert_eq!(result, 36);

        Ok(())
    }

    #[test]
    fn part_2_input_should_eq_value() -> Result<(), ParseError> {
        let result = part2_count_points_tails_visited(INPUT)?;

        assert_eq!(result, 2522);

        Ok(())
    }
}
