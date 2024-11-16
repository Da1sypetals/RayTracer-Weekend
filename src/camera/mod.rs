pub mod camera;

#[cfg(test)]
pub mod tests {
    use std::f64::consts::PI;

    use glm::UVec2;

    use crate::helpers::types::vec3;

    use super::camera::CameraBuilder;

    #[test]
    fn test_cam() {
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

        let lt = cam.image_space.orig;
        let rb = cam.image_space.pixel_center_at(1919, 1079);
        let diff = (lt + rb).magnitude() - 4.0;
        dbg!(diff);
        assert!(diff < 1e-4);
    }
}
