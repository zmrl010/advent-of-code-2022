//! # Grid
//!
//! Module for an implementation of a basic grid container
//!
//! Credit to [this guide](https://blog.adamchalmers.com/grids-1/)
//! for most of the implementation

/// Index for a 2D grid
#[derive(Clone, Copy)]
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
        use rayon::prelude::*;

        let width = self.width;

        self.items.par_iter_mut().enumerate().for_each(|(i, item)| {
            *item = setter(Point {
                x: i % width,
                y: i / width,
            });
        });
    }
}
