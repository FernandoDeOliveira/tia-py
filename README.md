# tia-py

Static **Test Impact Analysis (TIA)** for Python: discover which tests are likely affected by code changes **without running coverage**, using fast static analysis.

> **Status:** Phase 1 (Bootstrap) complete — Rust + PyO3 + maturin wired; first native function exported. No static analysis, parsing, or graph yet.

## Roadmap

| Phase | Goal |
|-------|------|
| **1 — Bootstrap** | Project setup, maturin, PyO3, module layout, `hello()` from Rust ✓ |
| **2 — Parsing** | tree-sitter-python, extract imports/defs (planned) |
| **3 — Graph** | Dependency graph with petgraph (planned) |
| **4+** | Impact analysis, incremental cache, CI (planned) |

Phase 1 intentionally does **not** include AST parsing, dependency graphs, or TIA logic.

## Phase 1 API (bootstrap)

Equivalent to the bootstrap spec (`from pytia import hello`), using this repo’s import name `tia_py`:

```python
from tia_py import hello

print(hello())  # "hello from rust"
```

Also available (added while scaffolding the real TIA API):

```python
import tia_py

tia_py.version()      # "0.1.0"
tia_py.Analyzer()     # placeholder type for later phases
```

## Requirements

- Python ≥ 3.9
- Rust ≥ 1.75 ([rustup](https://rustup.rs/))
- [maturin](https://www.maturin.rs/)

## Build

Compile the native extension in debug mode and install it into the active virtualenv:

```bash
maturin develop
```

Release build (optimized):

```bash
maturin develop --release
```

Run Rust unit tests:

```bash
cargo test
```

Build a wheel (distribution artifact):

```bash
maturin build
```

## Install locally

```bash
python3 -m venv .venv
source .venv/bin/activate   # Windows: .venv\Scripts\activate

pip install -U pip maturin pytest
pip install -e ".[dev]"

maturin develop
```

## Run / verify

```bash
python -m pytest
```

```bash
python -c "from tia_py import hello; print(hello())"
```

Expected:

```text
hello from rust
```

Use **`python -m pytest`** (not bare `pytest`) so the same interpreter as your venv is used. A global `pytest` shim (e.g. pyenv) may not see `tia_py`.

## Architecture

```text
Python API (tia_py)
    ↓
PyO3 bindings (tia_py._native)   ← Phase 1 stops here
    ↓
Rust analysis engine             (future)
    ↓
AST parsing + dependency graph   (future)
    ↓
Impact analysis engine           (future)
```

## Repository layout

| Path | Role |
|------|------|
| `python/tia_py/` | Public Python package |
| `src/lib.rs` | PyO3 module entry (`tia_py._native`) |
| `src/bindings/` | Functions/types exposed to Python (`hello`, `version`, `Analyzer`) |
| `Cargo.toml` | Rust crate (single crate; no workspace yet) |
| `pyproject.toml` | Python packaging + maturin config |
| `tests/` | pytest integration tests |
| `examples/` | Reserved |
| `benches/` | Reserved |

## Naming

| Context | Name |
|---------|------|
| Repository / distribution | `tia-py` |
| `import` | `tia_py` |
| Native module | `tia_py._native` |
| Rust package | `tia-py` |
| Rust library | `tia_py` |

The bootstrap spec used the temporary name `pytia`; this project uses **`tia-py` / `tia_py`** to match the repository directory.

## Tech stack

| Tool | Phase 1 | Later |
|------|---------|-------|
| Rust, PyO3, maturin | ✓ | |
| tree-sitter-python | | ✓ |
| petgraph, rayon, serde, anyhow, tracing | | ✓ |

## License

MIT
