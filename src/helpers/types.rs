pub type vec<const N: usize> = nalgebra::SVector<f64, N>;
pub type vec2 = vec<2>;
pub type vec3 = vec<3>;
pub type color = vec<3>;
#[derive(Debug, Clone, Copy)]
pub struct GammaColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl GammaColor {
    pub fn quantize_u8(self) -> [u8; 3] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        ]
    }
}

pub type mat<const M: usize, const N: usize> = nalgebra::SMatrix<f64, M, N>;
pub type mat3 = mat<3, 3>;
