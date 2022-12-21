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

use crate::point::Point;

/// Container that stores elements at points across a 2D plane
pub trait GridLike<T> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    /// Get the element at a given point
    fn get(&self, p: Point) -> &T;

    // /// Find an item in the grid, returning [`Some(value)`] if found, else [`None`]
    // fn find(&self, item: &T) -> Option<&T>;
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

        let size = (items.len() as f64).sqrt().floor() as usize;

        Self {
            items,
            width: size,
            height: size,
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
