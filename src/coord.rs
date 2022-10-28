#[cfg(all(feature = "std", feature = "rand"))]
use rand::distributions::Distribution;

#[cfg(feature = "alloc")]
use crate::VecD;
use crate::{heapless, Config, Vector, OVERLAP_THRESHOLD};

/// A network coordinate consisting of a dimensional vector, and some metadata
#[derive(Debug)]
pub struct Coord<T> {
    /// The dimensional vector
    pub(crate) vec: T,
    /// An error estimate for this coordinate, which is a confidence level. Low
    /// error_estimates will result in less adjust when these coordinates
    /// are given as the `other` to other coordinates.
    pub(crate) error_estimate: f64,
    /// Using a height can increase the accuracy accounting for network
    /// anomalies. Height is handled automatically as part of the update and
    /// adjustment calculations.
    pub(crate) height: f64,
    /// Positive manual additions to distance calculations. Negative offsets are
    /// ignored
    pub(crate) offset: f64,
}

impl<T> Clone for Coord<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            vec: self.vec.clone(),
            error_estimate: self.error_estimate,
            height: self.height,
            offset: self.offset,
        }
    }
}

impl<T> Default for Coord<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            vec: T::default(),
            error_estimate: OVERLAP_THRESHOLD,
            height: 0.0,
            offset: 0.0,
        }
    }
}

impl<T> Coord<T>
where
    T: Vector,
{
    /// Create a new default coordinate vector
    pub fn new() -> Self {
        Self {
            vec: T::default(),
            ..Default::default()
        }
    }

    /// Create a new node with an initialized random coordinate vector. This is
    /// useful so that coordinates don't start out overlapping one another.
    #[cfg(all(feature = "std", feature = "rand"))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "std", feature = "rand"))))]
    pub fn rand() -> Self {
        let mut vec = T::default();
        let mut rng = rand::thread_rng();
        let die = rand::distributions::Uniform::from(-1.0..1.0);
        for n in vec.as_mut() {
            *n = die.sample(&mut rng);
        }

        Self {
            vec,
            ..Default::default()
        }
    }

    /// Estimate the distance between this coordinate and the other
    /// coordinate's vector coordinate, adding any positive offset from
    /// either coordinate
    #[cfg_attr(feature = "std", doc = "```rust")]
    #[cfg_attr(not(feature = "std"), doc = "```no_run")]
    /// use violin::{heapless::VecD, Coord};
    ///
    /// let mut c1 = Coord::from(VecD::from([2.3, 3.2, 4.1]));
    /// c1.set_offset(8.0);
    /// let c2 = Coord::from(VecD::from([4.5, -6.1, -4.1]));
    /// assert_eq!(c1.distance_to(&c1), 16.0);
    /// assert_eq!(c1.distance_to(&c2), c2.distance_to(&c1));
    /// assert_eq!(c1.distance_to(&c2), 20.592458060283544);
    /// ```
    pub fn distance_to(&self, other: &Coord<T>) -> f64 {
        self.raw_distance_to(other) + self.offset + other.offset
    }

    /// Estimate the distance between this coordinate and the other
    /// coordinate's vector coordinate, _without_ adding any positive offset
    /// from either coordinate. However, height is always included.
    #[cfg_attr(feature = "std", doc = "```rust")]
    #[cfg_attr(not(feature = "std"), doc = "```no_run")]
    /// use violin::{heapless::VecD, Coord};
    ///
    /// let mut c1 = Coord::from(VecD::from([2.3, 3.2, 4.1]));
    /// // this should be ignored
    /// c1.set_offset(8.0);
    /// let c2 = Coord::from(VecD::from([4.5, -6.1, -4.1]));
    /// assert_eq!(c1.raw_distance_to(&c1), 0.0);
    /// assert_eq!(c1.raw_distance_to(&c2), c2.raw_distance_to(&c1));
    /// assert_eq!(c1.raw_distance_to(&c2), 12.592458060283544);
    /// ```
    pub fn raw_distance_to(&self, other: &Coord<T>) -> f64 {
        self.vec.distance(&other.vec) + self.height + other.height
    }

    /// Returns the raw coordinate vector
    pub fn raw_coord(&self) -> &T { &self.vec }

    /// Returns the raw height
    pub fn height(&self) -> f64 { self.height }

    /// Set the raw height
    pub fn set_height(&mut self, height: f64) { self.height = height; }

    /// Returns the raw offset
    pub fn offset(&self) -> f64 { self.offset }

    /// Set the raw offset. Negative offsets will be ignored and effectively
    /// reset the offset to `0.0`
    pub fn set_offset(&mut self, offset: f64) { self.offset = f64::max(0.0, offset); }

    /// Returns true of all values of the coordinates inner vector are neither
    /// NaN or Infinite
    pub fn is_finite(&self) -> bool { self.vec.as_ref().iter().all(|f| f.is_finite()) }

    /// Update the node's coordinate based off the RTT (in seconds) of the
    /// `other` coordinate.
    ///
    /// A high `other` error estimate will reduce the "force" applied to this
    /// coordinate's movement (i.e. it will move less because the `other` is
    /// asserting that it is less confident in the accuracy
    /// of it's coordinate position.)
    ///
    /// # Panics
    ///
    /// Panics if any:
    ///
    /// - `rtt <= 0.0`
    /// - This coordinate's AND the other's error estimate `<= 0.0`
    pub fn update(&mut self, rtt: f64, other: &Coord<T>, cfg: &Config) {
        // @TODO assert or Err()?
        assert!(self.error_estimate > 0.0 || other.error_estimate > 0.0);
        assert!(rtt > 0.0);

        // Sample weight balances local and other error
        //  - A high local error = greater movement
        //  - A high other error = less movement
        let err_weight = self.error_estimate / (self.error_estimate + other.error_estimate);

        // Compute relative error of this sample.
        let dist = self.vec.distance(&other.vec);
        let err = f64::max(dist - rtt, 0.0) / rtt;

        // Update weighted moving average of local error
        self.error_estimate =
            err * cfg.ce * err_weight + self.error_estimate * (1.0 - cfg.ce * err_weight);

        // Update local coordinates
        let delta = cfg.cc * err_weight;
        let force = delta * (rtt - dist);
        self.apply_force_from(other, force, cfg);
    }

    /// Gravity pulls the coordinate back toward the origin to prevent drift
    pub fn apply_gravity(&mut self, origin: &Coord<T>, cfg: &Config) {
        let dist = self.distance_to(origin);
        let rel_grav = dist / cfg.gravity_rho;
        let force = -1.0 * (rel_grav * rel_grav);
        self.apply_force_from(origin, force, cfg);
    }

    fn apply_force_from(&mut self, other: &Coord<T>, force: f64, cfg: &Config) {
        self.height = f64::max(self.height, cfg.height_min);
        let (mag, uvec) = self.vec.unit_vector_from(&other.vec);
        self.vec += uvec * force;
        if mag > OVERLAP_THRESHOLD {
            self.height = (self.height) + (force * (self.height / mag));
            self.height = f64::max(self.height, cfg.height_min);
        }
    }
}

