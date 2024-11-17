use super::image_space::ImageSpace;
use crate::{
    helpers::types::{vec2, vec3},
    math::{distributions::sample_on_disk, panics::PanickingNormalize},
};
use rand::rngs::ThreadRng;

pub struct LensCameraBuilder {
    /// Lens
    pub defocus_angle: f64,

    pub resolution: glm::UVec2,
    pub yfov: f64,
    pub vd: f64,

    pub pos: vec3,

    // directions
    pub lookat: vec3,
    pub up: vec3,
}

impl LensCameraBuilder {
    pub fn build(self) -> LensCamera {
        LensCamera::new(self)
    }
}

pub struct LensCamera {
    /// Lens
    radius: f64,

    pub resolution: glm::UVec2,
    /// ### In radians
    pub yfov: f64,
    /// ### Distance from camera to viewport
    pub vd: f64,
    /// ### Image span (x, y) in real world coords unit
    pub image_span: vec2,

    pos: vec3,

    // directions
    pub lookat: vec3,
    pub up: vec3,
    pub right: vec3,

    pub image_space: ImageSpace,
}

impl LensCamera {
    pub fn new(cam: LensCameraBuilder) -> Self {
        let mut cam = cam;
        cam.lookat = cam.lookat.p_normalize();
        cam.up = cam.up.p_normalize();

        // camera
        let right = cam.lookat.cross(&cam.up).p_normalize();

        let yspan = 2.0 * cam.vd * (0.5 * cam.yfov).tan();
        let xspan = yspan * (cam.resolution.x as f64) / cam.resolution.y as f64;
        let image_span = vec2::new(xspan, yspan);

        // image space
        let image_space = ImageSpace {
            xdir: right,
            ydir: -cam.up,
            //   cam pos  +  viewport to cam dist -     half Y      - half X
            orig: cam.pos + cam.lookat * cam.vd + cam.up * yspan * 0.5 - right * xspan * 0.5,
            delta: yspan / cam.resolution.y as f64,
        };

        Self {
            radius: cam.vd * (cam.defocus_angle * 0.5).tan(),
            resolution: cam.resolution,
            yfov: cam.yfov,
            pos: cam.pos,
            lookat: cam.lookat,
            up: cam.up,
            vd: cam.vd,

            image_span,
            right,
            image_space,
        }
    }

    pub fn sample_position(&self, rng: &mut ThreadRng) -> vec3 {
        let delta_unit = sample_on_disk(rng);
        let delta = delta_unit.x * self.right + delta_unit.y * self.up;
        self.pos + self.radius * delta
    }
}
