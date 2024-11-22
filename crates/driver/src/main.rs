use image::RgbImage;
use indicatif::ParallelProgressIterator;
use rand::rngs::ThreadRng;
use rayon::iter::ParallelIterator;
use raytrace::{helpers::traits::Color, tracer::tracers::tracer_lens::TracerLens};
mod debug;

fn main() -> anyhow::Result<()> {
    let tracer = TracerLens::configured("config/lens/tracer.toml")?;

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