impl<const N: usize, T> From<T> for Coord<heapless::VecD<N>>
where
    T: Into<heapless::VecD<N>>,
{
    fn from(vec: T) -> Self {
        Self {
            vec: vec.into(),
            ..Default::default()
        }
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, T> From<T> for Coord<VecD<N>>
where
    T: Into<VecD<N>>,
{
    fn from(vec: T) -> Self {
        Self {
            vec: vec.into(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(feature = "alloc"))]
    use crate::heapless::VecD;

    #[test]
    fn apply_force_from() {
        let cfg = Config::default();
        let mut origin = Coord::new();
        let above = Coord::from(VecD::from([0.0, 0.0, 2.9]));
        origin.apply_force_from(&above, 5.3, &cfg);
        assert_eq!(origin.raw_coord().as_ref(), &[0.0, 0.0, -5.3]);

        // adjust and re-calcright
        let right = Coord::from(VecD::from([3.4, 0.0, -5.3]));
        origin.apply_force_from(&right, 2.0, &cfg);
        assert_eq!(origin.raw_coord().as_ref(), &[-2.0, 0.0, -5.3]);
    }

    #[test]
    fn unit_vec_div_zero() {
        let cfg = Config::default();
        let mut c1 = Coord::<VecD<3>>::new();
        let c2 = Coord::new();
        c1.apply_force_from(&c2, 1.0, &cfg);
        assert_eq!(c2.distance_to(&c1), 1.0);
    }

    #[test]
    fn min_height_factor() {
        let cfg = Config {
            height_min: 0.01,
            ..Default::default()
        };
        let mut origin = Coord::new();
        let above = Coord::from(VecD::from([0.0, 0.0, 2.9]));
        origin.apply_force_from(&above, 5.3, &cfg);
        assert_eq!(origin.raw_coord().as_ref(), &[0.0, 0.0, -5.3]);
        assert_eq!(origin.height, 0.028_275_862_068_965_52);
    }

    #[test]
    fn min_height() {
        let cfg = Config {
            height_min: 10.0,
            ..Default::default()
        };
        let mut origin = Coord::new();
        let above = Coord::from(VecD::from([0.0, 0.0, 2.9]));
        origin.apply_force_from(&above, -5.3, &cfg);
        assert_eq!(origin.raw_coord().as_ref(), &[0.0, 0.0, 5.3]);
        assert_eq!(origin.height, cfg.height_min);
    }

    #[test]
    fn distance_to() {
        let c1 = Coord::from(VecD::from([2.3, 3.2, 4.1]));
        let c2 = Coord::from(VecD::from([4.5, -6.1, -4.1]));
        assert_eq!(c1.distance_to(&c1), 0.0);
        assert_eq!(c1.distance_to(&c2), c2.distance_to(&c1));
        #[cfg(feature = "std")]
        assert_eq!(c1.distance_to(&c2), 12.592458060283544);
        #[cfg(not(feature = "std"))]
        assert_eq!(c1.distance_to(&c2), 12.59245806);
    }

    #[test]
    fn raw_distance_to() {
        let mut c1 = Coord::from(VecD::from([2.3, 3.2, 4.1]));
        c1.set_offset(8.0);
        let c2 = Coord::from(VecD::from([4.5, -6.1, -4.1]));
        assert_eq!(c1.raw_distance_to(&c1), 0.0);
        assert_eq!(c1.raw_distance_to(&c2), c2.raw_distance_to(&c1));
        #[cfg(feature = "std")]
        assert_eq!(c1.raw_distance_to(&c2), 12.592458060283544);
        #[cfg(not(feature = "std"))]
        assert_eq!(c1.raw_distance_to(&c2), 12.59245806);
    }

    #[test]
    fn raw_distance_to_with_height() {
        let mut c1 = Coord::from(VecD::from([2.3, 3.2, 4.1]));
        c1.set_offset(8.0);
        c1.set_height(2.0);
        let c2 = Coord::from(VecD::from([4.5, -6.1, -4.1]));
        assert_eq!(c1.raw_distance_to(&c1), 4.0);
        assert_eq!(c1.raw_distance_to(&c2), c2.raw_distance_to(&c1));
        #[cfg(feature = "std")]
        assert_eq!(c1.raw_distance_to(&c2), 14.592458060283544);
        #[cfg(not(feature = "std"))]
        assert_eq!(c1.raw_distance_to(&c2), 14.59245806);
    }

    #[test]
    fn distance_with_offset() {
        let mut c1 = Coord::from(VecD::from([2.3, 3.2, 4.1]));
        let mut c2 = Coord::from(VecD::from([4.5, -6.1, -4.1]));
        c1.set_offset(1.2);
        c2.set_offset(10.243);
        #[cfg(feature = "std")]
        assert_eq!(c1.distance_to(&c2), 24.035458060283545);
        #[cfg(not(feature = "std"))]
        assert_eq!(c1.distance_to(&c2), 24.03545806);
    }

    #[test]
    fn distance_with_neg_offset() {
        let mut c1 = Coord::from(VecD::from([0.2, -3.3, 1.1]));
        let c2 = Coord::from(VecD::from([3.232, 3.123, -3.4]));

        c1.set_offset(-10.34);

        #[cfg(feature = "std")]
        assert_eq!(c1.distance_to(&c2), 8.408207478410603);
        #[cfg(not(feature = "std"))]
        assert_eq!(c1.distance_to(&c2), 8.40820748);
    }

    #[test]
    fn distance_with_height() {
        let mut c1 = Coord::from(VecD::from([2.3, 3.2, 4.1]));
        let mut c2 = Coord::from(VecD::from([4.5, -6.1, -4.1]));
        c1.set_height(1.2);
        c2.set_height(10.243);
        #[cfg(feature = "std")]
        assert_eq!(c1.distance_to(&c2), 24.035458060283545);
        #[cfg(not(feature = "std"))]
        assert_eq!(c1.distance_to(&c2), 24.03545806);
    }
}
