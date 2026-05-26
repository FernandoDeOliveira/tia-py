//! tree-sitter integration

use crate::parser::nodes::ParsedModule;

/// Parse Python source text into a [`ParsedModule`]
pub fn parse_source(path: &std::path::Path, _source: &str) -> anyhow::Result<ParsedModule> {
    Ok(ParsedModule::empty(path.to_path_buf()))
}