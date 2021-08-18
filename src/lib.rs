#![deny(
 //   missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_allocation,
    trivial_numeric_casts
)]
#![forbid(unsafe_code)]

#[cfg(feautre = "alloc")]
pub mod heap;
pub mod heapless;
mod point;
mod state;

#[cfg(feautre = "alloc")]
pub use heap::BoxVecN;
pub use heapless::VecN;
pub use state::StateBuilder;

// pub type Point<const N: usize> = point::Point<VecN<N>, N>;
// pub type State<const N: usize> = state::State<Point<N>, N>;

// #[cfg(feature = "alloc")]
// pub type BoxPoint<const N: usize> = point::Point<BoxVecN<N>, N>;
// #[cfg(feature = "alloc")]
// pub type BoxState<const N: usize> = state::State<BoxPoint<N>, N>;

#[cfg(feature = "std")]
use std::default::Default;

#[cfg(not(feature = "std"))]
use core::default::Default;

#[cfg(feature = "std")]
use std::ops::{Add, AddAssign, Mul};

#[cfg(not(feature = "std"))]
use core::ops::{Add, AddAssign, Mul};

pub trait Coordinate:
    Default + Add<Self, Output = Self> + Mul<f64, Output = Self> + AddAssign<Self>
{
    fn direction(&self, other: &Self) -> Self;
    fn distance(&self, other: &Self) -> f64;
    fn from_arr<const N: usize>(arr: [f64; N]) -> Self;
}
