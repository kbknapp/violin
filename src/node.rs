use crate::Coordinate;

#[cfg(feautre = "builder")]
mod builder;
#[cfg(feautre = "builder")]
pub use builder::NodeBuilder;

#[derive(Debug, Clone)]
pub struct Node<T> {
    coord: T,
    error_estimate: f64,
    resistance: f64,
    error_sensitivity: f64,
    pos_sensitivity: f64,
    height: u8,
}

impl<T> Node<T>
where
    T: Coordinate,
{
    pub fn new() -> Self {
        Self {
            coord: T::initialize(),
            error_estimate: 0.0,
            resistance: 1.0,
            error_sensitivity: 0.25,
            pos_sensitivity: 0.25,
            height: 0,
        }
    }

    pub fn coordinate(&self) -> &T {
        &self.coord
    }

    pub fn update(&mut self, rtt: f64, remote: &T, remote_err: f64) {
        // @TODO assert or Err()?

        // balances local and remote error
        //  - A high local error = greater movement
        //  - A high remote error = less movement
        let err_weight = self.error_estimate / (self.error_estimate + remote_err);

        // Compute relative error of this sample
        let measured_err = self.error_estimate - rtt;
        let relative_err = measured_err / rtt;

        // Compute direction of error, and scale accordingly
        let dir_of_err = self.coord.direction(&remote);
        let scaled_dir = dir_of_err * measured_err; // Or relative_err?

        // Update error estimate moving average of local error
        self.error_estimate = relative_err * self.error_sensitivity * err_weight
            + self.error_estimate * (1.0 - self.error_sensitivity * err_weight);

        // Update local position
        self.resistance = self.pos_sensitivity * err_weight;
        self.coord += scaled_dir * self.resistance;
    }

    pub fn distance(&self, remote: &T) -> f64 {
        self.coord.distance(remote)
    }
}
