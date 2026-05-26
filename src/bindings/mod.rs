//! Py03 bindings: Rust types/functions exposed to Python.

mod analyzer;

use pyo3::prelude::*;

/// Register all symbols on `tia_py._native`
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_class::<analyzer::Analyzer>()?;
    Ok(())
}

#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_matches_cargo_package() {
        assert_eq!(super::version(), env!("CARGO_PKG_VERSION"));
    }
}