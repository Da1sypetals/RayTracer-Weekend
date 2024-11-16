use crate::{
    math::interval::Interval,
    tracer::ray::{hit::Hit, ray::Ray},
};

use super::traits::Entity;
use std::sync::Arc;

pub struct Scene {
    pub entities: Vec<Arc<dyn Entity>>,
}

impl Scene {
    pub fn new(entities: Vec<Arc<dyn Entity>>) -> Self {
        Self { entities }
    }
}

impl Scene {
    pub fn hit_by(&self, ray: Ray) -> Option<Hit> {
        let interval = Interval::GreaterThan(ray.tmin);

        let mut scene_hit = None;

        // clone the arcs
        for ent in self.entities.clone() {
            if let Some(hit) = ent.hit_by(ray, interval) {
                scene_hit = Some(hit);
                interval.clamp_max(hit.t);
            }
        }

        scene_hit
    }
}
