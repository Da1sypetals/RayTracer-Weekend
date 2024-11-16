use image::RgbImage;
use nalgebra_glm::UVec2;
use rand::rngs::ThreadRng;
use rand_distr::{Uniform, UnitSphere};
use rayon::iter::ParallelIterator;
use raytrace::{
    camera::camera::CameraBuilder,
    entity::{analytic::sphere::Sphere, scene::Scene, traits::Entity},
    helpers::types::vec3,
    tracer::versions::tracer_aa::TracerAa,
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

    let mut ents: Vec<Arc<dyn Entity>> = Vec::new();
    ents.push(Arc::new(Sphere::new(vec3::new(5.0, 0.0, 0.0), 2.0)));
    ents.push(Arc::new(Sphere::new(vec3::new(5.0, -18.0, 0.0), 16.0)));

    let scene = Scene::new(ents);

    let mut img = RgbImage::new(cam.resolution.x, cam.resolution.y);
    // let tracer = TracerSimple {
    //     cam,
    //     scene,
    //     sphere_dist: UnitSphere,
    // };

    let tracer = TracerAa {
        cam,
        scene,
        sphere_dist: UnitSphere,
        uniform: Uniform::new(0.0, 1.0),
    };

    img.par_enumerate_pixels_mut().for_each(|(ix, iy, px)| {
        let mut rng = ThreadRng::default();

        let color = tracer.color_at(ix, iy, &mut rng);

        let r = (color[0] * 255.0) as u8;
        let g = (color[1] * 255.0) as u8;
        let b = (color[2] * 255.0) as u8;
        *px = [r, g, b].into();
    });

    img.save("traced.png").unwrap();
}
