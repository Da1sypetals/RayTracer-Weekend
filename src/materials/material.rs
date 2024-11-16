use crate::helpers::types::color;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian {
        albedo: color,
    },
    Metal {
        albedo: color,
    },
    FuzzedMetal {
        albedo: color,
        /// between [0, 1], 0 for perfect reflection.
        /// radius of fuzz sphere.
        fuzz: f64,
    },
}