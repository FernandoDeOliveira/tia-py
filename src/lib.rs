//! tia-py library root: parser (always) + PyO3 extension (feature `extension-module`).

pub mod parser;

#[cfg(feature = "extension-module")]
mod bindings;

#[cfg(feature = "extension-module")]
use pyo3::prelude::*;

/// Python module `tia_py._native` (must match `module-name` in pyproject.toml).
#[cfg(feature = "extension-module")]
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    bindings::register(m)?;
    Ok(())
}