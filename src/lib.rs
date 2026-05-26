//! Native extension for tia-py (Py03)

use pyo3::prelude::*;

/// Python module `tia_py._native` (must match `module-name` in pyproject.toml)
#[pymodule]
fn _native(_m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}