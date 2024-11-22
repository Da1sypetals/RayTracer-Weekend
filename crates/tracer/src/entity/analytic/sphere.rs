use crate::{
    entity::traits::Entity,
    helpers::types::vec3,
    materials::material::Material,
    math::panics::{PanickingFloatMethods, PanickingNormalize},
    tracer::ray::hit::{Hit, Normal},
};

pub struct Sphere {
    center: vec3,
    radius: f64,
    mat: Material,
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

            let pos = ray.at(t);
            let v = ray.orig + ray.dir * interval.min().expect("Ray should have minimum t!")
                - self.center;
            let normal = if v.norm_squared() > self.radius * self.radius {
                Normal::Outward((pos - self.center).p_normalize())
            } else {
                Normal::Inward((self.center - pos).p_normalize())
            };
            Some(Hit {
                in_dir: ray.dir,
                pos,
                normal,
                t,
                material: self.mat,
            })
        }
    }

    fn material(&self) -> crate::materials::material::Material {
        self.mat
    }
}
