use crate::helpers::types::vec3;

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub pos: vec3,
    pub normal: vec3,
    pub t: f64,
}
