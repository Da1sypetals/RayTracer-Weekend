use image::RgbImage;
use indicatif::ParallelProgressIterator;
use rand::rngs::ThreadRng;
use rayon::iter::ParallelIterator;
use raytrace::{helpers::traits::Color, tracer::tracers::tracer_iter::TracerIter};
use std::time::Instant;
mod debug;

fn run() -> anyhow::Result<()> {
    let tracer = TracerIter::configured("config/litup/tracer.toml")?;

    // ########################### Main work ###########################
    let mut img = RgbImage::new(tracer.cam.resolution.x, tracer.cam.resolution.y);
    img.par_enumerate_pixels_mut()
        .progress()
        .for_each(|(ix, iy, px)| {
            let mut rng = ThreadRng::default();

            let color = tracer.color_at(ix, iy, &mut rng).to_gamma();
            *px = color.quantize_u8().into()
        });
    // ######################### Main work end #########################

    img.save(tracer.out_path)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    run()?;
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
    Ok(())
}
