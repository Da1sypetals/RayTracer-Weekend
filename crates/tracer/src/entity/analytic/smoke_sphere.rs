use crate::{
    entity::traits::Entity,
    helpers::types::vec3,
    materials::material::{FragMaterial, Material},
    math::distributions::sample_uniform_01,
    tracer::ray::hit::{Hit, Normal},
};
use rand::thread_rng;

use super::sphere::Sphere;

#[derive(Debug)]
pub struct SmokeSphere {
    pub sphere: Sphere,
    pub mat: Material,
    pub k: f64,
}

impl SmokeSphere {
    pub fn new(center: vec3, radius: f64, mat: Material) -> Self {
        let k = if let Material::Smoke { k } = mat {
            k
        } else {
            panic!("Unsupported material!")
        };
        Self {
            sphere: Sphere::new(center, radius, mat.clone()),
            mat,
            k,
        }
    }
}

impl Entity for SmokeSphere {
    fn hit_by(
        &self,
        ray: crate::tracer::ray::ray::Ray,
        interval: crate::math::interval::Interval,
    ) -> Option<crate::tracer::ray::hit::Hit> {
        let nearest_hit = self.sphere.hit_by(ray, interval);

        // find a nearest hit, refer to sphere struct
        if self.contains(ray.orig) {
            if let Some(hit) = nearest_hit {
                let mut rng = thread_rng();
                let rand = sample_uniform_01(&mut rng);

                let t = if rand > self.k * hit.t {
                    // pass through
                    hit.t
                } else {
                    rand / self.k
                };
                if interval.contains(t) {
                    let hitpos = ray.at(t);
                    Some(Hit {
                        in_dir: ray.dir,
                        pos: hitpos,
                        material: FragMaterial::Smoke,
                        t,
                        normal: Normal::Outward(vec3::new(1.0, 0.0, 0.0)),
                    })
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            // origin is outside, transparent
            nearest_hit.map(|mut hit| {
                hit.material = FragMaterial::Transparent;
                hit
            })
        }
    }
}

impl SmokeSphere {
    fn contains(&self, hitpos: vec3) -> bool {
        (hitpos - self.sphere.center).norm_squared() < self.sphere.radius * self.sphere.radius
    }
}
