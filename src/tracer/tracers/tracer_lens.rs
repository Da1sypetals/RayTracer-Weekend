use crate::{
    camera::camera_lens::LensCamera,
    entity::scene::Scene,
    helpers::{
        constants::MAX_NUM_REFLECTION,
        types::{color, vec3},
    },
    math::{distributions::sample_uniform_01, panics::PanickingNormalize},
    tracer::ray::ray::Ray,
};
use rand::rngs::ThreadRng;

pub struct TracerLens {
    pub cam: LensCamera,
    pub scene: Scene,
}

impl TracerLens {
    fn sample_pixel_delta(&self, spp: usize, rng: &mut ThreadRng) -> Vec<(f64, f64)> {
        (0..spp)
            .map(|_| (sample_uniform_01(rng), sample_uniform_01(rng)))
            .collect()
    }

    pub fn color_at(&self, ix: u32, iy: u32, rng: &mut ThreadRng) -> color {
        let pixel_lefttop = self.cam.image_space.pixel_lefttop_at(ix, iy);

        let spp = 16;
        let deltas: Vec<_> = self.sample_pixel_delta(spp, rng);

        let mut color = vec3::zeros();
        for (dx, dy) in deltas {
            let pixel = pixel_lefttop + self.cam.image_space.pixel_offset(dx, dy);

            let cam_pos = self.cam.sample_position(rng);
            let v = pixel - cam_pos;
            // let mag = v.magnitude();
            let dir = v.p_normalize();
            let ray = Ray::new(cam_pos, dir, 0.0);

            color += self.color_from_ray(ray, rng, 0);
        }

        color / spp as f64
    }
}

impl TracerLens {
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
            // Skybox
            let t = 0.5 * (ray.dir.y + 1.0);
            (1.0 - t) * color::new(1.0, 1.0, 1.0) + t * color::new(0.5, 0.7, 1.0)
        }
    }
}
