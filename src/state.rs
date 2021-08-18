#[cfg(not(feature = "std"))]
use core::{default::Default, ops::Add};

#[cfg(feature = "std")]
use std::{default::Default, ops::Add};

use crate::{point::Point, Coordinate};

mod builder;
pub use builder::StateBuilder;

#[derive(Debug, Clone)]
pub struct State<T> {
    point: Point<T>,
    error_estimate: f64,
    resistance: f64,
    error_sensitivity: f64,
    pos_sensitivity: f64,
    height: u8,
}

impl<T> State<T>
where
    T: Coordinate,
{
    pub fn new() -> Self {
        Self {
            point: Point::default(),
            error_estimate: 0.0,
            resistance: 1.0,
            error_sensitivity: 0.25,
            pos_sensitivity: 0.25,
            height: 0,
        }
    }

    pub fn update(&mut self, rtt: u64, remote: &Point<T>, remote_err: u64) {
        // @TODO assert or Err()?

        // balances local and remote error
        //  - A high local error = greater movement
        //  - A high remote error = less movement
        let err_weight = self.error_estimate / (self.error_estimate + remote_err as f64);

        // Compute relative error of this sample
        let measured_err = self.error_estimate - rtt as f64;
        let relative_err = measured_err / rtt as f64;

        // Compute direction of error, and scale accordingly
        let dir_of_err = self.point.direction(&remote);
        let scaled_dir = dir_of_err * measured_err; // Or relative_err?

        // Update error estimate moving average of local error
        self.error_estimate = relative_err * self.error_sensitivity * err_weight
            + self.error_estimate * (1.0 - self.error_sensitivity * err_weight);

        // Update local position
        self.resistance = self.pos_sensitivity * err_weight;
        self.point += scaled_dir * self.resistance;
    }

    pub fn distance(&self, remote: &Point<T>) -> f64 {
        self.point.distance(remote)
    }
}
