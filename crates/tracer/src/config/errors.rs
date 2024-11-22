use thiserror::Error;

#[derive(Error, Debug)]
pub enum SerdeError {
    #[error("Require (field: {}, type: {})", field, ty)]
    RequireFieldType { field: String, ty: String },
}
