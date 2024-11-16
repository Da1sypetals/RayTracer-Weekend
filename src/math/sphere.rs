use crate::helpers::types::vec3;
use rand::rngs::ThreadRng;
use rand_distr::{Distribution, UnitSphere};

pub const SPHERE_DIST: UnitSphere = UnitSphere;
pub fn sample_on_sphere(rng: &mut ThreadRng) -> vec3 {
    SPHERE_DIST.sample(rng).into()
}
