use super::{
    errors::SerdeError,
    materials::MaterialMap,
    toml_common::{value_get_into, value_get_into_option},
};
use crate::{
    entity::{
        analytic::{
            box_::Box, parallelogram::Parallelogram, plane::Plane, smoke_sphere::SmokeSphere,
            somke_box::SmokeBox, sphere::Sphere, triangle::Triangle,
        },
        animated::{plane::AnimatedPlane, sphere::AnimatedSphere},
        animated_scene::AnimatedScene,
        scene::Scene,
        traits::{AnimatedEntity, Entity},
    },
    helpers::types::vec3,
};
use std::{fs, sync::Arc};
use toml::Value;

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
        Ok(val.into())
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

        let background = value_get_into(&value, "background");

        let entities = ents
            .iter()
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

                #[rustfmt::skip]
            let entity: Arc<dyn Entity> = match ent_type {
                "Sphere" => Arc::new(Sphere::new(
                    value_get_into(ent, "center"),
                    value_get_into(ent, "radius"),
                    mat.clone(),
                )),
                "Plane" => Arc::new(Plane::new(
                    value_get_into(ent, "point"),
                    value_get_into(ent, "normal"),
                    mat.clone(),
                )),
                "Triangle" => Arc::new(Triangle::new(
                    value_get_into(ent, "a"),
                    value_get_into(ent, "b"),
                    value_get_into(ent, "c"),
                    mat.clone(),
                )),
                "Parallelogram" => Arc::new(Parallelogram::new(
                    value_get_into(ent, "a"),
                    value_get_into(ent, "b"),
                    value_get_into(ent, "c"),
                    mat.clone(),
                )),
                "Box" => {
                    Arc::new(Box::new(value_get_into(ent, "a"),
                    value_get_into(ent, "b"),
                    value_get_into(ent, "c"),
                    value_get_into(ent, "d"), mat.clone()))
                }
                "SmokeSphere" => Arc::new(SmokeSphere::new(
                    value_get_into(ent, "center"),
                    value_get_into(ent, "radius"),
                    mat.clone(),
                )),
                "SmokeBox" => {
                    Arc::new(SmokeBox::new(value_get_into(ent, "a"),
                    value_get_into(ent, "b"),
                    value_get_into(ent, "c"),
                    value_get_into(ent, "d"), mat.clone()))
                }

                _ => panic!("Unsupported entity type"),
            };

                entity
            })
            .collect();

        Self {
            entities,
            background,
        }
    }
}

// ################################################################
// ########################### animated ###########################
// ################################################################

impl AnimatedScene {
    pub fn configured(path: &str, n_step: u32) -> anyhow::Result<Self> {
        let val: Value = toml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        if val.get("materials_path").is_none() || !val.get("materials_path").unwrap().is_str() {
            return Err(SerdeError::RequireFieldType {
                field: "materials_path".into(),
                ty: "string".into(),
            }
            .into());
        }
        Ok((val, n_step).into())
    }
}

impl From<(Value, u32)> for AnimatedScene {
    fn from(val: (Value, u32)) -> Self {
        let (value, n_step) = val;
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

        let background = value_get_into(&value, "background");

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

                let entity: Arc<dyn AnimatedEntity> = match ent_type {
                    "Sphere" => Arc::new(AnimatedSphere::new(
                        Sphere::new(
                            value_get_into(ent, "center"),
                            value_get_into(ent, "radius"),
                            mat.clone(),
                        ),
                        value_get_into_option(ent, "delta").unwrap_or(vec3::zeros()),
                    )),
                    "Plane" => {
                        let normal = value_get_into(ent, "normal");
                        Arc::new(AnimatedPlane::new(
                            Plane::new(value_get_into(ent, "point"), normal, mat.clone()),
                            value_get_into_option(ent, "delta_point").unwrap_or(vec3::zeros()),
                            value_get_into_option(ent, "new_normal").unwrap_or(normal),
                        ))
                    }

                    _ => panic!("Unsupported entity type"),
                };

                entity
            })
            .collect();

        Self::new(entities, background, n_step)
    }
}
