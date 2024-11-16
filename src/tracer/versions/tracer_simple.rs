use crate::{
    camera::camera::Camera,
    entity::scene::Scene,
    helpers::types::{color, vec3},
    tracer::ray::ray::Ray,
};
use rand::rngs::ThreadRng;
use rand_distr::{Distribution, UnitSphere};

pub struct TracerSimple {
    pub cam: Camera,
    pub scene: Scene,

    pub sphere_dist: UnitSphere,
}

impl TracerSimple {
    fn sample_on_sphere(&self, rng: &mut ThreadRng) -> vec3 {
        self.sphere_dist.sample(rng).into()
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
        let ray = self.cam.ray_at(ix, iy);
        self.color_from_ray(ray, rng)
    }
}
