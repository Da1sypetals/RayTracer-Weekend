use super::types::{color, vec};

pub trait Color {
    fn r(&self) -> f64;
    fn g(&self) -> f64;
    fn b(&self) -> f64;
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
}
