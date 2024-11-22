use std::sync::Arc;

use crate::{
    materials::material::Material,
    math::interval::Interval,
    tracer::ray::{hit::Hit, ray::Ray},
};

pub trait Entity: Sync + Send {
    fn hit_by(&self, ray: Ray, interval: Interval) -> Option<Hit>;
    fn material(&self) -> Material;
}

pub trait AnimatedEntity: Sync + Send + Entity {
    fn step_at(&self, t: f64) -> Arc<dyn AnimatedEntity>;
}
