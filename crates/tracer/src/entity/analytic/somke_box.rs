use rand::thread_rng;

use super::{commons::Point, parallelogram::Parallelogram};
use crate::{
    entity::traits::Entity,
    helpers::types::vec3,
    materials::material::{FragMaterial, Material},
    math::distributions::sample_uniform_01,
    tracer::ray::hit::{Hit, Normal},
};

#[derive(Debug)]
pub struct SmokeBox {
    pub a: vec3,
    pub b: vec3,
    pub c: vec3,
    pub d: vec3,

    pub k: f64,
    pub mat: Material,

    ab: vec3,
    ac: vec3,
    ad: vec3,

    faces: [Parallelogram; 6],
}

impl SmokeBox {
    #[rustfmt::skip]
    pub fn new(a: vec3, b: vec3, c: vec3, d: vec3, mat: Material) -> Self {
        let k = if let Material::Smoke { k } = mat { k } else {panic!("Unsupported material!")};
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
        Self { a, b, c, d, ab:b-a, ac:c-a, ad:d-a, k, mat, faces }
    }
}

impl Entity for SmokeBox {
    fn hit_by(
        &self,
        ray: crate::tracer::ray::ray::Ray,
        interval: crate::math::interval::Interval,
    ) -> Option<crate::tracer::ray::hit::Hit> {
        let mut nearest_hit = None;
        let mut interval = interval;

        // find a nearest hit
        for face in &self.faces {
            if let Some(hit) = face.hit_by(ray, interval) {
                interval = interval.clamp_high(hit.t);
                nearest_hit = Some(hit);
            }
        }

        if self.contains(ray.orig) {
            // if orig in box, smoke
            if let Some(hit) = nearest_hit {
                let mut rng = thread_rng();
                let rand = sample_uniform_01(&mut rng);

                let t = if rand > self.k * hit.t {
                    // pass through
                    hit.t
                } else {
                    rand / self.k
                };
                if interval.contains(t) {
                    let hitpos = ray.at(t);
                    Some(Hit {
                        in_dir: ray.dir,
                        pos: hitpos,
                        material: FragMaterial::Smoke,
                        t,
                        normal: Normal::Outward(vec3::new(1.0, 0.0, 0.0)),
                    })
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            // origin is outside, transparent
            nearest_hit.map(|mut hit| {
                hit.material = FragMaterial::Transparent;
                hit
            })
        }
    }
}

impl SmokeBox {
    fn contains(&self, p: vec3) -> bool {
        //
        let ab1 = self.ab.x;
        let ab2 = self.ab.y;
        let ab3 = self.ab.z;
        //
        let ac1 = self.ac.x;
        let ac2 = self.ac.y;
        let ac3 = self.ac.z;
        //
        let ad1 = self.ad.x;
        let ad2 = self.ad.y;
        let ad3 = self.ad.z;
        //
        let p1 = p.x;
        let p2 = p.y;
        let p3 = p.z;
        //
        let a1 = self.a.x;
        let a2 = self.a.y;
        let a3 = self.a.z;
        //

        let k1 = (-a1 * ac2 * ad3 + a1 * ac3 * ad2 + a2 * ac1 * ad3
            - a2 * ac3 * ad1
            - a3 * ac1 * ad2
            + a3 * ac2 * ad1
            + ac1 * ad2 * p3
            - ac1 * ad3 * p2
            - ac2 * ad1 * p3
            + ac2 * ad3 * p1
            + ac3 * ad1 * p2
            - ac3 * ad2 * p1)
            / (ab1 * ac2 * ad3 - ab1 * ac3 * ad2 - ab2 * ac1 * ad3
                + ab2 * ac3 * ad1
                + ab3 * ac1 * ad2
                - ab3 * ac2 * ad1);
        let k2 = (a1 * ab2 * ad3 - a1 * ab3 * ad2 - a2 * ab1 * ad3
            + a2 * ab3 * ad1
            + a3 * ab1 * ad2
            - a3 * ab2 * ad1
            - ab1 * ad2 * p3
            + ab1 * ad3 * p2
            + ab2 * ad1 * p3
            - ab2 * ad3 * p1
            - ab3 * ad1 * p2
            + ab3 * ad2 * p1)
            / (ab1 * ac2 * ad3 - ab1 * ac3 * ad2 - ab2 * ac1 * ad3
                + ab2 * ac3 * ad1
                + ab3 * ac1 * ad2
                - ab3 * ac2 * ad1);
        let k3 = (-a1 * ab2 * ac3 + a1 * ab3 * ac2 + a2 * ab1 * ac3
            - a2 * ab3 * ac1
            - a3 * ab1 * ac2
            + a3 * ab2 * ac1
            + ab1 * ac2 * p3
            - ab1 * ac3 * p2
            - ab2 * ac1 * p3
            + ab2 * ac3 * p1
            + ab3 * ac1 * p2
            - ab3 * ac2 * p1)
            / (ab1 * ac2 * ad3 - ab1 * ac3 * ad2 - ab2 * ac1 * ad3
                + ab2 * ac3 * ad1
                + ab3 * ac1 * ad2
                - ab3 * ac2 * ad1);

        k1 >= 0.0 && k1 <= 1.0 && k2 >= 0.0 && k2 <= 1.0 && k3 >= 0.0 && k3 <= 1.0
    }
}
