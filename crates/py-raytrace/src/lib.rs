use pyo3::prelude::*;
pub mod tracer;

/// A Python module implemented in Rust.
#[pymodule]
pub mod py_raytrace {
    #[pymodule_export]
    pub use super::tracer::tracer; // write your module impl in another file // write your module impl in another file
}
