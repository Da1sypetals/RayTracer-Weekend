use crate::{camera::camera::Camera, helpers::types::vec3, math::panics::PanickingNormalize};

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

impl Camera {
    /// ### Maps image space coords to (ray, t_min)
    pub fn ray_at(&self, ix: u32, iy: u32) -> Ray {
        let pixel = self.image_space.pixel_center_at(ix, iy);

        let v = pixel - self.pos;
        let mag = v.magnitude();
        let dir = v.p_normalize();
        Ray::new(self.pos, dir, mag)
    }
}
