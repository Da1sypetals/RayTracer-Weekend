use image::RgbImage;
use nalgebra_glm::UVec2;
use rand::rngs::ThreadRng;
use rand_distr::Uniform;
use rayon::iter::ParallelIterator;
use raytrace::{
    camera::camera::CameraBuilder,
    entity::{analytic::sphere::Sphere, scene::Scene, traits::Entity},
    helpers::{
        traits::Color,
        types::{color, vec3},
    },
    materials::material::Material,
    tracer::tracers::tracer_mat::TracerMat,
};
use std::{f64::consts::PI, sync::Arc};

fn main() {
    let yfov = 75.0 / 360.0 * 2.0 * PI;
    let cam = CameraBuilder {
        resolution: UVec2::new(1920, 1080),
        yfov,
        vd: 2.0,
        pos: vec3::zeros(),
        lookat: vec3::new(1.0, 0.0, 0.0),
        up: vec3::new(0.0, 1.0, 0.0),
    }
    .build();

    // ######################## entities ########################

    let blue = Material::Lambertian {
        albedo: color::new(0.1, 0.2, 0.5),
    };
    let green = Material::Lambertian {
        albedo: color::new(0.1, 0.5, 0.2),
    };
    let mirror = Material::Metal {
        albedo: color::new(0.8, 0.8, 0.8),
    };
    let mirror_yellow = Material::Metal {
        albedo: color::new(0.8, 0.6, 0.2),
    };
    let fuzz_purple = Material::FuzzedMetal {
        albedo: color::new(0.58, 0.23, 0.72),
        fuzz: 0.1,
    };

    let mut ents: Vec<Arc<dyn Entity>> = Vec::new();
    ents.push(Arc::new(Sphere::new(vec3::new(5.0, 0.0, 0.0), 1.5, blue)));
    ents.push(Arc::new(Sphere::new(
        vec3::new(5.0, 1.5, -1.5),
        0.5,
        mirror_yellow,
    )));
    ents.push(Arc::new(Sphere::new(vec3::new(5.0, 0.0, 3.5), 2.0, mirror)));
    ents.push(Arc::new(Sphere::new(
        vec3::new(5.0, -0.2, -3.5),
        1.5,
        fuzz_purple,
    )));
    ents.push(Arc::new(Sphere::new(
        vec3::new(5.0, -18.0, 0.0),
        16.0,
        green,
    )));

    // ######################## entities ########################

    let scene = Scene::new(ents);

    let mut img = RgbImage::new(cam.resolution.x, cam.resolution.y);

    let tracer = TracerMat {
        cam,
        scene,
        uniform: Uniform::new(0.0, 1.0),
        reflect_ratio: 0.5,
    };

    img.par_enumerate_pixels_mut().for_each(|(ix, iy, px)| {
        let mut rng = ThreadRng::default();

        let color = tracer.color_at(ix, iy, &mut rng).to_gamma();

        let r = (color.r * 255.0) as u8;
        let g = (color.g * 255.0) as u8;
        let b = (color.b * 255.0) as u8;
        *px = [r, g, b].into();
    });

    img.save("traced.png").unwrap();
}
