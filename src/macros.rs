macro_rules! impl_vec {
    ($t:ty) => {
        impl<const N: usize> Vector for $t {
            const LEN: usize = N;

            fn difference(&self, other: &Self) -> Self {
                let mut ret = Self::default();
                for (n, (s, r)) in ret
                    .inner
                    .iter_mut()
                    .zip(self.inner.iter().zip(other.inner.iter()))
                {
                    *n = s - r;
                }
                ret
            }

            fn magnitude2(&self) -> f64 {
                let mut term: f64 = 0.0;
                for n in self.inner.iter() {
                    term += n * n;
                }
                term
            }
        }

        impl<const N: usize> Add<$t> for $t {
            type Output = $t;

            fn add(self, rhs: Self) -> Self::Output {
                let mut ret = Self::default();
                for (n, (s, r)) in ret
                    .inner
                    .iter_mut()
                    .zip(self.inner.iter().zip(rhs.inner.iter()))
                {
                    *n = s + r;
                }
                ret
            }
        }

        impl<const N: usize> AsRef<[f64]> for $t {
            fn as_ref(&self) -> &[f64] { self.inner.as_ref() }
        }

        impl<const N: usize> AsMut<[f64]> for $t {
            fn as_mut(&mut self) -> &mut [f64] { self.inner.as_mut() }
        }

        impl<const N: usize> AddAssign<$t> for $t {
            fn add_assign(&mut self, rhs: $t) {
                for (s, r) in self.inner.iter_mut().zip(rhs.inner.iter()) {
                    *s += r;
                }
            }
        }

        impl<const N: usize> Mul<f64> for $t {
            type Output = $t;

            fn mul(self, rhs: f64) -> Self {
                let mut ret = Self::default();
                for (n, s) in ret.inner.iter_mut().zip(self.inner.iter()) {
                    *n = s * rhs;
                }
                ret
            }
        }

        impl<const N: usize> Div<f64> for $t {
            type Output = $t;

            fn div(self, rhs: f64) -> Self {
                let mut ret = Self::default();
                for (n, s) in ret.inner.iter_mut().zip(self.inner.iter()) {
                    *n = s / rhs;
                }
                ret
            }
        }
    };
}
