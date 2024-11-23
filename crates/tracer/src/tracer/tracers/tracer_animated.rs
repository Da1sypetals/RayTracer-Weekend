use crate::{
    camera::camera_lens::{LensCamera, LensCameraBuilder},
    entity::animated_scene::AnimatedScene,
    helpers::{
        constants::MAX_NUM_REFLECTION,
        types::{color, vec3},
    },
    math::{distributions::sample_uniform_01, panics::PanickingNormalize},
    tracer::ray::ray::Ray,
};
use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct TracerAnimatedBuilder {
    camera: String,
    scene: String,
    spp: usize,
    out_path: String,
    n_step: u32,
}

pub struct TracerAnimated {
    pub cam: LensCamera,
    pub scene: AnimatedScene,
    pub spp: usize,
    pub out_path: String,
    pub n_step: u32,
}

impl TracerAnimated {
    pub fn configured(path: &str) -> anyhow::Result<Self> {
        let builder: TracerAnimatedBuilder =
            toml::from_str(fs::read_to_string(path).unwrap().as_str()).unwrap();

        let cam = LensCameraBuilder::configured(&builder.camera)?.build();
        let scene = AnimatedScene::configured(&builder.scene, builder.n_step)?;
        Ok(Self {
            cam,
            scene,
            spp: builder.spp,
            out_path: builder.out_path,
            n_step: builder.n_step,
        })
    }
}

impl TracerAnimated {
    fn sample_pixel_delta(&self, spp: usize, rng: &mut ThreadRng) -> Vec<(f64, f64)> {
        (0..spp)
            .map(|_| (sample_uniform_01(rng), sample_uniform_01(rng)))
            .collect()
    }

    pub fn color_at(&self, ix: u32, iy: u32, rng: &mut ThreadRng) -> color {
        let pixel_lefttop = self.cam.image_space.pixel_lefttop_at(ix, iy);

        let deltas = &self.sample_pixel_delta(self.spp, rng);

        let mut color = vec3::zeros();

        for (dx, dy) in deltas {
            let pixel = pixel_lefttop + self.cam.image_space.pixel_offset(*dx, *dy);

            let cam_pos = self.cam.sample_position(rng);
            let v = pixel - cam_pos;
            // let mag = v.magnitude();
            let dir = v.p_normalize();
            let ray = Ray::new(cam_pos, dir, 0.0);

            color += self.color_from_ray(ray, rng, 0);
        }

        color / self.spp as f64
    }
}

impl TracerAnimated {
    /// FIXME: modify this
    pub fn color_from_ray(&self, ray: Ray, rng: &mut ThreadRng, depth: u32) -> vec3 {
        if depth >= MAX_NUM_REFLECTION {
            return color::zeros();
        }

        if let Some(hit) = self.scene.hit_by(ray) {
            if let Some((attenuation, scattered_ray)) = hit.scatter(rng) {
                // bounce from other light ray
                attenuation.component_mul(&self.color_from_ray(scattered_ray, rng, depth + 1))
            } else {
                // absorbed
                color::zeros()
            }
        } else {
            self.scene.background.color(ray.dir)
        }
    }
}
