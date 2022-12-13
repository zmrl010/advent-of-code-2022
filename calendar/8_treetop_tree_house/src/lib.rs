use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    num::ParseIntError,
    str::FromStr,
};

use shared_lib::grid::{Grid, GridLike, ParseGridError, Point};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree {
    height: u8,
}

#[derive(Debug)]
pub struct ParseTreeError(ParseIntError);

impl From<ParseIntError> for ParseTreeError {
    fn from(err: ParseIntError) -> Self {
        ParseTreeError(err)
    }
}

impl Display for ParseTreeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ParseTreeError: failed to parse `Tree`\n\n{}", self.0)
    }
}

impl Error for ParseTreeError {}

impl FromStr for Tree {
    type Err = ParseTreeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height: u8 = s.parse()?;

        Ok(Tree { height })
    }
}

fn is_edge(grid: &Grid<Tree>, point: Point) -> bool {
    let (width, height) = (grid.width(), grid.height());

    point.x == 0 || point.y == 0 || point.x == width - 1 || point.y == height - 1
}

/// Calculate the scenic score for a tree at `point`
///
/// A tree's scenic score is found by multiplying together its
/// viewing distance in each of the four directions.
///
/// To measure the viewing distance from a given tree, look up, down, left, and right
/// from that tree; stop if you reach an edge or at the first tree that is the same
/// height or taller than the tree under consideration.
fn calculate_scenic_score(grid: &Grid<Tree>, point: Point) -> usize {
    let (width, height) = (grid.width(), grid.height());

    if is_edge(grid, point) {
        return 0;
    }

    let target_tree = grid.get(point);

    let mut left = 0usize;
    for x in (0..point.x).rev() {
        left += 1;
        let tree = grid.get((x, point.y).into());
        if tree >= target_tree {
            break;
        }
    }

    let mut right = 0usize;
    for x in point.x + 1..width {
        right += 1;
        let tree = grid.get((x, point.y).into());
        if tree >= target_tree {
            break;
        }
    }

    let mut up = 0usize;
    for y in (0..point.y).rev() {
        up += 1;
        let tree = grid.get((point.x, y).into());
        if tree >= target_tree {
            break;
        }
    }

    let mut down = 0usize;
    for y in point.y + 1..height {
        down += 1;
        let tree = grid.get((point.x, y).into());
        if tree >= target_tree {
            break;
        }
    }

    left * right * down * up
}

/// Check if the tree at `point` is visible
///
/// A tree is **visible** if all of the other trees between it
/// and an edge of the grid are **shorter** than it.
pub fn check_tree_visibility(grid: &Grid<Tree>, point: Point) -> bool {
    let (width, height) = (grid.width(), grid.height());

    if is_edge(grid, point) {
        return true;
    }

    let target_tree = grid.get(point);

    let check_x = |x| {
        let tree = grid.get((x, point.y).into());
        tree < target_tree
    };

    let check_y = |y| {
        let tree = grid.get((point.x, y).into());
        tree < target_tree
    };

    (0..point.x).rev().all(check_x)
        || (point.x + 1..width).all(check_x)
        || (0..point.y).rev().all(check_y)
        || (point.y + 1..height).all(check_y)
}

impl Tree {
    pub fn new(height: u8) -> Self {
        Self { height }
    }
}

pub fn count_visible_trees(input: &str) -> Result<u64, ParseGridError> {
    let grid = input.parse::<Grid<Tree>>()?;

    let mut count = 0;

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let tree_is_visible = check_tree_visibility(&grid, (x, y).into());

            if tree_is_visible {
                count += 1;
            }
        }
    }

    Ok(count)
}

pub fn find_max_scenic_score(input: &str) -> Result<usize, ParseGridError> {
    let grid = input.parse::<Grid<Tree>>()?;

    let mut max_scenic_score = 0;

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let scenic_score = calculate_scenic_score(&grid, (x, y).into());

            max_scenic_score = scenic_score.max(max_scenic_score)
        }
    }

    Ok(max_scenic_score)
}

#[cfg(test)]
mod tests {

    use super::{count_visible_trees, find_max_scenic_score, ParseGridError};

    const BASIC_INPUT: &str = "30373\n25512\n65332\n33549\n35390";

    const INPUT: &str = include_str!("../input");

    #[test]
    fn basic_input_should_result_in_21() -> Result<(), ParseGridError> {
        let result = count_visible_trees(BASIC_INPUT)?;

        assert_eq!(result, 21);

        Ok(())
    }

    #[test]
    fn input_should_result_in_value() -> Result<(), ParseGridError> {
        let result = count_visible_trees(INPUT)?;

        assert_eq!(result, 1560);

        Ok(())
    }

    #[test]
    fn part2_basic_input_should_result_in_8() -> Result<(), ParseGridError> {
        let result = find_max_scenic_score(BASIC_INPUT)?;

        assert_eq!(result, 8);

        Ok(())
    }

    #[test]
    fn part2_input_should_result_in_value() -> Result<(), ParseGridError> {
        let result = find_max_scenic_score(INPUT)?;

        assert_eq!(result, 252000);

        Ok(())
    }
}
