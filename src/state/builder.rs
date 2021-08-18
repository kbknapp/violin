#[cfg(not(feature = "std"))]
use core::{default::Default, ops::Add};

#[cfg(feature = "std")]
use std::{default::Default, ops::Add};

use crate::{point::Point, state::State, Coordinate};

#[derive(Debug, Clone, Copy)]
pub struct StateBuilder {
    error_estimate: u64,
    resistance: f64,
    error_sensitivity: f64,
    pos_sensitivity: f64,
    height: u8,
}

impl StateBuilder {
    pub fn new() -> Self {
        Self {
            error_estimate: 0,
            resistance: 1.0,
            error_sensitivity: 0.25,
            pos_sensitivity: 0.25,
            height: 0,
        }
    }

    pub fn error_estimate(mut self, ee: u64) -> Self {
        self.error_estimate = ee;
        self
    }

    pub fn resistance(mut self, r: f64) -> Self {
        self.resistance = r;
        self
    }

    pub fn error_sensitivity(mut self, es: f64) -> Self {
        self.error_sensitivity = es;
        self
    }

    pub fn position_sensitivity(mut self, ps: f64) -> Self {
        self.pos_sensitivity = ps;
        self
    }

    pub fn build<T>(self) -> State<T>
    where
        T: Coordinate,
    {
        State {
            point: Point::default(),
            error_estimate: self.error_estimate as f64,
            resistance: self.resistance,
            error_sensitivity: self.error_sensitivity,
            pos_sensitivity: self.pos_sensitivity,
            height: self.height,
        }
    }
}
