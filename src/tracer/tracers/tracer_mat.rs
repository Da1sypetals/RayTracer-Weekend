use crate::{
    camera::camera::Camera,
    entity::scene::Scene,
    helpers::{
        constants::MAX_NUM_REFLECTION,
        types::{color, vec3},
    },
    math::panics::PanickingNormalize,
    tracer::ray::ray::Ray,
};
use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Uniform};

pub struct TracerMat {
    pub cam: Camera,
    pub scene: Scene,

    pub uniform: Uniform<f64>,

    pub reflect_ratio: f64,
}

impl TracerMat {
    fn sample_pixel_delta(&self, spp: usize, rng: &mut ThreadRng) -> Vec<(f64, f64)> {
        (0..spp)
            .map(|_| (self.uniform.sample(rng), self.uniform.sample(rng)))
            .collect()
    }

    pub fn color_at(&self, ix: u32, iy: u32, rng: &mut ThreadRng) -> color {
        let pixel_lefttop = self.cam.image_space.pixel_lefttop_at(ix, iy);

        let spp = 16;
        let deltas: Vec<_> = self.sample_pixel_delta(spp, rng);

        let mut color = vec3::zeros();
        for (dx, dy) in deltas {
            let pixel = pixel_lefttop + self.cam.image_space.pixel_offset(dx, dy);

            let v = pixel - self.cam.pos;
            let mag = v.magnitude();
            let dir = v.p_normalize();
            let ray = Ray::new(self.cam.pos, dir, mag);

            color += self.color_from_ray(ray, rng, 0);
        }

        color / spp as f64
    }
}

impl TracerMat {
    /// FIXME: modify this
    pub fn color_from_ray(&self, ray: Ray, rng: &mut ThreadRng, depth: u32) -> vec3 {
        if depth >= MAX_NUM_REFLECTION {
            return color::zeros();
        }

        if let Some(hit) = self.scene.hit_by(ray) {
            if let Some((attenuation, scattered_ray)) = hit.scatter(rng) {
                attenuation.component_mul(&self.color_from_ray(scattered_ray, rng, depth + 1))
            } else {
                color::zeros()
            }
        } else {
            // Skybox
            let t = 0.5 * (ray.dir.y + 1.0);
            (1.0 - t) * color::new(1.0, 1.0, 1.0) + t * color::new(0.5, 0.7, 1.0)
        }
    }
}
