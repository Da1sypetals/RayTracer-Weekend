use image::RgbImage;
use indicatif::ProgressIterator;
use ndarray::prelude::ShapeBuilder;
use ndarray::Array;
use rand::rngs::ThreadRng;
use rayon::iter::{ParallelBridge, ParallelIterator};
use raytrace::{
    helpers::{traits::Color, types::vec3},
    tracer::tracers::tracer_animated::TracerAnimated,
};

fn main() -> anyhow::Result<()> {
    let mut tracer = TracerAnimated::configured("config/blurred/tracer.toml")?;

    // ########################### Main work ###########################
    let (x, y) = (
        tracer.cam.resolution.x as usize,
        tracer.cam.resolution.y as usize,
    );
    let mut colors: Array<vec3, _> = Array::zeros((x, y).f());

    for i in 0..tracer.n_step {
        println!("[raytrace] timestep {}", i);
        let time = i as f64 / tracer.n_step as f64;
        tracer.scene.step_at(time);
        colors
            .indexed_iter_mut()
            .progress()
            .par_bridge()
            .for_each(|((ix, iy), px)| {
                //
                let mut rng = ThreadRng::default();
                let color = tracer.color_at(ix as u32, iy as u32, &mut rng).to_gamma();
                *px += color.vector();
            });
    }

    let mut img = RgbImage::new(tracer.cam.resolution.x, tracer.cam.resolution.y);

    img.enumerate_pixels_mut()
        .progress()
        .for_each(|(ix, iy, px)| {
            let index = (ix as usize, iy as usize);
            let color = colors[index] / tracer.n_step as f64;
            *px = color.to_gamma().quantize_u8().into()
        });

    Ok(())
}
