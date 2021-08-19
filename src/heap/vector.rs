#[cfg(feature = "std")]
use std::{
    boxed::Box,
    convert::From,
    iter::Iterator,
    ops::{Add, AddAssign, Mul},
};

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

#[cfg(not(feature = "std"))]
use core::{
    convert::From,
    iter::Iterator,
    ops::{Add, AddAssign, Mul},
};

use rand::distributions::{Distribution, Uniform};

use crate::Coordinate;

#[derive(Clone, Debug)]
pub struct BoxVecN<const N: usize>(Box<[f64; N]>);

impl<const N: usize> BoxVecN<N> {
    pub fn new() -> Self {
        BoxVecN(Box::new([0.0f64; N]))
    }

    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64> {
        self.0.iter_mut()
    }
}

impl<const N: usize> Coordinate for BoxVecN<N> {
    fn initialize() -> Self {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(-1.0..1.0);
        let mut arr = [0f64; N];
        for n in arr.iter_mut() {
            *n = die.sample(&mut rng);
        }
        Self(Box::new(arr))
    }

    fn distance(&self, other: &Self) -> f64 {
        // @TODO @perf simd
        let mut term = 0.0;
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            term += (a - b) * (a - b);
        }
        term.sqrt()
    }

    fn direction(&self, other: &Self) -> Self {
        // @TODO @perf simd

        // @TODO @perf this allocates (stack) and zeros the array...granted we're talking small
        // arrays so maybe not a big deal
        let mut ret = BoxVecN::new();
        for (n, (s, r)) in ret.0.iter_mut().zip(self.0.iter().zip(other.0.iter())) {
            *n = s - r;
        }
        ret
    }
}

impl<const N: usize> Mul<f64> for BoxVecN<N> {
    type Output = BoxVecN<N>;
    fn mul(self, rhs: f64) -> Self {
        // @TODO @perf simd

        // @TODO @perf this allocates (stack) and zeros the array...granted we're talking small
        // arrays so maybe not a big deal
        let mut ret = BoxVecN::new();
        for (n, s) in ret.0.iter_mut().zip(self.0.iter()) {
            *n = s * rhs;
        }
        ret
    }
}

impl<const N: usize> Add<BoxVecN<N>> for BoxVecN<N> {
    type Output = BoxVecN<N>;
    fn add(self, rhs: BoxVecN<N>) -> Self {
        // @TODO @perf simd

        // @TODO @perf this allocates (stack) and zeros the array...granted we're talking small
        // arrays so maybe not a big deal
        let mut ret = BoxVecN::new();
        for (n, (s, r)) in ret.0.iter_mut().zip(self.0.iter().zip(rhs.0.iter())) {
            *n = s + r;
        }
        ret
    }
}

impl<const N: usize> AddAssign<BoxVecN<N>> for BoxVecN<N> {
    fn add_assign(&mut self, rhs: BoxVecN<N>) {
        // @TODO @perf simd

        for (s, r) in self.0.iter_mut().zip(rhs.0.iter()) {
            *s += r;
        }
    }
}

impl<const N: usize> From<[f64; N]> for BoxVecN<N> {
    fn from(arr: [f64; N]) -> Self {
        BoxVecN(Box::new(arr))
    }
}

impl<const N: usize> From<Box<[f64; N]>> for BoxVecN<N> {
    fn from(b: Box<[f64; N]>) -> Self {
        BoxVecN(b)
    }
}

impl<const N: usize> Default for BoxVecN<N> {
    fn defaut() -> Self {
        BoxVecN::new()
    }
}
