use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Display, Formatter},
    num::ParseIntError,
    str::FromStr,
};

use shared_lib::point;

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

    /// Move `head` one step in `direction` and adjust `tail` accordingly
    ///
    /// # Tail Movement Rules
    ///
    /// * If `head` is ever two steps directly up, down, left, or right
    /// from the `tail`, the `tail` **must** also move one step in that direction
    ///
    /// * If `head` and `tail` aren't touching, and aren't in the same row
    /// and column, the `tail` **always** moves one step diagonally to keep up
    fn step(head: &mut Point, tail: &mut Point, direction: &Direction) {
        *head = move_point(head, direction);

        let x_diff = head.x.abs_diff(tail.x);
        let y_diff = head.y.abs_diff(tail.y);

        if x_diff <= 1 && y_diff <= 1 {
            return;
        }

        match (x_diff, y_diff) {
            // If `head` is ever two steps directly up, down, left, or right
            // from the `tail`, the `tail` **must** also move one step in that direction
            (2, 0) => {
                if head.x > tail.x {
                    tail.x += 1;
                } else {
                    tail.x -= 1;
                };
            }
            (0, 2) => {
                if head.y > tail.y {
                    tail.y += 1;
                } else {
                    tail.y -= 1;
                };
            }
            // If `head` and `tail` aren't touching, and aren't in the same row
            // and column, the `tail` **always** moves one step diagonally to keep up
            (2, 1) | (1, 2) => {
                if head.x > tail.x {
                    tail.x += 1;
                } else {
                    tail.x -= 1;
                };
                if head.y > tail.y {
                    tail.y += 1;
                } else {
                    tail.y -= 1;
                };
            }
            _ => unreachable!(),
        }
    }
}

/// Move a [`Point`] a single step in `direction`
///
/// # Returns
///
/// New point with value adjusted depending on direction
fn move_point(point: &Point, dir: &Direction) -> Point {
    use Direction::*;

    match dir {
        Up => (point.x, point.y + 1).into(),
        Right => (point.x + 1, point.y).into(),
        Left => (point.x - 1, point.y).into(),
        Down => (point.x, point.y - 1).into(),
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
            Rope::step(&mut rope.head, &mut rope.tail, &direction);
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

        assert_eq!(result, 6212);

        Ok(())
    }
}
