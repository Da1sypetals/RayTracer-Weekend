use super::{commons::Point, parallelogram::Parallelogram};
use crate::{
    entity::traits::Entity,
    helpers::types::{vec2, vec3},
    materials::material::Material,
};

fn default_uv() -> vec2 {
    vec2::new(0.0, 1.0)
}

#[derive(Debug)]
pub struct Box {
    pub a: vec3,
    pub b: vec3,
    pub c: vec3,
    pub d: vec3,
    
    pub mat: Material,

    faces: [Parallelogram; 6],
}

impl Box {
    #[rustfmt::skip]
    pub fn new(a: vec3, b: vec3, c: vec3, d: vec3, mat: Material) -> Self {
        let e = b + c - a;
        let f = b + d - a;
        let g = d + c - a;
        let faces = [
            Parallelogram::new(Point::world(a), Point::world(b), Point::world(c), mat.clone()),
            Parallelogram::new(Point::world(a), Point::world(b), Point::world(d), mat.clone()),
            Parallelogram::new(Point::world(a), Point::world(c), Point::world(d), mat.clone()),
            Parallelogram::new(Point::world(b), Point::world(e), Point::world(f), mat.clone()),
            Parallelogram::new(Point::world(c), Point::world(e), Point::world(g), mat.clone()),
            Parallelogram::new(Point::world(d), Point::world(f), Point::world(g), mat.clone()),
        ];
        Self { a, b, c, d, mat, faces }
    }
}

impl Entity for Box {
    fn hit_by(
        &self,
        ray: crate::tracer::ray::ray::Ray,
        interval: crate::math::interval::Interval,
    ) -> Option<crate::tracer::ray::hit::Hit> {
        for face in &self.faces {
            if let Some(hit) = face.hit_by(ray, interval) {
                return Some(hit);
            }
        }
        None
    }

    fn material(&self) -> crate::materials::material::Material {
        self.mat.clone()
    }
}
