use crate::{
    entity::traits::Entity,
    helpers::types::vec3,
    materials::material::Material,
    math::panics::PanickingNormalize,
    tracer::ray::hit::{Hit, Normal},
};

#[derive(Debug)]
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
            let v = ray.orig - self.point;
            let t = self.normal.dot(&-v) / d_dot_n;
            let material = self.mat.clone().try_into().expect("Material error!");

            if interval.contains(t) {
                Some(Hit {
                    in_dir: ray.dir,
                    pos: ray.at(t),
                    material,
                    t,
                    normal: Normal::Outward(if v.dot(&self.normal) >= 0.0 {
                        self.normal
                    } else {
                        -self.normal
                    }),
                })
            } else {
                None
            }
        }
    }

}
