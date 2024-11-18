use std::{fs, sync::Arc};
use toml::Value;

use crate::entity::{analytic::sphere::Sphere, scene::Scene, traits::Entity};

use super::{errors::SerdeError, materials::MaterialMap, toml_common::value_get_into};

impl Scene {
    pub fn configured(path: &str) -> anyhow::Result<Self> {
        let val: Value = toml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        if val.get("materials_path").is_none() || !val.get("materials_path").unwrap().is_str() {
            return Err(SerdeError::RequireFieldType {
                field: "materials_path".into(),
                ty: "string".into(),
            }
            .into());
        }
        let res: Scene = val.into();
        Ok(res)
    }
}

impl From<Value> for Scene {
    fn from(value: Value) -> Self {
        let material_map_path = value
            .get("materials_path")
            .expect("Expected field [materials_path]")
            .as_str()
            .unwrap();

        let material_map = MaterialMap::configured(material_map_path).unwrap();

        let ents = value
            .get("entities")
            .expect("Expected list [[entities]]")
            .as_array()
            .unwrap();

        let entities = ents
            .into_iter()
            .map(|ent| {
                let ent_type = ent
                    .get("type")
                    .expect("Expect a type")
                    .as_str()
                    .expect("Expect entity type to be string");

                let mat_name = ent
                    .get("material")
                    .expect("Expect a material name")
                    .as_str()
                    .expect("Expect material name to be string");

                let mat = material_map
                    .map
                    .get(mat_name)
                    .expect(&format!("Material not found: {}", mat_name));

                let entity: Arc<dyn Entity> = match ent_type {
                    "Sphere" => Arc::new(Sphere::new(
                        value_get_into(ent, "center"),
                        value_get_into(ent, "radius"),
                        *mat,
                    )),

                    _ => panic!("Unsupported entity type"),
                };

                entity
            })
            .collect();

        Self { entities }
    }
}
