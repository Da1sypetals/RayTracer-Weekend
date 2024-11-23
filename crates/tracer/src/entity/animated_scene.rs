use super::{backgrounds::Background, traits::AnimatedEntity};
use crate::{
    math::interval::Interval,
    tracer::ray::{hit::Hit, ray::Ray},
};
use std::sync::Arc;

#[derive(Debug)]
pub struct AnimatedScene {
    pub entities: Vec<Arc<dyn AnimatedEntity>>,
    pub n_step: u32,
    pub i_step: u32,
    pub dt: f64,
    pub background: Background,
}

impl AnimatedScene {
    pub fn new(
        entities: Vec<Arc<dyn AnimatedEntity>>,
        background: Background,
        n_step: u32,
    ) -> Self {
        Self {
            entities,
            n_step,
            i_step: 0,
            dt: 1.0 / n_step as f64,
            background,
        }
    }

    pub fn step(&mut self) -> Option<u32> {
        self.i_step += 1;
        if self.i_step >= self.n_step {
            None
        } else {
            self.entities = self.entities.iter().map(|e| e.step(self.dt)).collect();
            Some(self.i_step)
        }
    }
}

impl AnimatedScene {
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
