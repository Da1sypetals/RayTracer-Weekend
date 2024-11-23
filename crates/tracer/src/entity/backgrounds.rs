use serde::{Deserialize, Serialize};

use crate::helpers::types::{color, vec3};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Background {
    Blend { from: color, to: color },
    Pure { color: color },
}

impl Background {
    pub fn color(&self, raydir: vec3) -> color {
        match self {
            Background::Blend { from, to: into } => {
                let t = 0.5 * (raydir.y + 1.0);
                (1.0 - t) * from + t * into
            }
            Background::Pure { color } => *color,
        }
    }
}
