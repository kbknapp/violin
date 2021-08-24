use crate::{
    error::{Error, ErrorKind, Result},
    std::time::Duration,
    Coord, Vector, DEFAULT_HEIGHT_MIN,
};

/// Tunables that affect how [`Node`]s handle coordinates and updates
#[derive(Debug, Copy, Clone)]
pub struct Config {
    /// Bounds error estimates to this upper limit. This is also the initial
    /// value of an error estimate before making any updates.
    pub error_max: f64,

    /// The minimum value of the height parameter
    pub height_min: f64,

    /// How hard gravity pulls coordinates back to center to avoid constant
    /// drift
    pub gravity_rho: f64,

    /// The maximum impact an observation can have on a node's confidence
    pub ce: f64,

    /// The maximum impact an observation can have on a node's coordinate
    pub cc: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            error_max: 1.5,
            height_min: DEFAULT_HEIGHT_MIN,
            gravity_rho: 150.0,
            ce: 0.25,
            cc: 0.25,
        }
    }
}

/// A `Node` is a higher level construct that abstracts over *using* Vivaldi and
/// includes things like maintaining adjustment calculations over a window of
/// RTT measurements.
///
/// The two generic arguments `V` and `A` are the coordinate vector `V` and the
/// adjustment window buffer `A`. By default the adjustment window buffer is set
/// to a heapless 0 sized buffer (`[0f64; 0]`) but can be made to be as large as
/// one requires, and use either the `heapless::VecD` or `heap::VecD` (with the
/// `alloc` feature)
#[derive(Debug, Clone, Default)]
pub struct Node<V, A = crate::heapless::VecD<0>> {
    coord: Coord<V>,
    cfg: Config,
    adjustments: A,
    adj_idx: usize,
}

impl<V, A> Node<V, A>
where
    V: Vector,
    A: Vector,
{
    /// Create a new node with a default coordinate and configuration
    pub fn new() -> Self { Self::default() }

    /// Create a new node with a default coordinate
    pub fn with_config(cfg: Config) -> Self { Self::with_coord_and_cfg(Coord::default(), cfg) }

    /// The coordinate's error_estimate and height will be reset using the
    /// default values. If other values are desired, you must pass a config
    /// with those values via `Node::with_coord_and_config`
    pub fn with_coord<U>(coord: U) -> Self
    where
        U: Into<Coord<V>>,
    {
        Self::with_coord_and_cfg(coord, Config::default())
    }

    /// The coordinate's height will be reset using the configs height_min value
    /// if it passed coordinate's height falls below the specified min
    /// value. Likewise, the error_estimate will be lowered to the specified
    /// config's error_max if it is beyond that value.
    pub fn with_coord_and_cfg<U>(coord: U, cfg: Config) -> Self
    where
        U: Into<Coord<V>>,
    {
        let mut coord = coord.into();
        coord.height = f64::max(cfg.height_min, coord.height);
        coord.error_estimate = f64::min(cfg.error_max, coord.error_estimate);
        Self {
            coord,
            cfg,
            adjustments: A::default(),
            adj_idx: 0,
        }
    }

    /// Create a new node with an initialized random coordinate
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn rand() -> Self { Self::rand_with_cfg(Config::default()) }

    /// Create a new node with an initialized random coordinate
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn rand_with_cfg(cfg: Config) -> Self { Self::with_coord_and_cfg(Coord::rand(), cfg) }

    /// Returns the inner coordinate
    pub fn coordinate(&self) -> &Coord<V> { &self.coord }

    /// Sets the inner coordinate, however the coordinate's error_estimate and
    /// height will be set within the bounds of the current node
    /// configuration.
    pub fn set_coordinate<U>(&mut self, coord: U)
    where
        U: Into<Coord<V>>,
    {
        let mut coord = coord.into();
        coord.height = f64::max(self.cfg.height_min, coord.height);
        coord.error_estimate = f64::min(self.cfg.error_max, coord.error_estimate);
        self.coord = coord;
    }

    /// Returns estimated latency to `other`
    pub fn distance_to(&self, other: &Coord<V>) -> Duration {
        Duration::from_secs_f64(self.coord.distance_to(other))
    }

    /// Update the node's coordinate based off the RTT of the
    /// `other` coordinate.
    ///
    /// # Panics
    ///
    /// Panics if any:
    ///
    /// - This coordinate's AND the remote's error estimate `<= 0.0`
    ///
    /// # Errors
    ///
    /// Returns an error if update cuased the coordinate to become invalid
    pub fn try_update(&mut self, rtt: Duration, other: &Coord<V>) -> Result<()> {
        let rtt = f64::max(f64::MIN_POSITIVE, rtt.as_secs_f64());

        self.coord.update(rtt, other, &self.cfg);
        self.update_offset(rtt, other);

        if self.coord.is_finite() {
            return Ok(());
        }
        Err(Error {
            kind: ErrorKind::InvalidCoordinate,
        })
    }

    /// Gravity pulls the coordinate back toward the origin to prevent drift
    pub fn update_gravity(&mut self, origin: &Coord<V>) {
        self.coord.apply_gravity(origin, &self.cfg);
    }

    fn update_offset(&mut self, rtt: f64, other: &Coord<V>) {
        if A::LEN == 0 {
            return;
        }

        self.adjustments.as_mut()[self.adj_idx] = rtt - self.coord.raw_distance_to(other);
        self.adj_idx = (self.adj_idx + 1) % A::LEN;

        let adj_sum = self.adjustments.as_ref().iter().fold(0.0, |acc, n| acc + n);
        self.coord.offset = adj_sum / (2 * A::LEN) as f64;
    }
}

impl<V, A> Node<V, A>
where
    V: Vector + Clone,
    A: Vector,
{
    /// Update the node's coordinate based off the RTT of the `other`
    /// coordinate. If the update causes the coordinate to become invalid,
    /// it will be reset and return `false`
    ///
    /// # Panics
    ///
    /// Panics if any:
    ///
    /// - This coordinate's AND the remote's error estimate `<= 0.0`
    pub fn update(&mut self, rtt: Duration, other: &Coord<V>) -> bool {
        let coord = self.coord.clone();
        if let Err(Error {
            kind: ErrorKind::InvalidCoordinate,
        }) = self.try_update(rtt, other)
        {
            self.coord = coord;
            return false;
        }
        true
    }
}
