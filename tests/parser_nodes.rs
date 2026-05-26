use std::path::PathBuf;

use tia_py::parser::nodes::{ParsedModule, PythonImport};
use tia_py::parser::parse_file;

#[test]
fn parsed_module_serializes_to_json() {
    let module = ParsedModule {
        path: PathBuf::from("sample.py"),
        imports: vec![PythonImport {
            module: "os".into(),
            name: None,
            alias: None,
            line: 1
        }],
        functions: vec![],
        classes: vec![],
    };

    let json = serde_json::to_string(&module).unwrap();
    assert!(json.contains("\"module\":\"os\""));
}

#[test]
fn parse_file_reads_disk_and_returns_empty_module_for_now() {
    let path = PathBuf::from("tests/fixtures/empty.py");
    std::fs::create_dir_all("tests/fixtures").ok();
    std::fs::write(&path, "# empty\n").unwrap();
    let parsed = parse_file(&path).unwrap();
    assert_eq!(parsed.path, path);
    assert!(parsed.imports.is_empty());
}