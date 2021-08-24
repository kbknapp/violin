//! Defines the `VecD` coordinate vector that does not use any heap allocation

use crate::{
    std::ops::{Add, AddAssign, Div, Mul},
    Vector,
};

/// A `VecD` is a coordiante vector made up of some number of `f64`s stored as
/// an array
///
/// ```rust
/// use violin::heapless::VecD;
///
/// // defines a 3D vector
/// let v3 = VecD::from([1.1, 2.2, 3.3]);
///
/// // defines a 2D vector
/// let v3 = VecD::from([1.1, 2.2]);
///
/// // defines a 8D vector
/// let v3 = VecD::from([1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8]);
/// ```
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct VecD<const N: usize> {
    inner: [f64; N],
}

impl<const N: usize> Default for VecD<N> {
    fn default() -> Self { Self { inner: [0.0f64; N] } }
}

impl<const N: usize> From<[f64; N]> for VecD<N> {
    fn from(arr: [f64; N]) -> Self { Self { inner: arr } }
}

impl_vec!(VecD<N>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance() {
        assert_eq!(
            VecD::from([1., 0., 5.]).distance(&VecD::from([0., 2., 4.])),
            2.449489742783178
        );
    }

    #[test]
    fn magnitude() {
        assert_eq!(VecD::<3>::default().magnitude(), 0.0);
        assert_eq!(VecD::from([1.0, -2.0, 3.0]).magnitude(), 3.7416573867739413);
        assert_eq!(VecD::from([-2., 4., -4.]).magnitude(), 6.0f64);
    }

    #[test]
    fn unit_vector() {
        let (_, uv) = VecD::from([1., 0., 5.]).unit_vector_from(&VecD::from([0., 2., 4.]));
        assert_eq!(
            uv,
            VecD::from([0.4082482904638631, -0.8164965809277261, 0.4082482904638631])
        );

        let a = VecD::from([1.0, 2.0, 3.0]);
        let b = VecD::from([0.5, 0.6, 0.7]);
        let (mag, uv) = a.unit_vector_from(&b);
        assert_eq!(
            uv,
            VecD::from([0.18257418583505536, 0.511207720338155, 0.8398412548412546])
        );
        let uv_mag = uv.magnitude();
        assert!(uv_mag > 0.9999999 && uv_mag <= 1.0);
        assert_eq!(mag, a.difference(&b).magnitude());
    }

    #[test]
    fn equal_unit_vectors() {
        // equal coordinates should not get a divide by zero
        let a = VecD::from([1.0, 2.0, 3.0]);
        let (mag, uv) = a.unit_vector_from(&a);
        assert_eq!(uv.magnitude(), 1.0);
        assert_eq!(mag, 0.0);
    }

    #[test]
    fn add() {
        let a = VecD::from([1.0, -3.0, 3.0]);
        let b = VecD::from([-4.0, 5.0, 6.0]);
        assert_eq!(a + b, VecD::from([-3.0, 2.0, 9.0]));
        assert_eq!(a + VecD::default(), a);
    }

    #[test]
    fn difference() {
        let a = VecD::from([1.0, -3.0, 3.0]);
        let b = VecD::from([-4.0, 5.0, 6.0]);
        assert_eq!(a.difference(&b), VecD::from([5.0, -8.0, -3.0]));
        assert_eq!(a.difference(&VecD::default()), a);
    }
}
