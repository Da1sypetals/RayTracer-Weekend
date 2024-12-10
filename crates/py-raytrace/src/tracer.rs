use pyo3::pymodule;

#[pymodule]
pub mod tracer {
    use image::RgbImage;
    use indicatif::ParallelProgressIterator;
    use pyo3::{
        exceptions::{PyException, PyIOError},
        pyclass, pymethods, PyResult,
    };
    use rand::rngs::ThreadRng;
    use rayon::iter::ParallelIterator;
    use raytrace::{
        camera::camera_lens::{LensCamera, LensCameraBuilder},
        entity::scene::Scene as RenderScene,
        helpers::traits::Color,
        math::angles::deg2rad,
        tracer::tracers::tracer_iter::TracerIter,
    };

    #[pyclass]
    pub struct RayTracer {
        tracer: TracerIter,
    }

    #[pyclass]
    pub struct Camera {
        pub cam: LensCamera,
    }

    #[pymethods]
    impl Camera {
        #[new]
        pub fn new(
            defocus_angle: f64,
            resolution: [u32; 2],
            yfov: f64,
            viewport_distance: f64,
            pos: [f64; 3],
            lookat: [f64; 3],
            up: [f64; 3],
        ) -> PyResult<Self> {
            let cb = LensCameraBuilder {
                defocus_angle: deg2rad(defocus_angle),
                yfov: deg2rad(yfov),
                resolution: resolution.into(),
                viewport_distance,
                pos: pos.into(),
                lookat: lookat.into(),
                up: up.into(),
            };
            Ok(Self { cam: cb.build() })
        }
    }

    #[pyclass]
    pub struct Scene {
        pub scene: RenderScene,
    }

    #[pymethods]
    impl Scene {
        #[new]
        pub fn load(path: String) -> PyResult<Self> {
            let scene = RenderScene::configured(&path)
                .map_err(|e| PyException::new_err(e.to_string()))?;
            Ok(Self { scene })
        }
    }

    #[pymethods]
    impl RayTracer {
        #[new]
        pub fn new(
            cam: &mut Camera,
            scene: &mut Scene,
            spp: usize,
            out_path: String,
        ) -> PyResult<Self> {
            let res = Self {
                tracer: TracerIter {
                    cam: cam.cam.clone(),
                    scene: scene.scene.clone(),
                    spp,
                    out_path,
                },
            };

            Ok(res)
        }

        pub fn render(&self) -> PyResult<()> {
            // ########################### Main work ###########################
            let mut img =
                RgbImage::new(self.tracer.cam.resolution.x, self.tracer.cam.resolution.y);
            img.par_enumerate_pixels_mut()
                .progress()
                .for_each(|(ix, iy, px)| {
                    let mut rng = ThreadRng::default();

                    let color = self.tracer.color_at(ix, iy, &mut rng).to_gamma();
                    *px = color.quantize_u8().into()
                });
            // ######################### Main work end #########################

            img.save(&self.tracer.out_path)
                .map_err(|e| PyIOError::new_err(e.to_string()))?;
            Ok(())
        }
    }
}
