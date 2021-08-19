#[cfg(feature = "std")]
use std::{
    convert::From,
    iter::Iterator,
    ops::{Add, AddAssign, Mul},
};

#[cfg(not(feature = "std"))]
use core::{
    convert::From,
    iter::Iterator,
    ops::{Add, AddAssign, Mul},
};

use rand::distributions::{Distribution, Uniform};

use crate::Coordinate;

#[derive(Clone, Debug)]
pub struct VecN<const N: usize>([f64; N]);

impl<const N: usize> VecN<N> {
    pub fn new() -> Self {
        VecN([0.0f64; N])
    }

    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64> {
        self.0.iter_mut()
    }
}

impl<const N: usize> Coordinate for VecN<N> {
    fn initialize() -> Self {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(-1.0..1.0);
        let mut arr = [0f64; N];
        for n in arr.iter_mut() {
            *n = die.sample(&mut rng);
        }
        Self(arr)
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
        let mut ret = VecN::new();
        for (n, (s, r)) in ret.0.iter_mut().zip(self.0.iter().zip(other.0.iter())) {
            *n = s - r;
        }
        ret
    }
}

impl<const N: usize> Mul<f64> for VecN<N> {
    type Output = VecN<N>;
    fn mul(self, rhs: f64) -> Self {
        // @TODO @perf simd

        // @TODO @perf this allocates (stack) and zeros the array...granted we're talking small
        // arrays so maybe not a big deal
        let mut ret = VecN::new();
        for (n, s) in ret.0.iter_mut().zip(self.0.iter()) {
            *n = s * rhs;
        }
        ret
    }
}

impl<const N: usize> Add<VecN<N>> for VecN<N> {
    type Output = VecN<N>;
    fn add(self, rhs: VecN<N>) -> Self {
        // @TODO @perf simd

        // @TODO @perf this allocates (stack) and zeros the array...granted we're talking small
        // arrays so maybe not a big deal
        let mut ret = VecN::new();
        for (n, (s, r)) in ret.0.iter_mut().zip(self.0.iter().zip(rhs.0.iter())) {
            *n = s + r;
        }
        ret
    }
}

impl<const N: usize> AddAssign<VecN<N>> for VecN<N> {
    fn add_assign(&mut self, rhs: VecN<N>) {
        // @TODO @perf simd

        for (s, r) in self.0.iter_mut().zip(rhs.0.iter()) {
            *s += r;
        }
    }
}

impl<const N: usize> From<[f64; N]> for VecN<N> {
    fn from(arr: [f64; N]) -> Self {
        VecN(arr)
    }
}

impl<const N: usize> Default for VecN<N> {
    fn default() -> Self {
        VecN::new()
    }
}
