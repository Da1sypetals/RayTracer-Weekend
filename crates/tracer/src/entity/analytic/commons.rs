use crate::helpers::types::{vec2, vec3};
use serde::{Deserialize, Serialize};

fn default_uv() -> vec2 {
    vec2::new(0.0, 1.0)
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub world: vec3,
    #[serde(default = "default_uv")]
    pub uv: vec2,
}

impl Point {
    pub fn world(p: vec3) -> Self {
        Self {
            world: p,
            uv: vec2::zeros(),
        }
    }
}
