//! `Analyzer` - entry point for static test impact analysis (placeholder)

use pyo3::prelude::*;

/// Entry point for static test impact analysis

#[pyclass]
pub struct Analyzer {}

#[pymethods]
impl Analyzer {
    #[new]
    fn new() -> Self {
        Self {}
    }
    
    fn __repr__(&self) -> &'static str {
    "Analyzer()"
    }
} 
    