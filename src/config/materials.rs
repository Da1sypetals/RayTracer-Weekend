use std::{collections::BTreeMap, fs};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use toml::Value;

use crate::materials::material::Material;

use super::toml_common::value_get_into;

#[derive(Debug)]
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

                    _ => panic!("Unsupported material type"),
                };

                (name.to_string(), material)
            })
            .collect();

        Self { map }
    }
}
