use super::ray::Ray;
use crate::{
    helpers::{
        constants::IGNORE_HIT_EPS,
        types::{color, vec3},
    },
    materials::material::FragMaterial,
    math::{
        distributions::{sample_on_sphere, sample_uniform_01},
        panics::PanickingFloatMethods,
        ray::{reflectance, RayDir},
    },
};
use rand::rngs::ThreadRng;

#[derive(Debug, Clone, Copy)]
pub enum Normal {
    /// If a body have no _inside_, then every normal is outwards;
    Outward(vec3),
    /// If a body is watertight, then it is necessary to distinguish between inward and outward.
    Inward(vec3),
}

#[derive(Debug, Clone)]
pub struct Hit {
    pub in_dir: vec3,
    pub pos: vec3,
    pub material: FragMaterial,
    pub t: f64,

    // Normal
    pub normal: Normal,
}

impl Hit {
    /// If possible, get the ray that reflected from this hit.
    /// Returns attenuation and scattered ray (by probabilistic means)
    pub fn scatter(&self, rng: &mut ThreadRng) -> Option<(color, Ray)> {
        match self.material {
            FragMaterial::Lambertian { albedo } => {
                if let Normal::Outward(normal) = self.normal {
                    let dir = normal + sample_on_sphere(rng);
                    let scattered_ray = Ray::new(self.pos, dir, IGNORE_HIT_EPS);

                    Some((albedo, scattered_ray))
                } else {
                    None
                }
            }
            FragMaterial::Metal { albedo } => {
                if let Normal::Outward(normal) = self.normal {
                    let reflected_dir = self.in_dir.reflected_by(&normal);

                    let reflected_ray = Ray::new(self.pos, reflected_dir, IGNORE_HIT_EPS);

                    Some((albedo, reflected_ray))
                } else {
                    None
                }
            }
            FragMaterial::FuzzedMetal { albedo, fuzz } => {
                if let Normal::Outward(normal) = self.normal {
                    let dir = (self.in_dir.reflected_by(&normal)
                        + fuzz * sample_on_sphere(rng))
                    .normalize();

                    let scattered_ray = Ray::new(self.pos, dir, IGNORE_HIT_EPS);

                    Some((albedo, scattered_ray))
                } else {
                    None
                }
            }
            FragMaterial::Dielectric { eta } => {
                let (eta_ratio, normal) = match self.normal {
                    Normal::Outward(normal) => (1.0 / eta, normal),
                    Normal::Inward(normal) => (eta, normal),
                };

                let cosine = normal.dot(&-self.in_dir).min(1.0);
                let sine = (1.0 - cosine * cosine).p_sqrt();

                let reflected_dir = if eta_ratio * sine > 1.0
                    || reflectance(cosine, eta_ratio) > sample_uniform_01(rng)
                {
                    // total internal reflection
                    self.in_dir.reflected_by(&normal)
                } else {
                    self.in_dir.refracted_by(&normal, eta_ratio)
                };

                let reflected_ray = Ray::new(self.pos, reflected_dir, IGNORE_HIT_EPS);

                Some((color::new(1.0, 1.0, 1.0), reflected_ray))
            }
        }
    }
}
