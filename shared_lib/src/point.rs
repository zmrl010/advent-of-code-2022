//! # Point
//!
//! Module for [`Point`] structure that signifies a location on a 2D plane

use std::{
    cmp,
    fmt::{self, Display, Formatter},
    ops::{Add, AddAssign, Sub, SubAssign},
};

/// Index on a 2D grid
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point<T = usize> {
    pub x: T,
    pub y: T,
}

impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self { x, y } = self;
        write!(f, "Point({x}, {y})")
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign for Point<T>
where
    T: Copy + Add<Output = T>,
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> SubAssign for Point<T>
where
    T: Copy + Sub<Output = T>,
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

/// Trait
pub trait Touch {}

impl Touch for Point {}

/// Trait for operating between two points in relation to each other
pub trait Relative {
    /// Compare self and other to determine if they are touching
    fn is_touching(&self, other: &Self) -> bool;
    /// Move self in relation to other
    fn move_relative(&mut self, other: &Self);
    /// Get distance between self and other
    fn distance(&self, other: &Self) -> usize;
}

impl Relative for Point {
    fn move_relative(&mut self, other: &Self) {
        while !self.is_touching(other) {
            if self.x != other.x {
                if self.x < other.x {
                    self.x += 1;
                } else if self.x > other.x {
                    self.x -= 1;
                }
            }
            if self.y != other.y {
                if self.y < other.y {
                    self.y += 1;
                } else {
                    self.y -= 1;
                }
            }
        }
    }

    /// Compare `self` and `other` to determine if they are touching.
    ///
    /// Touching means that the one point is one step away from the other; including
    /// left, right, up, down, diagonally, and overlapping.
    fn is_touching(&self, other: &Self) -> bool {
        if self == other {
            return true;
        }

        let x_diff = self.x.abs_diff(other.x);
        let y_diff = self.y.abs_diff(other.y);

        x_diff <= 1 && y_diff <= 1
    }

    fn distance(&self, other: &Self) -> usize {
        let delta_x = self.x.abs_diff(other.x);
        let delta_y = self.y.abs_diff(other.y);

        cmp::max(delta_x, delta_y)
    }
}
