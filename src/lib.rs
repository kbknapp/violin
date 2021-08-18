#![deny(
    missing_docs,
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
    Point: Point<N>,
    err_est: f64,
    resist: f64,
    pos_sens_adj: f64,
    err_sens_adj: f64,
}

impl<const N: usize> State<N> {
    pub fn new() -> Self {
        Self {
            Point: Point::new(),
            err_est: 2000.0,
            resist: 1.0,
            pos_sens_adj: 0.25,
            err_sens_adj: 0.25,
        }
    }

    pub fn update(&mut self, rtt: f64, r_c: Point<N>, r_e: f64) {
        // Sample weight balances local and remote error
        let err_weight = self.err_est / (self.err_est + r_e);
        // Compute relative error of this sample
        let e = dist(&self.Point, &r_c) - rtt;
        let rel_e = e / rtt;
        // Update err_weighted moving average of local error
        self.err_est = rel_e * self.err_sens_adj * err_weight
            + self.err_est * (1.0 - self.err_sens_adj * err_weight);
        // Update local Pointinates
        self.resist = self.pos_sens_adj * err_weight;
        let scaled_dir = dir(&self.Point, &r_c) * e;
        self.Point += scaled_dir * self.resist;
    }
}

fn dist<const N: usize>(p1: &Point<N>, p2: &Point<N>) -> f64 {
    todo!("impl dist()")
}

fn dir<const N: usize>(p1: &Point<N>, p2: &Point<N>) -> Point<N> {
    todo!("impl dir()")
}
