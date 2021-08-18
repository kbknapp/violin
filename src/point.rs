use std::ops::{Add, AddAssign, Mul};

use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};

#[derive(Debug, Clone)]
pub struct Point<const N: usize> {
    uvec: [f64; N],
    height: u8,
}

impl<const N: usize> Point<N> {
    pub fn new() -> Self {
        let mut arr = [0.0f64; N];
        let mut rng = rand::thread_rng();
        let die = Uniform::from(-1.0..1.0);
        for n in arr.iter_mut() {
            *n = die.sample(&mut rng);
        }
        Point {
            uvec: arr,
            height: 0,
        }
    }
    pub fn with_height(h: u8) -> Self {
        let mut c = Self::new();
        c.height = h;
        c
    }
}

impl<const N: usize> Mul<f64> for Point<N> {
    type Output = Point<N>;
    fn mul(self, rhs: f64) -> Self {
        todo!("impl Point::mul")
    }
}

impl<const N: usize> Add<Point<N>> for Point<N> {
    type Output = Point<N>;
    fn add(self, rhs: Point<N>) -> Self {
        todo!("impl Point::add")
    }
}

impl<const N: usize> AddAssign<Point<N>> for Point<N> {
    fn add_assign(&mut self, rhs: Point<N>) {
        todo!("impl Point::add")
    }
}
