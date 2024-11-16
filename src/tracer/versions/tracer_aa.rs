use crate::{
    camera::camera::Camera,
    entity::scene::Scene,
    helpers::types::{color, vec3},
    math::panics::PanickingNormalize,
    tracer::ray::ray::Ray,
};
use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Uniform, UnitSphere};

pub struct TracerAa {
    pub cam: Camera,
    pub scene: Scene,

    pub sphere_dist: UnitSphere,
    pub uniform: Uniform<f64>,
}

impl TracerAa {
    fn sample_on_sphere(&self, rng: &mut ThreadRng) -> vec3 {
        self.sphere_dist.sample(rng).into()
    }

    fn sample_pixel_delta(&self, rng: &mut ThreadRng) -> (f64, f64) {
        (self.uniform.sample(rng), self.uniform.sample(rng))
    }

    pub fn color_from_ray(&self, ray: Ray, rng: &mut ThreadRng) -> vec3 {
        if let Some(hit) = self.scene.hit_by(ray) {
            // shoot ray from hit point
            let mut dir = self.sample_on_sphere(rng);
            if dir.dot(&hit.normal) < 0.0 {
                dir = -dir
            }
            let ray = Ray::new(hit.pos, dir, 0.0);
            0.5 * self.color_from_ray(ray, rng)
        } else {
            // default color based on ix and iy
            let t = 0.5 * (ray.dir.y + 1.0);
            (1.0 - t) * color::new(1.0, 1.0, 1.0) + t * color::new(0.5, 0.7, 1.0)
        }
    }

    pub fn color_at(&self, ix: u32, iy: u32, rng: &mut ThreadRng) -> color {
        let pixel_orig = self.cam.image_space.pixel_center_at(ix, iy)
            - self.cam.image_space.xdir * 0.5
            - self.cam.image_space.ydir * 0.5;

        let n_sample = 16;
        let deltas: Vec<_> = (0..n_sample)
            .map(|_| self.sample_pixel_delta(rng))
            .collect();

        let mut color = vec3::zeros();
        for (dx, dy) in deltas {
            let pixel =
                pixel_orig + self.cam.image_space.xdir * dx + self.cam.image_space.ydir * dy;

            let v = pixel - self.cam.pos;
            let mag = v.magnitude();
            let dir = v.p_normalize();
            let ray = Ray::new(self.cam.pos, dir, mag);

            color += self.color_from_ray(ray, rng);
        }

        color / n_sample as f64
    }
}
