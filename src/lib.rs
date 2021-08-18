#![deny(
 //   missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_allocation,
    trivial_numeric_casts
)]
#![forbid(unsafe_code)]

mod point;

pub use point::Point;

#[derive(Debug, Clone)]
pub struct State<const N: usize> {
    pos: Point<N>,
    error_estimate: f64,
    resistance: f64,
    error_sensitivity: f64,
    pos_sensitivity: f64,
}

impl<const N: usize> State<N> {
    pub fn new() -> Self {
        Self {
            pos: Point::new(),
            error_estimate: 0.0,
            resistance: 1.0,
            error_sensitivity: 0.25,
            pos_sensitivity: 0.25,
        }
    }

    pub fn error_estimate(&mut self, ee: f64) {
        self.error_estimate = ee;
    }

    pub fn point(&self) -> &Point<N> {
        &self.pos
    }

    pub fn resistance(&mut self, r: f64) {
        self.resistance = r;
    }

    pub fn error_sensitivity(&mut self, es: f64) {
        self.error_sensitivity = es;
    }

    pub fn position_sensitivity(&mut self, ps: f64) {
        self.pos_sensitivity = ps;
    }

    pub fn update(&mut self, rtt: f64, remote: Point<N>, remote_err: f64) {
        // @TODO assert or Err()?
        assert!(rtt >= 0.0);
        assert!(remote_err > 0.0 || self.error_estimate > 0.0);

        // balances local and remote error
        //  - A high local error = greater movement
        //  - A high remote error = less movement
        let err_weight = self.error_estimate / (self.error_estimate + remote_err);

        // Compute relative error of this sample
        let measured_err = self.error_estimate - rtt;
        let relative_err = measured_err / rtt;

        // Compute direction of error, and scale accordingly
        let dir_of_err = self.direction(&remote);
        let scaled_dir = dir_of_err * measured_err; // Or relative_err?

        // Update error estimate moving average of local error
        self.error_estimate = relative_err * self.error_sensitivity * err_weight
            + self.error_estimate * (1.0 - self.error_sensitivity * err_weight);

        // Update local position
        self.resistance = self.pos_sensitivity * err_weight;
        self.pos += scaled_dir * self.resistance;
    }

    fn direction(&self, p2: &Point<N>) -> Point<N> {
        self.pos.direction(p2)
    }

    pub fn distance(&self, p2: &Point<N>) -> f64 {
        self.pos.distance(p2)
    }
}
