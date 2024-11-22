use crate::{
    materials::material::Material,
    math::interval::Interval,
    tracer::ray::{hit::Hit, ray::Ray},
};

pub trait Entity: Sync + Send {
    fn hit_by(&self, ray: Ray, interval: Interval) -> Option<Hit>;
    fn material(&self) -> Material;
}
