use crate::math::panics::PanickingFloatMethods;

use super::types::{color, GammaColor};

pub trait Color {
    fn r(&self) -> f64;
    fn g(&self) -> f64;
    fn b(&self) -> f64;
    fn to_gamma(&self) -> GammaColor;
}

impl Color for color {
    fn r(&self) -> f64 {
        self.x
    }

    fn g(&self) -> f64 {
        self.y
    }

    fn b(&self) -> f64 {
        self.z
    }

    fn to_gamma(&self) -> GammaColor {
        GammaColor {
            r: self.x.p_sqrt(),
            g: self.y.p_sqrt(),
            b: self.z.p_sqrt(),
        }
    }
}
