//! # `violin`
//!
//! ![Rust Version][rustc-image]
//! [![crates.io][crate-image]][crate-link]
//! [![Documentation][docs-image]][docs-link]
//! [![Dependency Status][deps-image]][deps-link]
//!
//! A Rust `no_std` no `alloc` implementation of the [Vivaldi algorithm][1](PDF)
//! for a network coordinate system.
//!
//! A network coordinate system allows nodes to accurately estimate network
//! latencies by merely exchanging coordinates.
//!
//!
//! <!-- vim-markdown-toc GFM -->
//!
//! * [Violin - The Pitch](#violin---the-pitch)
//! * [Violin - The Anit-Pitch](#violin---the-anit-pitch)
//! * [Compile from Source](#compile-from-source)
//! * [Usage](#usage)
//! * [Benchmarks](#benchmarks)
//!     * [Notes on `no_std` Performance](#notes-on-no_std-performance)
//! * [License](#license)
//!     * [Contribution](#contribution)
//! * [Related Papers and Research](#related-papers-and-research)
//!
//! <!-- vim-markdown-toc -->
//!
//! ## Violin - The Pitch
//!
//! Violin is an implementation of Vivaldi network coordinates that works in
//! `no_std` and no `alloc` environments. Each coordinate is small consisting of
//! a dimensional vector made up of an array of `f64`s. The arrays use const
//! generics, so they can be as small as a single f64 or large as one needs.
//! Although above a certain dimension there are diminishing returns.
//!
//! Nodes can measure real latencies between an origin node, or each-other to
//! adjust their coordinates in space.
//!
//! The real power comes from being able to calculate distance between a remote
//! coordinate without ever having done a real latency check. For example node
//! `A` measures against node `Origin`, node `B` does the same. Then `A` can be
//! given the coordinates to `B` and accurately estimate the latency without
//! ever having measured `B` directly.
//!
//! ## Violin - The Anit-Pitch
//!
//! Vivaldi isn't a magic bullet and still requires measuring real latencies to
//! adjust the coordinates. In a naive implementation, conducting a latency
//! check prior to a coordinate calculation is not much better than just using
//! the latency check directly as the answer. However, this is not how it's
//! supposed to be used.
//!
//! Transferring a Violin coordinate in practice can be comparable data to a
//! small set of ICMP messages. For example an 8-Dimension coordinate (plus
//! three additional `f64`s of metadata) is 88 bytes. However, unlike ICMP
//! messages, the Violin coordinates are a single transmission and only need to
//! be re-transmitted on significant change. Work could even be done to only
//! transmit deltas as well.
//!
//! ## Compile from Source
//!
//! Ensure you have a [Rust toolchain installed][rustup].
//!
//! ```notrust
//! $ git clone https://github.com/kbknapp/violin
//! $ cd violin
//! $ RUSTFLAGS='-Ctarget-cpu=native' cargo build --release
//! ```
//!
//! **NOTE:** The `RUSTFLAGS` can be omitted. However, if on a recent CPU that
//! supports SIMD instructions, and the code will be run on the same CPU it's
//! compiled for, including this flag can improve performance.
//!
//! ## Usage
//!
//! See the `examples/` directory in this repository for complete details,
//! although at quick glance creating three coordinates (`origin`, `a` and `b`)
//! and updating `a` and `b`'s coordinate from experienced real latency would
//! look like this:
//!
//! ```notrust
//! use std::time::Duration;
//! use violin::{heapless::VecD, Coord, Node};
//!
//! // Create two nodes and an "origin" coordinate, all using a 4-Dimensional
//! // coordinate. `VecD` is a dimensional vector.
//! let origin = Coord::<VecD<4>>::rand();
//! let mut a = Node::<VecD<4>>::rand();
//! let mut b = Node::<VecD<4>>::rand();
//!
//! // **conduct some latency measurement from a to origin**
//! // let's assume we observed a value of `0.2` seconds...
//! //
//! // **conduct some latency measurement from b to origin**
//! // let's assume we observed a value of `0.03` seconds...
//!
//! a.update(Duration::from_secs_f64(0.2), &origin);
//! b.update(Duration::from_secs_f64(0.03), &origin);
//!
//! // Estimate from a to b even though we never measured them directly
//! println!(
//!     "a's estimate to b: {:.2}ms",
//!     a.distance_to(&b.coordinate()).as_millis()
//! );
//! ```
//!
//! ## Benchmarks
//!
//! A set of benchmarks are included using 8D, 4D, and 2D coordinates both using
//! `heap::VecD` (requires the `alloc` feature) and `heapless::VecD`.
//!
//! The benchmarks measure both the higher level `Node` as well as a lower level
//! `Coord` abstractions.
//!
//! To measure we create 10,000 coordinates and the coordinates are
//! update for each coordinate 100 times, totaling 1,000,000 updates.
//!
//! On my 8 core AMD Ryzen 7 5850U laptop with 16GB RAM the benchmarks look as
//! follows:
//!
//! | Abstraction | Memory   | Dimensions | Time |
//! | :-: | :-:      | :-:        | :-:  |
//! | `Node` | heap     | 8          | 66.537 ms |
//! | `Coord` | heap     | 8          | 55.402 ms |
//! | `Node` | heapless | 8          | 24.997 ms |
//! | `Coord` | heapless | 8          | 16.552 ms |
//! | `Node` | heap     | 4          | 49.501 ms |
//! | `Coord` | heap     | 4          | 39.163 ms |
//! | `Node` | heapless | 4          | 16.795 ms |
//! | `Coord` | heapless | 4          | 11.780 ms |
//! | `Node` | heap     | 2          | 54.363 ms |
//! | `Coord` | heap     | 2          | 46.001 ms |
//! | `Node` | heapless | 2          | 13.181 ms |
//! | `Coord` | heapless | 2          | 10.916 ms |
//!
//! To run the benchmarks yourself use `RUSTFLAGS='-Ctarget-cpu=native' cargo
//! bench`.
//!
//! ### Notes on `no_std` Performance
//!
//! The `no_std` version is _much_ slower because it cannot use platform
//! intrinsics for square roots, floating point rounding, etc. Instead these
//! functions had to be hand written.
//!
//! Additionally, the `no_std` square root functions round up to 8 decimals of
//! precision.
//!
//! One should realistically only use the `no_std` version when there is a good
//! reason to do so, such as an embedded device that absolutely does not support
//! `std`.
//!
//! A single Vivaldi calculation only requires one square root calculation per
//! distance estimate. So pragmatically, it should be rare where such a device
//! is _also_ needing to calculate thousands of square root operations per
//! second.
//!
//! ## License
//!
//! This crate is licensed under either of
//!
//!  * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
//!  * [MIT license](http://opensource.org/licenses/MIT)
//!
//! at your option.
//!
//! ### Contribution
//!
//! Unless you explicitly Node otherwise, any contribution intentionally
//! submitted for inclusion in the work by you, as defined in the Apache-2.0
//! license, shall be dual licensed as above, without any additional terms or
//! conditions.
//!
//! ## Related Papers and Research
//!
//! - [Vivaldi - A Decentralized Network Coordinate System][1](PDF)
//! - [Network Coordinates in the Wild][2](PDF)
//! - [Towards Network Triangle Inequality Violation Aware Distributed
//!   Systems][3](PDF)
//! - [On Suitability of Euclidean Embedding for Host-based Network Coordinate
//!   Systems][4](PDF)
//! - [Practical, Distributed Network Coordinates][5](PDF)
//! - [Armon Dadgar on Vivaldi: Decentralized Network Coordinate
//!   System][6](Video)
//!
//! [//]: # (badges)
//!
//! [rustc-image]: https://img.shields.io/badge/rustc-1.59+-blue.svg
//! [crate-image]: https://img.shields.io/crates/v/violin.svg
//! [crate-link]: https://crates.io/crates/violin
//! [docs-image]: https://docs.rs/violin/badge.svg
//! [docs-link]: https://docs.rs/violin
//! [deps-image]: https://deps.rs/repo/github/kbknapp/violin/status.svg
//! [deps-link]: https://deps.rs/repo/github/kbknapp/violin
//!
//! [//]: # (links)
//!
//! [rustup]: https://rustup.rs
//! [1]: https://pdos.csail.mit.edu/papers/vivaldi:sigcomm/paper.pdf
//! [2]: https://www.usenix.org/legacy/event/nsdi07/tech/full_papers/ledlie/ledlie.pdf
//! [3]: https://www.cs.rice.edu/~eugeneng/papers/IMC07.pdf
//! [4]: https://www-users.cse.umn.edu/~zhang089/Papers/Lee-Suitability-tonfinal.pdf
//! [5]: http://www.news.cs.nyu.edu/~jinyang/pub/hotnets03.pdf
//! [6]: https://youtu.be/AszPoJjWK9Q?t=1690
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_allocation,
    trivial_numeric_casts
)]
#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

