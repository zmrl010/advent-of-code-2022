//! # Grid
//!
//! Module for an implementation of a basic grid container
//!
//! Credit to [this guide](https://blog.adamchalmers.com/grids-1/)
//! for most of the implementation

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
    vec::IntoIter,
};

/// Index for a 2D grid
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

pub trait Touch {
    /// Compare `self` and `other` to determine if they are touching.
    fn is_touching(&self, other: &Self) -> bool;
}

impl Touch for Point {
    /// Compare `self` and `other` to determine if they are touching.
    ///
    /// Touching means that the one point is one step away from the other; including
    /// left, right, up, down, diagonally, and overlapping.
    fn is_touching(&self, other: &Self) -> bool {
        if self == other {
            return true;
        }

        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }
}

/// Container that stores elements at points across a 2D plane
pub trait GridLike<T> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    /// Get the element at a given point
    fn get(&self, p: Point) -> &T;
}

#[derive(Debug)]
pub struct Grid<T> {
    items: Vec<T>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub struct ParseGridError;

impl Display for ParseGridError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ParseGridError: failed to parse Grid")
    }
}

impl Error for ParseGridError {}

impl<T: FromStr> FromStr for Grid<T> {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Result<Vec<_>, _> = s
            .trim()
            .lines()
            .flat_map(|line| {
                line.split("")
                    .filter(|val| *val != "")
                    .map(|val| val.parse::<T>())
            })
            .collect();

        Ok(Self::from(items.map_err(|_| ParseGridError)?))
    }
}

impl<T> Grid<T>
where
    T: Default + Copy,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            items: [T::default()].repeat(width * height),
            width,
            height,
        }
    }
}

impl<T> From<Vec<T>> for Grid<T> {
    fn from(v: Vec<T>) -> Self {
        let width = (v.len() as f64).sqrt().floor() as usize;

        Self {
            items: v,
            width,
            height: width,
        }
    }
}

impl<T> FromIterator<T> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let items: Vec<T> = iter.into_iter().collect();

        let width = (items.len() as f64).sqrt().floor() as usize;

        Self {
            items,
            width,
            height: width,
        }
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T> GridLike<T> for Grid<T> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&self, p: Point) -> &T {
        &self.items[p.y * self.width + p.x]
    }
}
