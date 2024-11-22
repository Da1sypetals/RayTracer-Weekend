use crate::{
    math::interval::Interval,
    tracer::ray::{hit::Hit, ray::Ray},
};

use super::traits::AnimatedEntity;
use std::sync::Arc;

pub struct AnimatedScene {
    pub start_entities: Vec<Arc<dyn AnimatedEntity>>,
    pub entities: Vec<Arc<dyn AnimatedEntity>>,
}

impl AnimatedScene {
    pub fn new(entities: Vec<Arc<dyn AnimatedEntity>>) -> Self {
        Self {
            start_entities: entities.clone(),
            entities,
        }
    }

    pub fn step_at(&mut self, time: f64) {
        self.entities = self
            .start_entities
            .iter()
            .map(|e| e.step_at(time))
            .collect();
    }
}

impl AnimatedScene {
    pub fn hit_by(&self, ray: Ray) -> Option<Hit> {
        let mut interval = Interval::GreaterThan(ray.tmin);

        let mut scene_hit = None;

        // clone the arcs
        for ent in self.entities.clone() {
            if let Some(hit) = ent.hit_by(ray, interval) {
                scene_hit = Some(hit);
                interval = interval.clamp_high(hit.t);
            }
        }

        scene_hit
    }
}
