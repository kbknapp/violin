#[cfg(not(feature = "std"))]
use core::{
    default::Default,
    ops::{Add, AddAssign, Mul},
};

#[cfg(feature = "std")]
use std::{
    default::Default,
    ops::{Add, AddAssign, Mul},
};

use rand::distributions::{Distribution, Uniform};

use crate::Coordinate;

#[derive(Debug, Clone)]
pub struct Point<T> {
    vec: T,
}

impl<T> Point<T>
where
    T: Coordinate,
{
    pub fn new() -> Self {
        let mut arr = T::default();
        let mut rng = rand::thread_rng();
        let die = Uniform::from(-1.0..1.0);
        for n in arr.iter_mut() {
            *n = die.sample(&mut rng);
        }
        Self { vec: T::from(arr) }
    }
}

impl<T> Default for Point<T>
where
    T: Coordinate,
{
    fn default() -> Self {
        Point::new()
    }
}

impl<T> Coordinate for Point<T>
where
    T: Coordinate,
{
    fn direction(&self, other: &Self) -> Self {
        Point {
            vec: self.vec.direction(&other.vec),
        }
    }

    fn distance(&self, other: &Self) -> f64 {
        self.vec.distance(&other.vec)
    }
}

impl<T, const N: usize> From<[f64; N]> for Point<T>
where
    T: Coordinate,
{
    fn from(arr: [f64; N]) -> Self {
        Self { vec: T::from(arr) }
    }
}

impl<T> Mul<f64> for Point<T>
where
    T: Coordinate,
{
    type Output = Self;
    fn mul(mut self, rhs: f64) -> Self {
        self.vec = self.vec * rhs;
        self
    }
}

impl<T> Add<Point<T>> for Point<T>
where
    T: Coordinate,
{
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self.vec = self.vec + rhs.vec;
        self
    }
}

impl<T> AddAssign<Point<T>> for Point<T>
where
    T: Coordinate,
{
    fn add_assign(&mut self, rhs: Self) {
        self.vec += rhs.vec;
    }
}
