use super::{backgrounds::Background, traits::Entity};
use crate::{
    math::interval::Interval,
    tracer::ray::{hit::Hit, ray::Ray},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct Scene {
    pub entities: Vec<Arc<dyn Entity>>,
    pub background: Background,
}

impl Scene {
    pub fn new(entities: Vec<Arc<dyn Entity>>, background: Background) -> Self {
        Self {
            entities,
            background,
        }
    }
}

impl Scene {
    pub fn hit_by(&self, ray: Ray) -> Option<Hit> {
        let mut interval = Interval::GreaterThan(ray.tmin);

        let mut scene_hit = None;

        // clone the arcs
        for ent in self.entities.clone() {
            if let Some(hit) = ent.hit_by(ray, interval) {
                interval = interval.clamp_high(hit.t);
                scene_hit = Some(hit);
            }
        }

        scene_hit
    }
}
