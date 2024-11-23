use crate::{
    entity::{
        analytic::sphere::Sphere,
        traits::{AnimatedEntity, Entity},
    },
    helpers::types::vec3,
};
use glm::lerp;
use std::sync::Arc;

#[derive(Debug)]
pub struct AnimatedSphere {
    sphere: Sphere,
    delta: vec3,
}

impl AnimatedSphere {
    pub fn new(sphere: Sphere, delta: vec3) -> Self {
        Self { sphere, delta }
    }
}

impl Entity for AnimatedSphere {
    #[inline]
    fn hit_by(
        &self,
        ray: crate::tracer::ray::ray::Ray,
        interval: crate::math::interval::Interval,
    ) -> Option<crate::tracer::ray::hit::Hit> {
        self.sphere.hit_by(ray, interval)
    }
}

impl AnimatedEntity for AnimatedSphere {
    fn step(&self, t: f64) -> std::sync::Arc<dyn AnimatedEntity> {
        Arc::new(AnimatedSphere {
            sphere: Sphere {
                center: lerp(&self.sphere.center, &(self.sphere.center + self.delta), t),
                radius: self.sphere.radius,
                mat: self.sphere.mat.clone(),
            },
            delta: self.delta,
        })
    }
}
