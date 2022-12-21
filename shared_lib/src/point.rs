//! # Point
//!
//! Module for [`Point`] structure that signifies a location on a 2D plane

use std::{
    fmt::{self, Display, Formatter},
    ops::{Add, AddAssign, Sub, SubAssign},
};

use num::{self, traits::NumAssignOps, Integer, Signed};

/// Index on a 2D grid
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point<T = usize> {
    pub x: T,
    pub y: T,
}

impl<T: AddAssign + Copy> Point<T> {
    /// Move point by adding another to it
    pub fn move_add(&mut self, point: &Point<T>) {
        self.x += point.x;
        self.y += point.y;
    }
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

/// Trait for operating between two points in relation to each other
pub trait Relative {
    /// Compare self and other to determine if they are touching
    fn is_touching(&self, other: &Self) -> bool;
    /// Move self in relation to other
    fn move_relative(&mut self, other: &Self);
}

impl<T> Relative for Point<T>
where
    T: NumAssignOps<T> + Integer + Signed + Clone,
{
    // impl Relative for Point<isize> {
    fn move_relative(&mut self, other: &Self) {
        while !self.is_touching(other) {
            if self.x != other.x {
                if self.x < other.x {
                    self.x += T::one();
                } else if self.x > other.x {
                    self.x -= T::one();
                }
            }
            if self.y != other.y {
                if self.y < other.y {
                    self.y += T::one();
                } else {
                    self.y -= T::one();
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

        let x_diff = num::abs_sub(self.x.clone(), other.x.clone());
        let y_diff = num::abs_sub(self.y.clone(), other.y.clone());

        x_diff <= T::one() && y_diff <= T::one()
    }
}
