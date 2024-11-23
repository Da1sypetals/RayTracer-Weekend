use super::errors::MaterialError;
use crate::helpers::types::{color, vec3};
use image::ImageBuffer;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureMap {
    pub resolution: u32,
    pub path: String,
    #[serde(skip)]
    pub map: ImageBuffer<image::Rgb<f32>, Vec<f32>>,
}

impl TextureMap {
    /// x, y should be in [0, 1]
    pub fn query(&self, x: f64, y: f64) -> vec3 {
        let x = (x * self.resolution as f64) as u32;
        let y = (y * self.resolution as f64) as u32;

        vec3::new(
            self.map[(x, y)][0] as f64,
            self.map[(x, y)][1] as f64,
            self.map[(x, y)][2] as f64,
        )
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum Material {
    Lambertian {
        albedo: color,
    },
    Metal {
        albedo: color,
    },
    FuzzedMetal {
        albedo: color,
        /// between \[0, 1\], 0 for perfect reflection.
        /// - `fuzz` :radius of fuzz sphere.
        fuzz: f64,
    },
    Dielectric {
        eta: f64,
    },
    // below are not used in generation.
    PolarChecker {
        color1: color,
        color2: color,
        ntheta: u32,
        nphi: u32,
    },
    Texture {
        map: Arc<TextureMap>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum FragMaterial {
    Lambertian {
        albedo: color,
    },
    Metal {
        albedo: color,
    },
    FuzzedMetal {
        albedo: color,
        /// between \[0, 1\], 0 for perfect reflection.
        /// - `fuzz` :radius of fuzz sphere.
        fuzz: f64,
    },
    Dielectric {
        eta: f64,
    },
}

impl TryFrom<Material> for FragMaterial {
    type Error = MaterialError;

    fn try_from(value: Material) -> Result<Self, Self::Error> {
        match value {
            Material::Lambertian { albedo } => Ok(FragMaterial::Lambertian { albedo }),
            Material::Metal { albedo } => Ok(FragMaterial::Metal { albedo }),
            Material::FuzzedMetal { albedo, fuzz } => {
                Ok(FragMaterial::FuzzedMetal { albedo, fuzz })
            }
            Material::Dielectric { eta } => Ok(FragMaterial::Dielectric { eta }),
            Material::PolarChecker { .. } => Err(MaterialError::CannotConvert {
                mat_type: "polar checker".into(),
            }),
            Material::Texture { .. } => Err(MaterialError::CannotConvert {
                mat_type: "texture".into(),
            }),
        }
    }
}
