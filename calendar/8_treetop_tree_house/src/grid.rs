//! # Grid
//!
//! Module for an implementation of a basic grid container
//!
//! Credit to [this guide](https://blog.adamchalmers.com/grids-1/)
//! for most of the implementation

use rayon::prelude::*;
use std::vec::IntoIter;

/// Index for a 2D grid
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

/// Container that stores elements at points across a 2D plane
pub trait GridLike<T> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    /// Get the element at a given point
    fn get(&self, p: Point) -> &T;

    /// Set all elements of the grid, using the `setter` function.
    ///
    /// Uses [`rayon`] crate to set elements in parallel
    ///
    /// # Arguments
    ///
    /// * `setter` - [`Fn`] that takes a [`Point`] and returns the value
    /// which should be assigned to the grid at that point.
    fn set_all_parallel<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(Point) -> T,
        T: Send;
}

#[derive(Debug)]
pub struct Grid<T> {
    items: Vec<T>,
    width: usize,
    height: usize,
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

    fn set_all_parallel<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(Point) -> T,
        T: Send,
    {
        let width = self.width;

        self.items.par_iter_mut().enumerate().for_each(|(i, item)| {
            *item = setter(Point {
                x: i % width,
                y: i / width,
            });
        });
    }
}
