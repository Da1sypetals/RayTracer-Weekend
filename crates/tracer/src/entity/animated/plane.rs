use std::sync::Arc;

use glm::{lerp, slerp};

use crate::{
    entity::{
        analytic::plane::Plane,
        traits::{AnimatedEntity, Entity},
    },
    helpers::types::vec3,
    math::panics::PanickingNormalize,
};

#[derive(Debug)]
pub struct AnimatedPlane {
    plane: Plane,
    delta_point: vec3,
    new_normal: vec3,
}

impl AnimatedPlane {
    pub fn new(plane: Plane, delta_point: vec3, new_normal: vec3) -> Self {
        Self {
            plane,
            delta_point,
            new_normal: new_normal.p_normalize(),
        }
    }
}

impl Entity for AnimatedPlane {
    #[inline]
    fn hit_by(
        &self,
        ray: crate::tracer::ray::ray::Ray,
        interval: crate::math::interval::Interval,
    ) -> Option<crate::tracer::ray::hit::Hit> {
        self.plane.hit_by(ray, interval)
    }

    #[inline]
    fn material(&self) -> crate::materials::material::Material {
        self.plane.material()
    }
}

impl AnimatedEntity for AnimatedPlane {
    fn step(&self, t: f64) -> std::sync::Arc<dyn AnimatedEntity> {
        let point = lerp(&self.plane.point, &(self.plane.point + self.delta_point), t);
        let normal = slerp(&self.plane.normal, &self.new_normal, t);
        Arc::new(Self {
            plane: Plane {
                point,
                normal,
                mat: self.plane.mat,
            },
            ..*self
        })
    }
}
