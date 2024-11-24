use crate::{
    camera::camera_lens::{LensCamera, LensCameraBuilder},
    entity::scene::Scene,
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
pub struct TracerIterBuilder {
    camera: String,
    scene: String,
    spp: usize,
    out_path: String,
}

pub struct TracerIter {
    pub cam: LensCamera,
    pub scene: Scene,
    pub spp: usize,
    pub out_path: String,
}

impl TracerIter {
    pub fn configured(path: &str) -> anyhow::Result<Self> {
        let builder: TracerIterBuilder =
            toml::from_str(fs::read_to_string(path).unwrap().as_str()).unwrap();

        let cam = LensCameraBuilder::configured(&builder.camera)?.build();
        let scene = Scene::configured(&builder.scene)?;
        Ok(Self {
            cam,
            scene,
            spp: builder.spp,
            out_path: builder.out_path,
        })
    }
}

impl TracerIter {
    fn sample_pixel_delta(&self, spp: usize, rng: &mut ThreadRng) -> Vec<(f64, f64)> {
        (0..spp)
            .map(|_| (sample_uniform_01(rng), sample_uniform_01(rng)))
            .collect()
    }

    pub fn color_at(&self, ix: u32, iy: u32, rng: &mut ThreadRng) -> color {
        let pixel_lefttop = self.cam.image_space.pixel_lefttop_at(ix, iy);

        let deltas: Vec<_> = self.sample_pixel_delta(self.spp, rng);

        let mut color = vec3::zeros();
        for (dx, dy) in deltas {
            let pixel = pixel_lefttop + self.cam.image_space.pixel_offset(dx, dy);

            let cam_pos = self.cam.sample_position(rng);
            let v = pixel - cam_pos;
            // let mag = v.magnitude();
            let dir = v.p_normalize();
            let ray = Ray::new(cam_pos, dir, 0.0);

            color += self.color_from_ray(ray, rng);
        }

        color / self.spp as f64
    }
}

impl TracerIter {
    /// FIXME: modify this
    pub fn color_from_ray(&self, ray: Ray, rng: &mut ThreadRng) -> vec3 {
        let mut total_color = color::zeros();
        let mut current_ray = ray;
        let mut current_attenuation = vec3::new(1.0, 1.0, 1.0);

        for _ in 0..MAX_NUM_REFLECTION {
            if let Some(hit) = self.scene.hit_by(current_ray) {
                let emitted = hit.emit();
                total_color += current_attenuation.component_mul(&emitted);

                if let Some((attenuation, scattered_ray)) = hit.scatter(rng) {
                    current_attenuation = current_attenuation.component_mul(&attenuation);
                    current_ray = scattered_ray;
                } else {
                    break;
                }
            } else {
                total_color += current_attenuation
                    .component_mul(&self.scene.background.color(current_ray.dir));
                break;
            }
        }

        total_color
    }
}
