//! Native extension for tia-py (PyO3).

mod bindings;

use pyo3::prelude::*;

/// Python module `tia_py._native` (must match `module-name` in pyproject.toml)
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    bindings::register(m)?;
    Ok(())
}