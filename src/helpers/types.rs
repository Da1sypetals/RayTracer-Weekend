pub type vec<const N: usize> = nalgebra::SVector<f64, N>;
pub type vec2 = vec<2>;
pub type vec3 = vec<3>;
pub type color = vec<3>;

pub type mat<const M: usize, const N: usize> = nalgebra::SMatrix<f64, M, N>;
pub type mat3 = mat<3, 3>;
