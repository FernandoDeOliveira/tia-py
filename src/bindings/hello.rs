//! `hello` — first function exported to Python (bootstrap smoke test).

use pyo3::prelude::*;

#[pyfunction]
pub fn hello() -> &'static str {
    "hello from rust"
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello_message() {
        assert_eq!(super::hello(), "hello from rust");
    }
}