// When we're building for a no-std target, we pull in `core`, but alias
// it as `std` so the `use` statements are the same between `std` and `core`.
#[cfg(all(not(feature = "std"), not(test)))]
#[macro_use]
extern crate core as std;

#[cfg(feature = "alloc")]
extern crate alloc;

use crate::std::{
    default::Default,
    ops::{Add, AddAssign, Div, Mul},
};

#[macro_use]
mod macros;
mod coord;
pub mod error;
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod heap;
pub mod heapless;
mod node;

pub use coord::Coord;
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub use heap::VecD;
pub use node::{Config, Node};

/// Determines at what threshold two coordinates overlap
const OVERLAP_THRESHOLD: f64 = 1.0e-6;
const DEFAULT_HEIGHT_MIN: f64 = 0.0;

/// The abstraction over coordinate vectors
pub trait Vector:
    Default
    + Add<Self, Output = Self>
    + Mul<f64, Output = Self>
    + AddAssign<Self>
    + Div<f64, Output = Self>
    + AsRef<[f64]>
    + AsMut<[f64]>
where
    Self: Sized,
{
    /// The length of the vector
    const LEN: usize;

    /// Returns a unit vector (`â`) from `other` pointing at `self` along
    /// with the magnitude of the difference beand tween both vectors
    fn unit_vector_from(&self, other: &Self) -> (f64, Self) {
        let diff = self.difference(other);
        let mag = diff.magnitude();
        // If the coordinates overlap return a unit vector in the first dimension
        if mag < OVERLAP_THRESHOLD {
            let mut ret = Self::default();
            ret.as_mut()[0] = 1.0;
            return (0.0, ret);
        }
        (mag, diff * (1. / mag))
    }

    /// Returns distance between `self` and `other`
    #[cfg_attr(feature = "std", doc = "```rust")]
    #[cfg_attr(not(feature = "std"), doc = "```no_run")]
    /// use violin::{heapless::VecD, Vector};
    ///
    /// let a = VecD::from([1., 0., 5.]);
    /// let b = VecD::from([0., 2., 4.]);
    ///
    /// assert_eq!(a.distance(&b), 2.449489742783178);
    /// ```
    fn distance(&self, other: &Self) -> f64 { self.difference(other).magnitude() }

    /// Returns the difference between two vectors
    ///
    /// ```rust
    /// use violin::{heapless::VecD, Vector};
    ///
    /// let a = VecD::from([1.0, -3.0, 3.0]);
    /// let b = VecD::from([-4.0, 5.0, 6.0]);
    ///
    /// assert_eq!(a.difference(&b), VecD::from([5.0, -8.0, -3.0]));
    /// assert_eq!(a.difference(&VecD::default()), a);
    fn difference(&self, other: &Self) -> Self;

    /// Returns the magnitude of the vector `v` (`|v|`) represented by `self`
    ///
    /// ```rust
    /// use violin::{heapless::VecD, Vector};
    ///
    /// let a = VecD::from([1.0, -2.0, 3.0]);
    /// let b = VecD::from([-2., 4., -4.]);
    /// assert_eq!(a.magnitude(), 3.7416573867739413);
    /// assert_eq!(b.magnitude(), 6.0f64);
    /// ```
    #[cfg(feature = "std")]
    fn magnitude(&self) -> f64 { self.magnitude2().sqrt() }

    /// Returns the magnitude of the vector `v` (`|v|`) represented by `self`
    #[cfg_attr(feature = "std", doc = "```rust")]
    #[cfg_attr(not(feature = "std"), doc = "```no_run")]
    /// use violin::{heapless::VecD, Vector};
    ///
    /// let a = VecD::from([1.0, -2.0, 3.0]);
    /// let b = VecD::from([-2., 4., -4.]);
    /// assert_eq!(a.magnitude(), 3.7416573867739413);
    /// assert_eq!(b.magnitude(), 6.0f64);
    /// ```
    #[cfg(not(feature = "std"))]
    fn magnitude(&self) -> f64 { _sqrt(self.magnitude2()) }

    /// Returns the magnitude of the vector `v` (`|v|`) represented by `self`
    /// **without** performing the expensive square root operation
    /// ```rust
    /// use violin::{heapless::VecD, Vector};
    ///
    /// let a = VecD::from([1.0, -2.0, 3.0]);
    /// let b = VecD::from([-2., 4., -4.]);
    /// let c = VecD::from([0., 0., 0.]);
    /// assert_eq!(a.magnitude2(), 14.0);
    /// assert_eq!(b.magnitude2(), 36.0);
    /// assert_eq!(c.magnitude2(), 0.0);
    /// ```
    fn magnitude2(&self) -> f64;
}

