use rand::rngs::ThreadRng;

use crate::{
    helpers::{
        constants::IGNORE_HIT_EPS,
        types::{color, vec3},
    },
    materials::material::Material,
    math::sphere::sample_on_sphere,
};

use super::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub in_dir: vec3,
    pub pos: vec3,
    pub normal: vec3,
    pub material: Material,
    pub t: f64,
}

impl Hit {
    /// If possible, get the ray that reflected from this hit.
    pub fn scatter(&self, rng: &mut ThreadRng) -> Option<(color, Ray)> {
        match self.material {
            Material::Lambertian { albedo } => {
                let dir = self.normal + sample_on_sphere(rng);
                let scattered_ray = Ray::new(self.pos, dir, IGNORE_HIT_EPS);

                Some((albedo, scattered_ray))
            }
            Material::Metal { albedo } => {
                let reflected_dir =
                    self.in_dir - 2.0 * self.normal.dot(&self.in_dir) * self.normal;

                let reflected_ray = Ray::new(self.pos, reflected_dir, IGNORE_HIT_EPS);

                Some((albedo, reflected_ray))
            }
            Material::FuzzedMetal { albedo, fuzz } => {
                let dir = self.normal + fuzz * sample_on_sphere(rng);
                let scattered_ray = Ray::new(self.pos, dir, IGNORE_HIT_EPS);

                Some((albedo, scattered_ray))
            }
        }
    }
}
