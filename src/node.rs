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
    #[cfg(all(feature = "std", feature = "rand"))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "std", feature = "rand"))))]
    pub fn rand() -> Self { Self::rand_with_cfg(Config::default()) }

    /// Create a new node with an initialized random coordinate
    #[cfg(all(feature = "std", feature = "rand"))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "std", feature = "rand"))))]
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

    /// Returns the raw error estimate.
    pub fn error_estimate(&self) -> f64 { self.coord.error_estimate() }

    /// Set the raw error estimate.
    ///
    /// # Panics
    ///
    /// If `err_est <= 0.0`
    pub fn set_error_estimate(&mut self, err_est: f64) { self.coord.set_error_estimate(err_est); }

    /// Set the raw error estimate.
    pub fn try_set_error_estimate(&mut self, err_est: f64) -> Result<()> {
        self.coord.try_set_error_estimate(err_est)
    }

    /// Continue to update the node's coordinate based off the RTT (in seconds)
    /// of the `other` coordinate until the estimated distance is within the
    /// given RTT +/- the threshold.
    ///
    /// > **WARNING**
    /// >
    /// > If `other` has low confidence (high error estimate) this can do many
    /// > updates
    ///
    /// > **WARNING 2**
    /// >
    /// > Making this coordinate accurate for a single other node does not
    /// > necessarily mean the coordinates are accurate. It takes a minimum of
    /// > three nodes to be certain of a more accurate coordinate. However,
    /// > simply updating for three other nodes in series (one after the other)
    /// > will not help as this node's coordinates will just shift around the
    /// > coordinate space. Instead one should perform the updates together.
    /// >
    /// > For example, for three nodes `A`, `B`, and `C` imagining it took three
    /// > updates on each to become accurate (three updates is just an arbitrary
    /// > number, in the real world it could be many, many more) instead of
    /// > updating `AAABBBCCC` the updates should be performed `ABCABCABC`,
    /// > which is what [`Node::update_until_all`] does.
    pub fn try_update_until(
        &mut self,
        rtt: Duration,
        other: &Coord<V>,
        threshold: f64,
    ) -> Result<()> {
        self.coord.try_update_until(
            f64::max(f64::MIN_POSITIVE, rtt.as_secs_f64()),
            other,
            threshold,
            &self.cfg,
        )?;
        Ok(())
    }

    /// Continue to update the node's coordinate based off the RTT (in seconds)
    /// of the `other` coordinate until the estimated distance is within the
    /// given RTT +/- the threshold.
    ///
    /// > **WARNING**
    /// >
    /// > If `other` has low confidence (high error estimate) this can do many
    /// > updates
    ///
    /// > **WARNING 2**
    /// >
    /// > Making this coordinate accurate for a single other node does not
    /// > necessarily mean the coordinates are accurate. It takes a minimum of
    /// > three nodes to be certain of a more accurate coordinate. However,
    /// > simply updating for three other nodes in series (one after the other)
    /// > will not help as this node's coordinates will just shift around the
    /// > coordinate space. Instead one should perform the updates together.
    /// >
    /// > For example, for three nodes `A`, `B`, and `C` imagining it took three
    /// > updates on each to become accurate (three updates is just an arbitrary
    /// > number, in the real world it could be many, many more) instead of
    /// > updating `AAABBBCCC` the updates should be performed `ABCABCABC`,
    /// > which is what [`Node::update_until_all`] does.
    ///
    /// # Panics
    ///
    /// Panics if any:
    ///
    /// - `rtt <= 0.0`
    /// - This coordinate's OR the other's error estimate `<= 0.0`
    pub fn update_until(&mut self, rtt: Duration, other: &Coord<V>, threshold: f64) {
        self.coord.update_until(
            f64::max(f64::MIN_POSITIVE, rtt.as_secs_f64()),
            other,
            threshold,
            &self.cfg,
        );
    }

    /// Continue to update the node's coordinate based off all the RTTs (in
    /// seconds) of the `other` coordinates until the estimated distance is
    /// within the given RTT +/- the threshold.
    ///
    /// > **WARNING**
    /// >
    /// > If any of `other` has low confidence (high error estimate) this can do
    /// > many updates
    ///
    /// Panics if any:
    ///
    /// - `rtt <= 0.0`
    /// - This coordinate's AND the other's error estimate `<= 0.0`
    #[cfg(all(feature = "std", feature = "alloc"))]
    pub fn update_until_all(&mut self, others: &[(Duration, &Coord<V>)], threshold: f64) {
        self.coord.update_until_all(
            others
                .iter()
                .map(|(rtt, coord)| (f64::max(f64::MIN_POSITIVE, rtt.as_secs_f64()), *coord)),
            threshold,
            &self.cfg,
        );
    }

    /// Continue to update the node's coordinate based off all the RTTs (in
    /// seconds) of the `other` coordinates until the estimated distance is
    /// within the given RTT +/- the threshold.
    ///
    /// > **WARNING**
    /// >
    /// > If any of `other` has low confidence (high error estimate) this can do
    /// > many updates
    #[cfg(all(feature = "std", feature = "alloc"))]
    pub fn try_update_until_all(
        &mut self,
        others: &[(Duration, &Coord<V>)],
        threshold: f64,
    ) -> Result<()> {
        self.coord.try_update_until_all(
            others
                .iter()
                .map(|(rtt, coord)| (f64::max(f64::MIN_POSITIVE, rtt.as_secs_f64()), *coord)),
            threshold,
            &self.cfg,
        )?;
        Ok(())
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
    /// Returns an error if update caused the coordinate to become invalid
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
    /// - This coordinate's OR the remote's error estimate `<= 0.0`
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
