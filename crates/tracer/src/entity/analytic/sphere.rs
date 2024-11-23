use std::f64::consts::{PI, TAU};

use crate::{
    entity::traits::Entity,
    helpers::types::vec3,
    materials::material::{FragMaterial, Material},
    math::panics::{PanickingFloatMethods, PanickingNormalize},
    tracer::ray::hit::{Hit, Normal},
};

#[derive(Debug)]
pub struct Sphere {
    pub center: vec3,
    pub radius: f64,
    pub mat: Material,
}

impl Sphere {
    pub fn new(center: vec3, radius: f64, mat: Material) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Entity for Sphere {
    fn hit_by(
        &self,
        ray: crate::tracer::ray::ray::Ray,
        interval: crate::math::interval::Interval,
    ) -> Option<crate::tracer::ray::hit::Hit> {
        let o = ray.orig - self.center;

        let a = ray.dir.norm_squared();
        let b = 2.0 * o.dot(&ray.dir);
        let c = o.norm_squared() - self.radius * self.radius;

        let delta = b * b - 4.0 * a * c;

        if delta < 0.0 {
            None
        } else {
            let sqrt_delta = delta.p_sqrt();
            let r1 = (-b + sqrt_delta) / 2.0 * a;
            let r2 = (-b - sqrt_delta) / 2.0 * a;
            let t = if interval.contains(r1) && interval.contains(r2) {
                r1.min(r2)
            } else if interval.contains(r1) {
                r1
            } else if interval.contains(r2) {
                r2
            } else {
                return None;
            };

            let hitpos = ray.at(t);
            let v = ray.orig + ray.dir * interval.min().expect("Ray should have minimum t!")
                - self.center;
            let normal = if v.norm_squared() > self.radius * self.radius {
                Normal::Outward((hitpos - self.center).p_normalize())
            } else {
                Normal::Inward((self.center - hitpos).p_normalize())
            };

            let material: FragMaterial = self.frag_material(hitpos);
            Some(Hit {
                in_dir: ray.dir,
                pos: hitpos,
                normal,
                t,
                material,
            })
        }
    }

    fn material(&self) -> crate::materials::material::Material {
        self.mat.clone()
    }
}

impl Sphere {
    #[inline]
    fn spherical_coords(&self, hitpos: vec3) -> (f64, f64) {
        let v = hitpos - self.center;
        ((v.y / v.p_magnitude()).acos(), f64::atan2(v.z, v.x) + PI)
    }

    fn frag_material(&self, hitpos: vec3) -> FragMaterial {
        match self.mat.clone().try_into() {
            Ok(fmat) => fmat,
            Err(_) => match &self.mat {
                Material::PolarChecker {
                    color1,
                    color2,
                    ntheta,
                    nphi,
                } => {
                    let dtheta = PI / *ntheta as f64;
                    let dphi = TAU / *nphi as f64;

                    let (theta, phi) = self.spherical_coords(hitpos);
                    let colored = (theta / dtheta) as u32 % 2 == (phi / dphi) as u32 % 2;

                    if colored {
                        FragMaterial::Lambertian { albedo: *color1 }
                    } else {
                        FragMaterial::Lambertian { albedo: *color2 }
                    }
                }

                Material::Texture { map } => {
                    let (theta, phi) = self.spherical_coords(hitpos);
                    let x = phi / TAU;
                    let y = theta / PI;
                    FragMaterial::Lambertian {
                        albedo: map.query(x, y),
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}
