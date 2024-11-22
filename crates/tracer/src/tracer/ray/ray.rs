use crate::{helpers::types::vec3, math::panics::PanickingNormalize};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub orig: vec3,
    pub dir: vec3,
    pub tmin: f64,
}

impl Ray {
    pub fn new(orig: vec3, dir: vec3, tmin: f64) -> Self {
        Self {
            orig,
            dir: dir.p_normalize(),
            tmin,
        }
    }

    pub fn at(&self, t: f64) -> vec3 {
        self.orig + t * self.dir
    }
}
