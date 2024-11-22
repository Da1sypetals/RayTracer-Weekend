use crate::{
    entity::traits::Entity,
    helpers::types::vec3,
    materials::material::Material,
    math::panics::PanickingNormalize,
    tracer::ray::hit::{Hit, Normal},
};

pub struct Plane {
    pub point: vec3,
    pub normal: vec3,
    pub mat: Material,
}

impl Plane {
    pub fn new(point: vec3, normal: vec3, mat: Material) -> Self {
        Self {
            point,
            normal: normal.p_normalize(),
            mat,
        }
    }
}

impl Entity for Plane {
    fn hit_by(
        &self,
        ray: crate::tracer::ray::ray::Ray,
        interval: crate::math::interval::Interval,
    ) -> Option<crate::tracer::ray::hit::Hit> {
        let d_dot_n = ray.dir.dot(&self.normal);
        if d_dot_n.abs() < f64::EPSILON {
            None
        } else {
            let t = self.normal.dot(&(self.point - ray.orig)) / d_dot_n;
            if interval.contains(t) {
                Some(Hit {
                    in_dir: ray.dir,
                    pos: ray.at(t),
                    material: self.mat,
                    t,
                    normal: Normal::Outward(self.normal),
                })
            } else {
                None
            }
        }
    }

    fn material(&self) -> crate::materials::material::Material {
        self.mat
    }
}
