use std::cell::LazyCell;

use crate::helpers::types::{vec2, vec3};
use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Uniform, UnitDisc, UnitSphere};

pub const SPHERE_DISTRIBUTION: UnitSphere = UnitSphere;
pub fn sample_on_sphere(rng: &mut ThreadRng) -> vec3 {
    SPHERE_DISTRIBUTION.sample(rng).into()
}

pub const DISK_DISTRIBUTION: UnitDisc = UnitDisc;
pub fn sample_on_disk(rng: &mut ThreadRng) -> vec2 {
    DISK_DISTRIBUTION.sample(rng).into()
}

pub const UNIFORM_01: LazyCell<Uniform<f64>> = LazyCell::new(|| Uniform::new(0.0, 1.0));
pub fn sample_uniform_01(rng: &mut ThreadRng) -> f64 {
    UNIFORM_01.sample(rng)
}
