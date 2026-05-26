//! Native extension for tia-py (Py03)

mod bindings;

use pyo3::prelude::*;

/// Python module `tia_py._native` (must match `module-name` in pyproject.toml)
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    bindings::register(m)?;
    Ok(())
}