use thiserror::Error;

#[derive(Error, Debug)]
pub enum MaterialError {
    #[error("Cannot convert with no argments: {}", mat_type)]
    CannotConvert { mat_type: String },
}
