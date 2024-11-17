use crate::{
    helpers::types::{vec2, vec3},
    math::panics::PanickingNormalize,
};

use super::image_space::ImageSpace;

pub struct CameraBuilder {
    pub resolution: glm::UVec2,
    pub yfov: f64,
    pub vd: f64,

    pub pos: vec3,

    // directions
    pub lookat: vec3,
    pub up: vec3,

    /// camera model
    pub model: CameraModel,
}

impl CameraBuilder {
    pub fn build(self) -> Camera {
        Camera::new(self)
    }
}

pub enum CameraModel {
    Pinhole,
    Lens { radius: f64 },
}

pub struct Camera {
    pub resolution: glm::UVec2,
    /// ### In radians
    pub yfov: f64,
    /// ### Distance from camera to viewport
    pub vd: f64,
    /// ### Image span (x, y) in real world coords unit
    pub image_span: vec2,

    pub pos: vec3,

    // directions
    pub lookat: vec3,
    pub up: vec3,
    pub right: vec3,

    pub image_space: ImageSpace,

    /// camera model
    pub model: CameraModel,
}

impl Camera {
    pub fn new(cam: CameraBuilder) -> Self {
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
            resolution: cam.resolution,
            yfov: cam.yfov,
            pos: cam.pos,
            lookat: cam.lookat,
            up: cam.up,
            vd: cam.vd,
            model: cam.model,

            image_span,
            right,
            image_space,
        }
    }
}
