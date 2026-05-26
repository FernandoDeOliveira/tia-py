//! Typed output of Python source parsing.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;


/// A single `import` / `from ... import` statement (surface syntax only).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonImport {
    /// Module in `import os` or `from pkg.sub import x` → `pkg.sub`
    pub module: String,
    /// Symbol in `from m import name` (None for `import m`)
    pub name: Option<String>,
    /// Alias in `import x as y` / `from m import x as y`
    pub alias: Option<String>,
    /// 1-based start line
    pub line: u32,
}


/// A function defined at module level (nested functions come in a later extension).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonFunction {
    pub name: String,
    pub line: u32,
}


/// A method inside a class body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonMethod {
    pub name: String,
    pub line: u32,
}


/// A class defined at module level.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonClass {
    pub name: String,
    pub line: u32,
    pub methods: Vec<PythonMethod>,
}


/// Result of parsing one Python file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParsedModule {
    pub path: PathBuf,
    pub imports: Vec<PythonImport>,
    pub functions: Vec<PythonFunction>,
    pub classes: Vec<PythonClass>,
}
impl ParsedModule {
    pub fn empty(path: PathBuf) -> Self {
        Self {
            path,
            imports: Vec::new(),
            functions: Vec::new(),
            classes: Vec::new(),
        }
    }
}