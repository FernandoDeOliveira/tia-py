//! Python AST parsing (tree-sitter)

pub mod nodes;

mod python;

use anyhow::Context;
pub use nodes::ParsedModule;
use std::path::Path;

/// Read and parse a `.py` file into a [`ParsedModule`]
pub fn parse_file(path: impl AsRef<Path>) -> anyhow::Result<ParsedModule> {
    let path = path.as_ref();
    let source = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    python::parse_source(path, &source)
}