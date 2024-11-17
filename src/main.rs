use image::RgbImage;
use indicatif::ParallelProgressIterator;
use nalgebra_glm::UVec2;
use rand::rngs::ThreadRng;
use rayon::iter::ParallelIterator;
use raytrace::{
    camera::camera_lens::LensCameraBuilder,
    entity::{analytic::sphere::Sphere, scene::Scene, traits::Entity},
    helpers::{
        traits::Color,
        types::{color, vec3},
    },
    materials::material::Material,
    math::angles::deg2rad,
    tracer::tracers::tracer_lens::TracerLens,
};
use std::sync::Arc;

fn main() {
    let yfov = deg2rad(75.0);
    // let cam = CameraBuilder {
    //     resolution: UVec2::new(1920, 1080),
    //     yfov,
    //     vd: 2.0,
    //     pos: vec3::zeros(),
    //     lookat: vec3::new(1.0, 0.0, 0.0),
    //     up: vec3::new(0.0, 1.0, 0.0),
    //     model: CameraModel::Pinhole,
    // }
    // .build();
    let cam = LensCameraBuilder {
        defocus_angle: deg2rad(3.0),
        resolution: UVec2::new(1920, 1080),
        yfov,
        vd: 4.0,
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
    let mirror_yellow = Material::Metal {
        albedo: color::new(0.8, 0.6, 0.2),
    };
    let fuzz_purple = Material::FuzzedMetal {
        albedo: color::new(0.58, 0.23, 0.72),
        fuzz: 0.1,
    };
    let transp = Material::Dielectric { eta: 0.75 };
    let transp_inner = Material::Dielectric { eta: 1.33 };

    let mut ents: Vec<Arc<dyn Entity>> = Vec::new();
    let center = ents.push(Arc::new(Sphere::new(vec3::new(4.0, 0.0, 0.0), 1.5, blue)));
    let left_top = ents.push(Arc::new(Sphere::new(
        vec3::new(5.0, 1.5, -1.5),
        0.6,
        fuzz_purple,
    )));
    let right = ents.push(Arc::new(Sphere::new(vec3::new(5.0, 0.0, 3.5), 2.0, transp)));
    let right_inner = ents.push(Arc::new(Sphere::new(
        vec3::new(6.0, 0.0, 3.5),
        1.6,
        transp_inner,
    )));
    let left = ents.push(Arc::new(Sphere::new(
        vec3::new(7.0, -0.6, -3.5),
        1.5,
        mirror_yellow,
    )));
    let ground = ents.push(Arc::new(Sphere::new(
        vec3::new(5.0, -18.0, 0.0),
        16.0,
        green,
    )));

    // ######################## entities ########################

    let scene = Scene::new(ents);

    let mut img = RgbImage::new(cam.resolution.x, cam.resolution.y);

    let tracer = TracerLens {
        cam,
        scene,
        reflect_ratio: 0.5,
    };

    // ######################## Main work ########################
    img.par_enumerate_pixels_mut()
        .progress()
        .for_each(|(ix, iy, px)| {
            let mut rng = ThreadRng::default();

            let color = tracer.color_at(ix, iy, &mut rng).to_gamma();

            *px = color.quantize_u8().into()
        });

    img.save("traced.png").unwrap();
}
