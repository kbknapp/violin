#![deny(
 //   missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_allocation,
    trivial_numeric_casts
)]
#![forbid(unsafe_code)]

#[cfg(feature = "std")]
use std::default::Default;

#[cfg(not(feature = "std"))]
use core::default::Default;

#[cfg(feature = "std")]
use std::ops::{Add, AddAssign, Mul};

#[cfg(not(feature = "std"))]
use core::ops::{Add, AddAssign, Mul};

#[cfg(feautre = "alloc")]
pub mod heap;
pub mod heapless;
mod node;

#[cfg(feautre = "alloc")]
pub use heap::BoxVecN;
pub use heapless::VecN;
pub use node::Node;
#[cfg(feautre = "builder")]
pub use node::NodeBuilder;

pub trait Coordinate:
    Default + Add<Self, Output = Self> + Mul<f64, Output = Self> + AddAssign<Self>
{
    fn direction(&self, other: &Self) -> Self;
    fn distance(&self, other: &Self) -> f64;
    fn initialize() -> Self;
}