#[cfg(not(feature = "std"))]
const PRECISION_INC: f64 = 1.0e-8;
#[cfg(not(feature = "std"))]
const PRECISION_POW: f64 = 1.0e+8;

#[cfg(not(feature = "std"))]
fn _sqrt(n: f64) -> f64 {
    if n == 0.0 {
        return 0.0;
    }
    let mut ans;
    let mut last;
    let mut mid = n / 2.0;
    let mut top = n;
    let r_n = _round(n);

    loop {
        last = mid;
        let sq = _round(mid * mid);
        if sq == r_n {
            ans = mid;
            break;
        }
        if sq > r_n {
            mid = last / 2.0;
            top = last;
        } else {
            mid += (top - mid) / 2.0;
        }
        if mid == last {
            mid += 1.0;
        }
    }

    ans = f64::max(ans - 1.0, 0.0);
    mid = 0.5;
    loop {
        last = mid;
        let sq = (mid + ans) * (mid + ans);
        let r_sq = _round(sq);
        if r_sq == r_n {
            ans += mid;
            break;
        }
        if sq > n {
            mid = last / 2.0;
            top = last;
        } else {
            mid += (top - mid) / 2.0;
        }

        if mid == last {
            mid += PRECISION_INC;
        }
    }

    _round(ans)
}

#[inline(always)]
#[cfg(not(feature = "std"))]
fn _round(n: f64) -> f64 { _ceil((n * PRECISION_POW) - 0.49999999) / PRECISION_POW }

#[inline(always)]
#[cfg(not(feature = "std"))]
fn _ceil(n: f64) -> f64 {
    let n_int = n as u64;
    if n > n_int as f64 {
        (n_int + 1) as f64
    } else {
        n
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "std"))]
    use super::*;

    #[cfg_attr(not(feature = "std"), test)]
    #[cfg(not(feature = "std"))]
    fn test_sqrt() {
        assert_eq!(_sqrt(36.0), 6.0);
        assert_eq!(_sqrt(27.2934), 5.22430857);
        assert_eq!(_sqrt(8.408207478410603), 2.89969093);
        assert_eq!(_sqrt(158.57), 12.59245806);
        assert_eq!(_sqrt(0.0), 0.0);
        assert_eq!(_sqrt(0.0), 0.0);
        assert_eq!(_sqrt(0.009983505350056349), 0.0999175);
    }
}
