use crate::helpers::types::vec;

pub trait PanickingNormalize {
    /// ## PANICS if the input cannot be normalized, i.e. has magnitude 0.
    fn p_normalize(&self) -> Self;
    fn p_magnitude(&self) -> f64;
}

impl<const N: usize> PanickingNormalize for vec<N> {
    #[inline]
    fn p_normalize(&self) -> Self {
        if self.magnitude() <= f64::EPSILON {
            panic!("Vector whose magnitude is 0 cannot be normalized!");
        }
        self.normalize()
    }

    #[inline]
    fn p_magnitude(&self) -> f64 {
        if self.magnitude() <= f64::EPSILON {
            panic!("Vector whose magnitude is 0 cannot be normalized!");
        }
        self.magnitude()
    }
}

pub trait PanickingFloatMethods {
    /// ## PANICS if the input is negative.
    fn p_sqrt(self) -> Self;
}

impl PanickingFloatMethods for f64 {
    #[inline]
    fn p_sqrt(self) -> Self {
        if self < -0.0 {
            panic!("Cannot take square root of floating point whose value < 0");
        }
        self.sqrt()
    }
}
