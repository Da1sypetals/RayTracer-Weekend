use crate::{
    math::interval::Interval,
    tracer::ray::{hit::Hit, ray::Ray},
};
use std::{fmt::Debug, sync::Arc};

pub trait Entity: Sync + Send + Debug {
    fn hit_by(&self, ray: Ray, interval: Interval) -> Option<Hit>;
}

pub trait AnimatedEntity: Entity {
    fn step(&self, t: f64) -> Arc<dyn AnimatedEntity>;
}
