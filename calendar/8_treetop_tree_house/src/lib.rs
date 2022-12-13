mod grid;

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    num::ParseIntError,
    str::FromStr,
};

use grid::{Grid, GridLike, Point};

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

#[derive(Debug)]
pub struct ParseTreeGridError(ParseTreeError);

impl Display for ParseTreeGridError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ParseTreeGridError: failed to parse `Grid<Tree>`\n\n{}",
            self.0
        )
    }
}

impl From<ParseTreeError> for ParseTreeGridError {
    fn from(err: ParseTreeError) -> Self {
        ParseTreeGridError(err)
    }
}

impl Error for ParseTreeGridError {}

impl FromStr for Grid<Tree> {
    type Err = ParseTreeGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Result<Vec<Tree>, _> = s
            .trim()
            .lines()
            .flat_map(|line| {
                line.split("")
                    .filter(|val| *val != "")
                    .map(|val| val.parse::<Tree>())
            })
            .collect();

        Ok(Self::from(items?))
    }
}

impl Grid<Tree> {
    pub fn check_tree_visibility(&self, point: Point) -> bool {
        let (width, height) = (self.width(), self.height());
        if point.x == 0 || point.y == 0 || point.x == width - 1 || point.y == height - 1 {
            return true;
        }

        let target_tree = self.get(point);

        (0..width).all(|x| {
            let tree = self.get(Point { x, y: point.y });

            target_tree >= tree
        }) && (0..height).all(|y| {
            let tree = self.get(Point { x: point.x, y });

            target_tree >= tree
        })
    }
}

impl Tree {
    pub fn new(height: u8) -> Self {
        Self { height }
    }
}

fn count_visible_trees(input: &str) -> Result<u64, ParseTreeGridError> {
    let grid = input.parse::<Grid<Tree>>()?;

    let grid = dbg!(grid);

    let mut count = 0;

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let tree_point = Point { x, y };
            let tree_is_visible = grid.check_tree_visibility(tree_point);

            if tree_is_visible {
                count += 1;
            }
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use crate::grid::{GridLike, Point};

    use super::{count_visible_trees, Grid, ParseTreeGridError, Tree};

    const BASIC_INPUT: &str = "30373\n25512\n65332\n33549\n35390";

    const INPUT: &str = include_str!("../input");

    #[test]
    fn basic_input_should_result_in_21() -> Result<(), ParseTreeGridError> {
        let result = count_visible_trees(BASIC_INPUT)?;

        assert_eq!(result, 21);

        Ok(())
    }
}
