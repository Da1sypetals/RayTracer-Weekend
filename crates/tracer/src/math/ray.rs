use crate::helpers::types::vec3;

pub trait RayDir {
    fn reflected_by(&self, normal: &Self) -> Self;
    fn refracted_by(&self, normal: &Self, eta_ratio: f64) -> Self;
}

impl RayDir for vec3 {
    fn reflected_by(&self, normal: &Self) -> Self {
        self - 2.0 * normal.dot(self) * normal
    }

    fn refracted_by(&self, normal: &Self, eta_ratio: f64) -> Self {
        let cos_theta = (self.dot(&-normal)).min(1.0);
        let r_out_perp = eta_ratio * (*self + cos_theta * *normal);
        let r_out_parallel = -((1.0 - r_out_perp.norm_squared()).abs().sqrt()) * *normal;
        r_out_perp + r_out_parallel
    }
}

pub fn reflectance(cosine: f64, eta_ratio: f64) -> f64 {
    let mut r0 = (1.0 - eta_ratio) / (1.0 + eta_ratio);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
