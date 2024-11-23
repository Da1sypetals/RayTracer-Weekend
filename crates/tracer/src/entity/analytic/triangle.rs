use super::commons::Point;
use crate::{
    entity::traits::Entity,
    helpers::types::vec3,
    materials::material::{FragMaterial, Material},
    math::panics::PanickingNormalize,
    tracer::ray::hit::{Hit, Normal},
};

#[derive(Debug)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub mat: Material,

    ab: Point,
    ac: Point,
    normal: vec3,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point, mat: Material) -> Self {
        Self {
            a,
            b,
            c,
            ab: Point {
                world: b.world - a.world,
                uv: b.uv - a.uv,
            },
            ac: Point {
                world: c.world - a.world,
                uv: c.uv - a.uv,
            },
            mat,
            normal: (b.world - a.world)
                .cross(&(c.world - a.world))
                .p_normalize(),
        }
    }
}

impl Entity for Triangle {
    fn hit_by(
        &self,
        ray: crate::tracer::ray::ray::Ray,
        interval: crate::math::interval::Interval,
    ) -> Option<crate::tracer::ray::hit::Hit> {
        if ray.dir.dot(&self.normal).abs() <= f64::EPSILON {
            return None;
        }
        // refer to plane.
        // use texture.
        let a1 = self.a.world.x;
        let a2 = self.a.world.y;
        let a3 = self.a.world.z;
        //
        let ab1 = self.ab.world.x;
        let ab2 = self.ab.world.y;
        let ab3 = self.ab.world.z;
        //
        let ac1 = self.ac.world.x;
        let ac2 = self.ac.world.y;
        let ac3 = self.ac.world.z;
        //
        let d1 = ray.dir.x;
        let d2 = ray.dir.y;
        let d3 = ray.dir.z;
        //
        let o1 = ray.orig.x;
        let o2 = ray.orig.y;
        let o3 = ray.orig.z;

        let k1 =
            (-a1 * ac2 * d3 + a1 * ac3 * d2 + a2 * ac1 * d3 - a2 * ac3 * d1 - a3 * ac1 * d2
                + a3 * ac2 * d1
                + ac1 * d2 * o3
                - ac1 * d3 * o2
                - ac2 * d1 * o3
                + ac2 * d3 * o1
                + ac3 * d1 * o2
                - ac3 * d2 * o1)
                / (ab1 * ac2 * d3 - ab1 * ac3 * d2 - ab2 * ac1 * d3
                    + ab2 * ac3 * d1
                    + ab3 * ac1 * d2
                    - ab3 * ac2 * d1);
        let k2 =
            (a1 * ab2 * d3 - a1 * ab3 * d2 - a2 * ab1 * d3 + a2 * ab3 * d1 + a3 * ab1 * d2
                - a3 * ab2 * d1
                - ab1 * d2 * o3
                + ab1 * d3 * o2
                + ab2 * d1 * o3
                - ab2 * d3 * o1
                - ab3 * d1 * o2
                + ab3 * d2 * o1)
                / (ab1 * ac2 * d3 - ab1 * ac3 * d2 - ab2 * ac1 * d3
                    + ab2 * ac3 * d1
                    + ab3 * ac1 * d2
                    - ab3 * ac2 * d1);
        let t = (a1 * ab2 * ac3 - a1 * ab3 * ac2 - a2 * ab1 * ac3
            + a2 * ab3 * ac1
            + a3 * ab1 * ac2
            - a3 * ab2 * ac1
            - ab1 * ac2 * o3
            + ab1 * ac3 * o2
            + ab2 * ac1 * o3
            - ab2 * ac3 * o1
            - ab3 * ac1 * o2
            + ab3 * ac2 * o1)
            / (ab1 * ac2 * d3 - ab1 * ac3 * d2 - ab2 * ac1 * d3
                + ab2 * ac3 * d1
                + ab3 * ac1 * d2
                - ab3 * ac2 * d1);

        let v = ray.orig - self.a.world;
        // dbg!(normal);
        if interval.contains(t) && k1 >= 0.0 && k2 >= 0.0 && k1 + k2 <= 1.0 {
            let material = self.frag_material(k1, k2);
            Some(Hit {
                in_dir: ray.dir,
                pos: ray.at(t),
                material,
                t,
                normal: Normal::Outward(if v.dot(&self.normal) >= 0.0 {
                    self.normal
                } else {
                    -self.normal
                }),
            })
        } else {
            None
        }
    }

    fn material(&self) -> crate::materials::material::Material {
        self.mat.clone()
    }
}

impl Triangle {
    fn frag_material(&self, k1: f64, k2: f64) -> FragMaterial {
        match self.mat.clone().try_into() {
            Ok(fmat) => fmat,
            Err(_) => match &self.mat {
                Material::PolarChecker { .. } => panic!("Unsupported material!"),

                Material::Texture { map } => {
                    let uv = self.a.uv + self.ab.uv * k1 + self.ac.uv * k2;
                    FragMaterial::Lambertian {
                        albedo: map.query(uv.x, uv.y),
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}
