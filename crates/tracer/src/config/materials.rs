use image::imageops::FilterType;
use serde::Serialize;
use std::{collections::BTreeMap, fs, sync::Arc};
use toml::Value;

use crate::{
    helpers::types::color,
    materials::material::{Material, TextureMap},
};

use super::toml_common::{value_get_into, value_get_into_option};

#[derive(Debug, Serialize)]
pub struct MaterialMap {
    pub map: BTreeMap<String, Material>,
}

impl MaterialMap {
    pub fn configured(path: &str) -> anyhow::Result<Self> {
        let val: Value = toml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        let res: MaterialMap = val.into();
        Ok(res)
    }
}

impl From<Value> for MaterialMap {
    fn from(value: Value) -> Self {
        let mats = value
            .get("materials")
            .expect("Expected list [[materials]]")
            .as_array()
            .unwrap();

        let map = mats
            .into_iter()
            .map(|mat| {
                let name = mat
                    .get("name")
                    .expect("Expect a name")
                    .as_str()
                    .expect("Expect name to be string");

                let mat_type = mat
                    .get("type")
                    .expect("Expect a material type")
                    .as_str()
                    .expect("Expect mat type to be string");

                let material = match mat_type {
                    "Lambertian" => Material::Lambertian {
                        albedo: value_get_into(mat, "albedo"),
                    },

                    "Metal" => Material::Metal {
                        albedo: value_get_into(mat, "albedo"),
                    },

                    "FuzzedMetal" => Material::FuzzedMetal {
                        albedo: value_get_into(mat, "albedo"),
                        fuzz: value_get_into(mat, "fuzz"),
                    },

                    "Dielectric" => Material::Dielectric {
                        eta: value_get_into(mat, "eta"),
                    },

                    "PolarChecker" => Material::PolarChecker {
                        color1: value_get_into(mat, "color1"),
                        color2: value_get_into_option(mat, "color2")
                            .unwrap_or(color::new(1.0, 1.0, 1.0)),
                        ntheta: value_get_into(mat, "ntheta"),
                        nphi: value_get_into(mat, "nphi"),
                    },

                    "Texture" => {
                        let path: String = value_get_into(mat, "map_path");
                        let resolution: u32 =
                            value_get_into_option(mat, "resolution").unwrap_or(1024);
                        let img = image::open(&path)
                            .expect("Image not found!")
                            .resize(resolution, resolution, FilterType::Gaussian)
                            .into_rgb32f();

                        Material::Texture {
                            map: Arc::new(TextureMap {
                                resolution,
                                path,
                                map: img,
                            }),
                        }
                    }

                    "DiffuseLight" => Material::DiffuseLight {
                        color: value_get_into(mat, "color"),
                    },

                    _ => panic!("Unsupported material type: {}", mat_type),
                };

                (name.to_string(), material)
            })
            .collect();

        Self { map }
    }
}
